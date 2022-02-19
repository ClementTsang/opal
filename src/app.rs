use yew::prelude::*;

#[cfg(feature = "console_log")]
#[allow(unused_imports)]
use log::debug;

use crate::components::*;

#[derive(Clone, PartialEq, Eq)]
pub enum Msg {
    Search(String),
    Toggle,
}

#[derive(Clone)]
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

#[derive(Clone)]
pub struct App {
    dark_mode: bool,
    mode: SearchMode,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            dark_mode: web_sys::window()
                .and_then(|window| window.match_media("(prefers-color-scheme: dark)").ok())
                .map(|res| match res {
                    Some(res) => res.matches(),
                    None => false,
                })
                .unwrap_or(false),
            mode: SearchMode::Normal,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Search(text) => {}
            Msg::Toggle => {
                self.mode = match &self.mode {
                    SearchMode::Ipa => SearchMode::Normal,
                    SearchMode::Normal => SearchMode::Ipa,
                };
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let root_classes = {
            let mut c = vec![
                "h-screen",
                "bg-teal-300",
                "flex",
                "flex-col",
                "items-center",
                "justify-center",
                "gap-4",
            ];
            if self.dark_mode {
                c.push("dark");
            }
            classes!(c)
        };
        let title_classes = classes!("text-6xl", "pb-6", "font-body");

        let text_ref = NodeRef::default();
        let link = ctx.link();
        let on_search = link.callback(|s: String| Msg::Search(s));
        let on_toggle = link.callback(|_| Msg::Toggle);
        let placeholder: &'static str = self.mode.placeholder_text();

        html! {
            <div class={root_classes}>
                <p class={title_classes}>{"opal"}</p>
                <SearchBar {text_ref} {on_search} {placeholder} {on_toggle} toggle_text={self.mode.button_text()}/>
            </div>
        }
    }
}
