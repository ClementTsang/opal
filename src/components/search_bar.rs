use crate::components::*;
use web_sys::{HtmlInputElement, KeyboardEvent};
use yew::{classes, function_component, functional::*, html, Callback, NodeRef, Properties};

#[derive(Clone, PartialEq, Properties)]
pub struct SearchBarProps {
    pub text_ref: NodeRef,
    pub on_search: Callback<()>,
}

#[function_component(SearchBar)]
pub fn search_bar(props: &SearchBarProps) -> Html {
    let search_bar_classes = classes!(
        "bg-white",
        "flex",
        "w-1/4",
        "rounded-md",
        "pl-4",
        "overflow-hidden"
    );
    let input_classes = classes!("font-mono", "text-lg", "h-14", "focus:outline-none", "grow");
    let button_classes = classes!(
        "flex",
        "items-center",
        "justify-center",
        "px-4",
        "bg-white",
        "hover:bg-sky-600",
        "hover:text-white"
    );
    let icon_classes = classes!("w-5", "h-5", "hover:white");

    let onclick = {
        let on_search = props.on_search.clone();
        move |_| {
            on_search.emit(());
        }
    };

    let onkeypress = {
        let on_search = props.on_search.clone();
        move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                on_search.emit(());
            }
        }
    };

    let input_ref = props.text_ref.clone();
    {
        let input_ref = input_ref.clone();

        use_effect(move || {
            if let Some(input) = input_ref.cast::<HtmlInputElement>() {
                let _ = input.focus();
            }
            || ()
        });
    }

    html! {
        <div class={search_bar_classes}>
            <input class={input_classes} type="text" id="search" placeholder="opal" ref={input_ref} {onkeypress} />
            <button class={button_classes} {onclick}>
                <span class={icon_classes}>
                    <MagnifyingGlassIcon/>
                </span>
            </button>
        </div>
    }
}
