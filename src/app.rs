use std::collections::HashMap;

use concat_string::concat_string;
use indexmap::IndexSet;
use js_sys::{Array, Function};
use serde::Deserialize;
use sql_js_httpvfs_rs::*;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[cfg(feature = "console_log")]
#[allow(unused_imports)]
use log::debug;

use crate::components::*;

const DB_CONFIG: &str = r#"
{
    "from": "inline",
    "config": {
        "serverMode": "full",
        "requestChunkSize": 1024,
        "url": "../databases/db.sqlite3"
    }
}
"#;

#[derive(Clone)]
pub enum Msg {
    SearchStart(String),
    Results(SearchResults),
    Toggle,
}

#[derive(Clone, Copy)]
enum SearchMode {
    Ipa,
    Normal,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct QueryResult {
    pub word: String,
    pub phonemes: String,
}

pub type SearchResults = (Vec<String>, HashMap<String, Vec<String>>);

impl SearchMode {
    fn placeholder_text(&self) -> &'static str {
        match self {
            SearchMode::Ipa => "oʊpəl",
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
    mode: SearchMode,
    first_load: bool,
    is_busy: bool,
    displayed_results: SearchResults,
}

// From https://github.com/yewstack/yew/issues/364#issuecomment-737138847
async fn wrap<F: std::future::Future>(f: F, finished_callback: yew::Callback<F::Output>) {
    finished_callback.emit(f.await);
}

async fn query(search_mode: SearchMode, search: String) -> SearchResults {
    let splits = search.split_ascii_whitespace();
    let mut result = HashMap::with_capacity(splits.size_hint().1.unwrap_or(0));
    let mode = match &search_mode {
        SearchMode::Ipa => " phonemes in (",
        SearchMode::Normal => " word in (",
    };

    let search_items = splits
        .map(|s| s.to_ascii_lowercase())
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
                    if let Ok(QueryResult { word, phonemes }) = entry.into_serde() {
                        result.entry(phonemes).or_insert_with(|| vec![]).push(word);
                    }
                }
            }
            SearchMode::Normal => {
                for entry in js_sys::Array::from(&res).iter() {
                    if let Ok(QueryResult { word, phonemes }) = entry.into_serde() {
                        result.entry(word).or_insert_with(|| vec![]).push(phonemes);
                    }
                }
            }
        }
    }
    (search_items.into_iter().collect::<Vec<_>>(), result)
}

fn initialize_worker_if_missing() {
    if !is_worker_initialized() {
        // This is *really* dumb but I don't think JsValue can just parse from
        // a string -> object.
        let v: serde_json::Value = serde_json::from_str(DB_CONFIG).unwrap();
        let x = JsValue::from_serde(&v).unwrap();
        spawn_local(async {
            create_db_worker(vec![x], "./static/code/sqlite.worker.js", "./sql-wasm.wasm").await;
        });
        // TODO: handle failure properly with some message.
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        initialize_worker_if_missing();
        Self {
            mode: SearchMode::Normal,
            first_load: true,
            is_busy: false,
            displayed_results: SearchResults::default(),
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            if let Some(window) = web_sys::window() {
                // A hack to force full height on mobile through JS code.
                let func = Function::new_no_args(
                        "document.documentElement.style.setProperty('--vh', `${window.innerHeight * 0.01}px`);",
                    );
                if func.apply(&JsValue::NULL, &Array::new()).is_ok() {
                    window.set_onresize(Some(&func));
                }
            }
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        self.first_load = false;
        match msg {
            Msg::SearchStart(search) => {
                initialize_worker_if_missing();

                self.is_busy = true;
                spawn_local(wrap(
                    query(self.mode, search),
                    ctx.link().callback(|results| Msg::Results(results)),
                ));
                true
            }
            Msg::Results(results) => {
                self.displayed_results = results;
                self.is_busy = false;
                true
            }
            Msg::Toggle => {
                self.mode = match &self.mode {
                    SearchMode::Ipa => SearchMode::Normal,
                    SearchMode::Normal => SearchMode::Ipa,
                };
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
        let title_classes = {
            classes!(
                "text-6xl",
                "pt-10",
                "pb-6",
                "font-title",
                "dark:text-slate-50",
                "text-slate-900",
            )
        };

        let text_ref = NodeRef::default();
        let link = ctx.link();
        let on_search = link.callback(|s: String| Msg::SearchStart(s));
        let on_toggle = link.callback(|_| Msg::Toggle);
        let placeholder: &'static str = self.mode.placeholder_text();

        // TODO: animation and proper placement when results are shown
        html! {
            <div class={root_classes}>
                <p class={title_classes}>{"opal"}</p>
                <SearchBar {text_ref} {on_search} {placeholder} {on_toggle} toggle_text={self.mode.button_text()} first_load={self.first_load}/>
                if !self.displayed_results.0.is_empty() {
                    <DisplayedResults to_display={self.displayed_results.clone()}/>
                }
            </div>
        }
    }
}
