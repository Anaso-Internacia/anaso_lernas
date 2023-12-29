use implicit_clone::unsync::{IArray, IString};
use yew::prelude::*;

use crate::components::card::Card;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub words: IArray<(IString, IString)>,
}

#[function_component]
pub fn Vortaro(props: &Props) -> Html {
    let cards = props
        .words
        .iter()
        .map(|(name, id)| {
            html! {
                <Card text={name} image_id={id} />
            }
        })
        .collect::<Html>();

    html! {
        <div style="width: 100vw; height: 100vh; display: grid; padding: 10px; grid-template-columns: repeat(auto-fit, minmax(100px, 1fr)); grid-auto-rows: min-content; gap: 5px;">
            {cards}
        </div>
    }
}
