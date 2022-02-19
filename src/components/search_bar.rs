use crate::components::*;
use web_sys::{HtmlInputElement, KeyboardEvent};
use yew::{classes, function_component, functional::*, html, Callback, NodeRef, Properties};

#[derive(Clone, PartialEq, Properties)]
pub struct SearchBarProps {
    pub text_ref: NodeRef,
    pub on_search: Callback<()>,
    pub on_toggle: Callback<()>,
    pub placeholder: &'static str,
    pub toggle_text: &'static str,
}

#[function_component(SearchBar)]
pub fn search_bar(props: &SearchBarProps) -> Html {
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

    let search_bar_classes = classes!(
        "bg-white",
        "flex",
        "lg:w-1/3",
        "md:w-1/2",
        "w-2/3",
        "rounded-full",
        "overflow-hidden",
        "min-w-0",
        "pl-4",
    );
    let input_classes = classes!(
        "bg-white",
        "font-mono",
        "md:text-lg",
        "text-base",
        "h-12",
        "focus:outline-none",
        "flex-1",
        "pl-2",
        "min-w-0",
    );
    let button_classes = classes!(
        "flex-none",
        "flex",
        "items-center",
        "justify-center",
        "px-4",
        "bg-white",
        "hover:bg-sky-600",
        "hover:text-white",
    );
    let toggle_classes = classes!(
        "h-10",
        "w-10",
        "rounded-full",
        "hover:bg-slate-100",
        "self-center",
        "font-mono",
        "text-sm",
    );
    let icon_classes = classes!("w-5", "h-5");

    let search_onclick = {
        let on_search = props.on_search.clone();
        move |_| {
            on_search.emit(());
        }
    };
    let toggle_onclick = {
        let on_search = props.on_toggle.clone();
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

    html! {
        <div class={search_bar_classes}>
            <button title="Toggle between searching IPA and text" class={toggle_classes} onclick={toggle_onclick}>{props.toggle_text}</button>
            <input class={input_classes} type="text" id="search" placeholder={props.placeholder} ref={input_ref} {onkeypress} />
            <button class={button_classes} onclick={search_onclick}>
                <span class={icon_classes}>
                    <MagnifyingGlassIcon/>
                </span>
            </button>
        </div>
    }
}
