use std::collections::HashMap;

use concat_string::concat_string;
use indexmap::IndexSet;
use js_sys::Function;
use serde::{Deserialize, Serialize};
use sql_js_httpvfs_rs::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::MediaQueryList;
use yew::prelude::*;

#[cfg(feature = "console_log")]
#[allow(unused_imports)]
use log::debug;

use crate::components::*;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Configuration {
    server_mode: &'static str,
    request_chunk_size: u64,
    url: &'static str,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct DatabaseConfig {
    from: &'static str,
    config: Configuration,
}

const DB_CONFIG: DatabaseConfig = DatabaseConfig {
    from: "inline",
    config: Configuration {
        server_mode: "full",
        request_chunk_size: 1024,
        url: "../databases/db.sqlite3",
    },
};

const OPAL_THEME_KEY: &str = "opal_theme";
const DARK_THEME: &str = "dark";
const LIGHT_THEME: &str = "light";

#[derive(Clone)]
pub enum Msg {
    SearchStart(String),
    Results(SearchResults),
    ToggleSearchMode,
    ToggleThemeMode(ThemeMode),
    CycleThemeMode,
}

#[derive(Clone, Copy, Debug)]
pub enum ThemeMode {
    Dark,
    Light,
    System,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SearchMode {
    Ipa,
    Normal,
}

impl Default for SearchMode {
    fn default() -> Self {
        SearchMode::Normal
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct QueryResult {
    pub word: String,
    pub phonemes: String,
}

pub type SearchResults = (Vec<String>, HashMap<String, Vec<String>>, SearchMode);

impl SearchMode {
    fn placeholder_text(&self) -> &'static str {
        match self {
            SearchMode::Ipa => "oʊpʌl",
            SearchMode::Normal => "opal",
        }
    }

    fn button_text(&self) -> &'static str {
        match self {
            SearchMode::Ipa => "/ə/",
            SearchMode::Normal => "abc",
        }
    }
}

pub struct App {
    search_mode: SearchMode,
    first_load: bool,
    is_busy: bool,
    displayed_results: SearchResults,
    current_theme_mode: ThemeMode,
    mql: Option<MediaQueryList>,
}

// From https://github.com/yewstack/yew/issues/364#issuecomment-737138847
async fn wrap<F: std::future::Future>(f: F, finished_callback: yew::Callback<F::Output>) {
    finished_callback.emit(f.await);
}

fn process_query_str(query: &str) -> Option<String> {
    let new_string = query
        .trim_end_matches(|c: char| c.is_ascii_punctuation())
        .trim_start_matches(|c: char| c != '\'' && c.is_ascii_punctuation())
        .to_ascii_lowercase();

    if new_string.is_empty() {
        None
    } else {
        Some(new_string)
    }
}

async fn query(search_mode: SearchMode, search: String) -> SearchResults {
    let splits = search.split_ascii_whitespace();
    let mut result = HashMap::with_capacity(splits.size_hint().1.unwrap_or(0));
    let mode = match &search_mode {
        SearchMode::Ipa => " phonemes in (",
        SearchMode::Normal => " word in (",
    };

    let search_items = splits
        .filter_map(|s| process_query_str(s))
        .collect::<IndexSet<_>>();
    let list = search_items
        .iter()
        .map(|s| concat_string!("'", s.replace("'", "''"), "'"))
        .collect::<Vec<_>>()
        .join(",");
    let query = concat_string!("SELECT * FROM english WHERE", mode, list, ");");
    if let Ok(res) = exec_query(query).await {
        match &search_mode {
            SearchMode::Ipa => {
                for entry in js_sys::Array::from(&res).iter() {
                    if let Ok(QueryResult { word, phonemes }) =
                        serde_wasm_bindgen::from_value(entry)
                    {
                        result.entry(phonemes).or_insert_with(|| vec![]).push(word);
                    }
                }
            }
            SearchMode::Normal => {
                for entry in js_sys::Array::from(&res).iter() {
                    if let Ok(QueryResult { word, phonemes }) =
                        serde_wasm_bindgen::from_value(entry)
                    {
                        result.entry(word).or_insert_with(|| vec![]).push(phonemes);
                    }
                }
            }
        }
    }
    (
        search_items.into_iter().collect::<Vec<_>>(),
        result,
        search_mode,
    )
}

fn initialize_worker_if_missing() {
    if !is_worker_initialized() {
        // This is *really* dumb but I don't think JsValue can just parse from
        // a string -> object.
        // let v: serde_json::Value = serde_json::from_str(DB_CONFIG).unwrap();
        // let x = wasm_bindgen::JsValue::from_serde(&v).unwrap();
        let x = serde_wasm_bindgen::to_value(&DB_CONFIG).unwrap();
        spawn_local(async {
            create_db_worker(vec![x], "./static/code/sqlite.worker.js", "./sql-wasm.wasm").await;
        });
        // TODO: handle failure properly with some message.
    }
}

fn theme_mode() -> ThemeMode {
    if let Some(window) = web_sys::window() {
        if let Ok(Some(local_storage)) = window.local_storage() {
            if let Ok(Some(res)) = local_storage.get_item(OPAL_THEME_KEY) {
                if res == DARK_THEME {
                    return ThemeMode::Dark;
                } else if res == LIGHT_THEME {
                    return ThemeMode::Light;
                }
            }
        }
    }

    ThemeMode::System
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        initialize_worker_if_missing();
        Self {
            search_mode: SearchMode::Normal,
            first_load: true,
            is_busy: false,
            displayed_results: SearchResults::default(),
            current_theme_mode: theme_mode(),
            mql: None,
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let callback = ctx.link().callback(|mode| Msg::ToggleThemeMode(mode));
            callback.emit(self.current_theme_mode);
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        self.first_load = false;
        match msg {
            Msg::SearchStart(search) => {
                initialize_worker_if_missing();

                self.is_busy = true;
                spawn_local(wrap(
                    query(self.search_mode, search),
                    ctx.link().callback(|results| Msg::Results(results)),
                ));
                true
            }
            Msg::Results(results) => {
                self.displayed_results = results;
                self.is_busy = false;
                true
            }
            Msg::ToggleSearchMode => {
                self.search_mode = match &self.search_mode {
                    SearchMode::Ipa => SearchMode::Normal,
                    SearchMode::Normal => SearchMode::Ipa,
                };
                true
            }
            Msg::ToggleThemeMode(mode) => {
                fn toggle_dark(enable: bool) {
                    if let Some(window) = web_sys::window() {
                        if let Some(document) = window.document() {
                            if let Some(document_element) = document.document_element() {
                                if enable {
                                    let _ = document_element.class_list().add_1("dark");
                                } else {
                                    let _ = document_element.class_list().remove_1("dark");
                                }
                            }
                        }
                    }
                }

                match mode {
                    ThemeMode::Dark => {
                        if let Some(window) = web_sys::window() {
                            if let Some(mql) = &mut self.mql {
                                mql.set_onchange(None);
                            }

                            if let Ok(Some(local_storage)) = window.local_storage() {
                                let _ = local_storage.set_item(OPAL_THEME_KEY, DARK_THEME);
                            }
                        }
                        toggle_dark(true);

                        true
                    }
                    ThemeMode::Light => {
                        if let Some(window) = web_sys::window() {
                            if let Some(mql) = &mut self.mql {
                                mql.set_onchange(None);
                            }

                            if let Ok(Some(local_storage)) = window.local_storage() {
                                let _ = local_storage.set_item(OPAL_THEME_KEY, LIGHT_THEME);
                            }
                        }
                        toggle_dark(false);

                        true
                    }
                    ThemeMode::System => {
                        if let Some(window) = web_sys::window() {
                            if let Ok(Some(mql)) =
                                window.match_media("(prefers-color-scheme: dark)")
                            {
                                toggle_dark(mql.matches());

                                // TODO: Use a closure to properly hook into Yew state.
                                // Maybe see https://github.com/rustwasm/wasm-bindgen/issues/843 and
                                // https://stackoverflow.com/a/19014495
                                mql.set_onchange(Some(&Function::new_with_args(
                                    "e",
                                    "
                                if (e.matches) {
                                    document.documentElement.classList.add('dark')
                                } else {
                                    document.documentElement.classList.remove('dark')
                                }
                                ",
                                )));

                                self.mql = Some(mql);
                            }

                            if let Ok(Some(local_storage)) = window.local_storage() {
                                let _ = local_storage.remove_item(OPAL_THEME_KEY);
                            }

                            true
                        } else {
                            false
                        }
                    }
                }
            }
            Msg::CycleThemeMode => {
                self.current_theme_mode = match self.current_theme_mode {
                    ThemeMode::Dark => ThemeMode::Light,
                    ThemeMode::Light => ThemeMode::System,
                    ThemeMode::System => ThemeMode::Dark,
                };

                let callback = ctx.link().callback(|mode| Msg::ToggleThemeMode(mode));
                callback.emit(self.current_theme_mode);

                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let root_classes = classes!(
            "h-screen",
            "flex",
            "flex-col",
            "items-center",
            "justify-start",
            "gap-4",
            "dark:bg-slate-800",
            "bg-slate-100",
            "overflow-y-auto",
        );
        let title_classes = classes!(
            "text-6xl",
            "pb-6",
            "font-title",
            "dark:text-slate-50",
            "text-slate-900",
        );
        let option_div_classes = classes!(
            "pr-[18px]",
            "pt-[18px]",
            "w-full",
            "flex",
            "flex-row-reverse",
            "gap-x-4",
        );
        let option_button_classes = classes!(
            "flex",
            "items-center",
            "justify-center",
            "p-1.5",
            "hover:bg-slate-300",
            "hover:dark:bg-slate-600",
            "rounded-md"
        );
        let mode_button_div_classes = classes!("h-5", "w-5", "text-blue-400");

        let text_ref = NodeRef::default();
        let link = ctx.link();
        let on_search = link.callback(|s: String| Msg::SearchStart(s));
        let on_toggle = link.callback(|_| Msg::ToggleSearchMode);
        let placeholder: &'static str = self.search_mode.placeholder_text();
        let open_theme_window = link.callback(|_| Msg::CycleThemeMode);

        let open_modal = Callback::from(|_| {
            if let Some(window) = web_sys::window() {
                if let Some(document) = window.document() {
                    if let Some(modal) = document.get_element_by_id("infoModal") {
                        let _ = modal.class_list().remove_1("hidden");
                    }
                }
            }
        });

        html! {
            <div class={root_classes}>
                <div class={option_div_classes}>
                    <button class={option_button_classes.clone()} onclick={open_modal}>
                        <div class={mode_button_div_classes.clone()}>
                            <InfoIcon/>
                        </div>
                    </button>
                    <button title="Change theme" class={option_button_classes} onclick={open_theme_window}>
                        <div class={mode_button_div_classes}>
                        {
                            match self.current_theme_mode {
                                ThemeMode::Dark => html!{<MoonIcon />},
                                ThemeMode::Light => html!{<SunIcon />},
                                ThemeMode::System => html!{<ComputerIcon />},
                            }
                        }
                        </div>
                    </button>
                </div>
                <InfoModal />
                <p class={title_classes}>{"opal"}</p>
                <SearchBar {text_ref} {on_search} {placeholder} {on_toggle} toggle_text={self.search_mode.button_text()} first_load={self.first_load} is_busy={self.is_busy}/>
                if self.is_busy {
                    <SpinnerIcon />
                }
                else if !self.displayed_results.0.is_empty() {
                    <DisplayedResults to_display={self.displayed_results.clone()}/>
                }
            </div>
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_string_stripping() {
        assert_eq!(process_query_str("test"), Some("test".to_string()));
        assert_eq!(process_query_str("'twas"), Some("'twas".to_string()));
        assert_eq!(process_query_str("!twas"), Some("twas".to_string()));
        assert_eq!(process_query_str("-"), None);
        assert_eq!(process_query_str("!'twas!"), Some("'twas".to_string()));
        assert_eq!(process_query_str("'twas'"), Some("'twas".to_string()));
    }
}
