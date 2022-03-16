use concat_string::concat_string;
use yew::{function_component, html, Html, Properties};

use crate::app::SearchResult;

#[derive(Clone, PartialEq, Properties)]
pub struct DisplayedResultsProps {
    pub to_display: Vec<SearchResult>,
}

#[function_component(DisplayedResults)]
pub fn results(props: &DisplayedResultsProps) -> Html {
    html! {
        <div>
            { props.to_display.iter().map(|res| {
                html!{
                    <div key={concat_string!(res.word, "-", res.phonemes)}>
                        { concat_string!(res.word, "-", res.phonemes) }
                    </div>
                }
            }).collect::<Html>() }
        </div>
    }
}
