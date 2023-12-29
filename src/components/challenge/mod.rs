pub use yew::prelude::*;

use crate::{components::challenge::select_image::SelectImage, WordData, WordSet};

use big_card::BigCard;

mod big_card;
mod select_image;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub target: WordSet,
    pub fakes: [WordSet; 3],
    pub difficulty: usize,
    pub on_success: Callback<usize>,
}

#[function_component]
pub fn Challenge(props: &Props) -> Html {
    let Props {
        target,
        fakes,
        difficulty,
        on_success,
    } = props;

    let challenge = match difficulty {
        0 => {
            // Just the image and the word
            html! {
                <BigCard word={target.0} data={target.1} on_touch={on_success.clone()} />
            }
        }
        1 => {
            // Word in middle, images in corners
            html! {
                <SelectImage target={*target} fakes={*fakes} on_success={on_success} />
            }
        }
        2 => {
            // Image in middle, words in corners
            html! {}
        }
        3 => {
            // Image in middle, words in corners
            // This time with challenge words
            html! {}
        }
        4 => {
            // Spell the word with a letter bank
            html! {}
        }
        _ => {
            // Spell the word without a letter bank
            html! {}
        }
    };

    html! {
        <div style="width: 100vw; height: 100vh; display: flex; justify-content: center; align-items: center;">
            {challenge}
        </div>
    }
}
