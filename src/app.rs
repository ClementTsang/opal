use yew::prelude::*;

#[cfg(feature = "console_log")]
#[allow(unused_imports)]
use log::debug;

use crate::components::*;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Msg {
    Search,
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

    fn button_text(&self) -> &str {
        match self {
            SearchMode::Ipa => "Use IPA",
            SearchMode::Normal => "Use Text",
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
            Msg::Search => {}
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

        let type_classes = classes!(
            "px-4",
            "w-48",
            "h-12",
            "font-semibold",
            "bg-white",
            "hover:bg-sky-600",
            "hover:text-white",
            "rounded-md"
        );

        let text_ref = NodeRef::default();
        let link = ctx.link();
        let on_search = link.callback(|_| Msg::Search);
        let on_toggle = link.callback(|_| Msg::Toggle);
        let placeholder: &'static str = self.mode.placeholder_text();

        html! {
            <div class={root_classes}>
                <SearchBar {text_ref} {on_search} {placeholder}/>
                <button onclick={on_toggle} class={type_classes}>{self.mode.button_text()}</button>
            </div>
        }
    }
}
