use concat_string::concat_string;
use yew::{classes, function_component, html, Html, Properties};

use crate::app::SearchResults;

#[derive(Clone, PartialEq, Properties)]
pub struct DisplayedResultsProps {
    pub to_display: SearchResults,
}

#[function_component(DisplayedResults)]
pub fn displayed_results(props: &DisplayedResultsProps) -> Html {
    let results_class = classes!(
        "pt-4",
        "pb-6",
        "md:w-3/4",
        "w-11/12",
        "min-w-0",
        "max-w-[840px]",
        "flex",
        "flex-col",
        "gap-4",
    );

    let card_classes = classes!(
        "dark:bg-slate-700",
        "bg-white",
        "rounded-md",
        "w-full",
        "drop-shadow-light",
        "px-4",
        "py-4",
        "dark:text-slate-50",
    );

    let word_class = classes!("font-body", "font-bold", "text-lg", "pb-1");

    let result_class = classes!(
        "font-body",
        "md:leading-none",
        "leading-none",
        "md:text-lg",
        "text-base",
        "dark:text-slate-50",
        "antialiased"
    );

    html! {
        <div class={results_class}>
            { props.to_display.0.iter().map(|search| {
                html!{
                    <div class={card_classes.clone()} key={search.as_str()}>
                        <div class={word_class.clone()}>{search}</div>
                        if let Some(results) = props.to_display.1.get(search) {
                            <div class={classes!("flex", "flex-col", "gap-1")}>
                            {
                                results.iter().map(|result| {
                                    html! {
                                        <div class={result_class.clone()} key={result.as_str()}>
                                            { concat_string!("/", result, "/") }
                                        </div>
                                    }
                                }).collect::<Html>()
                            }
                            </div>
                        } else {
                            <div class={{
                                let mut r = result_class.clone();
                                r.push("text-red-600");
                                r.push("dark:text-red-500");
                                r
                            }}>
                                {"No result"}
                            </div>
                        }
                    </div>
                }
            }).collect::<Html>() }
        </div>
    }
}
