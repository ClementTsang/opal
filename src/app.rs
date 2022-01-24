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
pub struct App {
    dark_mode: bool,
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
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Search => true,
            Msg::Toggle => true,
        }
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
            ];
            if self.dark_mode {
                c.push("dark");
            }
            classes!(c)
        };

        let text_ref = NodeRef::default();
        let link = ctx.link();
        let on_search = link.callback(|_| Msg::Search);

        html! {
            <div class={root_classes}>
                <SearchBar {text_ref} {on_search}/>
            </div>
        }
    }
}
