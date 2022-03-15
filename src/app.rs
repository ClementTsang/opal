use concat_string::concat_string;
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

type PromiseResult = Result<JsValue, JsValue>;

#[derive(Clone)]
pub enum Msg {
    SearchStart(String),
    Searching(String),
    Results(Vec<PromiseResult>),
    Toggle,
}

#[derive(Clone, Copy)]
enum SearchMode {
    Ipa,
    Normal,
}

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
}

// From https://github.com/yewstack/yew/issues/364#issuecomment-737138847
async fn wrap<F: std::future::Future>(f: F, finished_callback: yew::Callback<F::Output>) {
    finished_callback.emit(f.await);
}

async fn query(mode: SearchMode, search: String) -> Vec<PromiseResult> {
    let splits = search.split_ascii_whitespace();
    let mut result = Vec::with_capacity(splits.size_hint().1.unwrap_or(0));
    let mode = match mode {
        SearchMode::Ipa => " phonemes = '",
        SearchMode::Normal => " word = '",
    };
    for s in splits {
        let query = concat_string!("SELECT * FROM english WHERE", mode, s, "';");
        result.push(exec_query(query).await);
    }
    result
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        if !is_worker_initialized() {
            // This is *really* dumb but I don't think JsValue can just parse from
            // a string -> object.
            let v: serde_json::Value = serde_json::from_str(DB_CONFIG).unwrap();
            let x = JsValue::from_serde(&v).unwrap();
            spawn_local(async {
                create_db_worker(vec![x], "./static/code/sqlite.worker.js", "./sql-wasm.wasm")
                    .await;
            });
            // TODO: handle failure properly with some message.
        }
        Self {
            mode: SearchMode::Normal,
            first_load: true,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        self.first_load = false;
        match msg {
            Msg::SearchStart(search) => {
                // Living dangerously with no checks... ikz!
                ctx.link().send_message(Msg::Searching(search));
                false
            }
            Msg::Searching(search) => {
                spawn_local(wrap(
                    query(self.mode, search),
                    ctx.link().callback(|results| Msg::Results(results)),
                ));
                false
            }
            Msg::Results(results) => {
                // Get results
                debug!("Results: {:?}", results);
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
            "min-h-screen",
            "flex",
            "flex-col",
            "items-center",
            "justify-center",
            "gap-4",
            "dark:bg-slate-900",
            "bg-slate-200",
        );
        let title_classes = {
            classes!(
                "text-6xl",
                "pb-6",
                "font-body",
                "dark:text-slate-50",
                "text-slate-900",
            )
        };

        let text_ref = NodeRef::default();
        let link = ctx.link();
        let on_search = link.callback(|s: String| Msg::SearchStart(s));
        let on_toggle = link.callback(|_| Msg::Toggle);
        let placeholder: &'static str = self.mode.placeholder_text();

        html! {
            <div class={root_classes}>
                <p class={title_classes}>{"opal"}</p>
                <SearchBar {text_ref} {on_search} {placeholder} {on_toggle} toggle_text={self.mode.button_text()} first_load={self.first_load}/>
            </div>
        }
    }
}
