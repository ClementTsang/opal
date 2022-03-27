use concat_string::concat_string;
use yew::{function_component, html, Html, Properties};

use crate::app::SearchResults;

#[derive(Clone, PartialEq, Properties)]
pub struct DisplayedResultsProps {
    pub to_display: SearchResults,
}

#[function_component(DisplayedResults)]
pub fn results(props: &DisplayedResultsProps) -> Html {
    html! {
        <div>
            { props.to_display.0.iter().map(|search| {
                if let Some(results) = props.to_display.1.get(search) {
                    results.iter().map(|result| {
                        html!{
                            <div key={ concat_string!(search, "-", result) }>
                                { concat_string!(search, "-", result) }
                            </div>
                        }
                    }).collect::<Html>()
                } else {
                    html!{
                        <div key={ concat_string!(search, "-") }>
                            { concat_string!(search, "-", "N/A") }
                        </div>
                    }
                }
            }).collect::<Html>() }
        </div>
    }
}
