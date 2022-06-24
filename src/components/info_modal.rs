use crate::components::*;

use web_sys::MouseEvent;
use yew::{classes, function_component, html, Callback};

#[function_component(InfoModal)]
pub fn displayed_results() -> Html {
    let modal_back_classes = classes!(
        "hidden",
        "fixed",
        "top-0",
        "left-0",
        "w-full",
        "h-full",
        "outline-none",
        "overflow-x-hidden",
        "overflow-y-auto",
        "flex",
        "justify-center",
        "z-50",
        "bg-gray-700",
        "bg-opacity-50",
    );
    let modal_classes = classes!(
        "relative",
        "rounded-md",
        "overflow-hidden",
        "bg-white",
        "dark:bg-slate-700",
        "text-left",
        "w-full",
        "max-w-2xl",
        "h-full",
        "md:h-auto",
        "m-auto",
        "p-8",
        "flex",
        "flex-col",
        "gap-5",
        "drop-shadow-light",
    );
    let modal_header = classes!(
        "text-lg",
        "font-body",
        "font-bold",
        "dark:text-slate-50",
        "subpixel-antialiased",
        "pb-0.5",
    );
    let modal_text = classes!(
        "md:leading-snug",
        "leading-snug",
        "font-body",
        "text-base",
        "dark:text-slate-50",
        "subpixel-antialiased",
    );
    let link_hover = classes!(
        "text-blue-500",
        "dark:text-blue-400",
        "hover:text-blue-700",
        "hover:dark:text-blue-300"
    );

    let close_modal = Callback::from(|_| {
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                if let Some(modal) = document.get_element_by_id("infoModal") {
                    let _ = modal.class_list().add_1("hidden");
                }
            }
        }
    });

    let about_text = "opal is a simple static webapp to look up the IPA phonetics of English words, or \
                            vice versa. More language support or sources may be added in the future.";

    html! {
        <div id="infoModal" tabindex="-1" aria-hidden="true" role="dialog" aria-modal="true" class={modal_back_classes} onclick={close_modal.clone()}>
            <div class={modal_classes} onclick={Callback::from(|e: MouseEvent| e.stop_propagation())}>
                <div class={classes!("absolute", "top-0", "right-0", "mr-[8px]", "mt-[8px]",)}>
                    <button class={classes!( "flex", "items-center", "justify-center", "rounded-md")} onclick={close_modal}>
                        <div class={classes!("w-4", "h-4", "text-slate-400", "hover:text-slate-500")}>
                            <XMarkIcon />
                        </div>
                    </button>
                </div>
                <div>
                    <h1 class={modal_header.clone()}>
                        {"About"}
                    </h1>
                    <div class={modal_text.clone()}>
                        <p>
                            {about_text}
                        </p>
                        <br/>
                        <p>
                            {"All source code can be found "}
                            <a href="https://github.com/ClementTsang/opal" target="_blank" class={link_hover.clone()}>{"on GitHub"}</a>
                            {"."}
                        </p>
                    </div>
                </div>
                <div>
                    <h1 class={modal_header.clone()}>
                        {"Credits"}
                    </h1>
                    <p class={modal_text.clone()}>
                        {"opal would not be possible without:"}
                    </p>
                    <div class={modal_text}>
                        <ul class={classes!("list-disc", "pl-5", "pt-1")}>
                            <li>
                                { "English (US) IPA mappings based on " }
                                <a href="https://github.com/cmusphinx/cmudict" target="_blank" class={link_hover.clone()}>{"CMUDict"}</a>
                                { " (see " }
                                <a href="https://github.com/cmusphinx/cmudict/blob/master/LICENSE" target="_blank" class={link_hover.clone()}>{"original license"}</a>
                                {")" }
                            </li>
                            <li>
                                <a href="https://phiresky.github.io/blog/2021/hosting-sqlite-databases-on-github-pages/" target="_blank" class={link_hover.clone()}>{"phiresky"}</a>
                                { " for the idea of hosting SQLite on a static webpage, and writing libraries to do so." }
                            </li>
                            <li>
                                <a href="https://yew.rs/" target="_blank" class={link_hover.clone()}>{"Yew"}</a>
                                { ", the Rust frontend framework used to write this." }
                            </li>
                            <li>
                                <a href="https://tailwindcss.com/" target="_blank" class={link_hover.clone()}>{"Tailwind CSS"}</a>
                                { ", the CSS framework used because I'm bad at CSS." }
                            </li>
                        </ul>
                    </div>
                </div>
            </div>
        </div>
    }
}
