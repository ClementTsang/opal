use crate::components::*;
use web_sys::{HtmlInputElement, KeyboardEvent};
use yew::{classes, function_component, functional::*, html, Callback, NodeRef, Properties};

#[derive(Clone, PartialEq, Properties)]
pub struct SearchBarProps {
    pub text_ref: NodeRef,
    pub on_search: Callback<String>,
    pub on_toggle: Callback<()>,
    pub placeholder: &'static str,
    pub toggle_text: &'static str,
    pub first_load: bool,
}

#[function_component(SearchBar)]
pub fn search_bar(props: &SearchBarProps) -> Html {
    let text_empty = use_state_eq(|| true);

    let input_ref = props.text_ref.clone();
    if props.first_load {
        {
            let input_ref = input_ref.clone();
            use_effect(move || {
                if let Some(input) = input_ref.cast::<HtmlInputElement>() {
                    let _ = input.focus();
                }
                || ()
            });
        }
    }

    let search_bar_classes = classes!(
        "dark:bg-slate-700",
        "bg-white",
        "flex",
        "md:w-3/4",
        "w-11/12",
        "rounded-md",
        "overflow-hidden",
        "min-w-0",
        "max-w-[840px]",
        "pl-2",
    );
    let input_classes = classes!(
        "dark:bg-slate-700",
        "bg-white",
        "placeholder:text-gray-400",
        "placeholder:dark:text-gray-500",
        "dark:text-slate-50",
        "font-body",
        "md:text-lg",
        "text-base",
        "h-12",
        "focus:outline-none",
        "flex-1",
        "pl-1",
        "min-w-0",
    );
    let x_button_classes = classes!(
        "dark:text-gray-400",
        "flex-none",
        "flex",
        "items-center",
        "justify-center",
        "px-2"
    );
    let button_classes = classes!(
        "dark:bg-slate-700",
        "bg-white",
        "dark:text-gray-400",
        "flex-none",
        "flex",
        "items-center",
        "justify-center",
        "px-4",
        "hover:bg-blue-500",
        "hover:text-slate-50",
        "hover:dark:bg-blue-500",
        "hover:dark:text-slate-50",
    );
    let toggle_classes = classes!(
        "h-10",
        "w-10",
        "rounded-md",
        "hover:bg-slate-100",
        "hover:dark:bg-slate-600",
        "dark:text-slate-50",
        "self-center",
        "font-input",
        "text-sm",
        "text-center",
    );
    let x_mark_classes = classes!("w-4", "h-4");
    let icon_classes = classes!("w-5", "h-5");

    let clear_text = {
        let text_empty = text_empty.clone();
        let input_ref = input_ref.clone();
        move |_e| {
            if let Some(element) = input_ref.cast::<HtmlInputElement>() {
                element.set_value("");
                text_empty.set(true);
            }
        }
    };
    let search_onclick = {
        let input_ref = input_ref.clone();
        let on_search = props.on_search.clone();
        move |_e| {
            let s = input_ref
                .cast::<HtmlInputElement>()
                .map(|input| input.value())
                .unwrap_or_default();
            if !s.is_empty() {
                on_search.emit(s);
            }
        }
    };
    let toggle_onclick = {
        let on_toggle = props.on_toggle.clone();
        move |_e| {
            on_toggle.emit(());
        }
    };
    let onkeypress = {
        let input_ref = input_ref.clone();
        let on_search = props.on_search.clone();
        move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                let s = input_ref
                    .cast::<HtmlInputElement>()
                    .map(|input| input.value())
                    .unwrap_or_default();
                if !s.is_empty() {
                    on_search.emit(s);
                }
            }
        }
    };
    let oninput = {
        let text_empty = text_empty.clone();
        let input_ref = input_ref.clone();

        move |_e| {
            let s = input_ref
                .cast::<HtmlInputElement>()
                .map(|input| input.value())
                .unwrap_or_default();

            text_empty.set(s.is_empty());
        }
    };

    html! {
        <div class={search_bar_classes}>
            <button title="Toggle between searching IPA and text" class={toggle_classes} onclick={toggle_onclick}>{props.toggle_text}</button>
            <input title="Search query" class={input_classes} type="text" id="search" placeholder={props.placeholder} ref={input_ref} {oninput} {onkeypress} />
            if !(*text_empty) {
                <button title="Clear" class={x_button_classes} onclick={clear_text}>
                    <span class={x_mark_classes}>
                        <XMarkIcon/>
                    </span>
                </button>
            }
            <button title="Search" class={button_classes} onclick={search_onclick}>
                <span class={icon_classes}>
                    <MagnifyingGlassIcon/>
                </span>
            </button>
        </div>
    }
}
