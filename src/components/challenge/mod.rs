pub use yew::prelude::*;

use crate::{WordData, WordSet};

use big_card::BigCard;
use select_image::SelectImage;
use select_text::SelectText;
use spell_letters::SpellLetters;

mod big_card;
mod select_image;
mod select_text;
mod spell_letters;

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
            html! {
                <SelectText target={*target} fakes={*fakes} on_success={on_success} use_misses={false} />
            }
        }
        3 => {
            // Image in middle, words in corners
            // This time with challenge words
            html! {
                <SelectText target={*target} fakes={*fakes} on_success={on_success} use_misses={true} />
            }
        }
        4 => {
            // Spell the word with a letter bank
            html! {
                <SpellLetters word={target.0} data={target.1} on_success={on_success.clone()} extra_letters={0} />
            }
        }
        _ => {
            // Spell the word without a letter bank
            html! {
                <SpellLetters word={target.0} data={target.1} on_success={on_success.clone()} extra_letters={1} />
            }
        }
    };

    html! {
        <div style="position: absolute; top: 0; left: 0; width: 100%; height: 100%; display: flex; justify-content: center; align-items: center;">
            {challenge}
        </div>
    }
}
