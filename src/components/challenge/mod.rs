pub use yew::prelude::*;

use crate::{
    word_queue::{WordQueueAction, WordQueueContext},
    WordData, WordSet, DATA,
};

use big_card::BigCard;
use select_image::SelectImage;
use select_text::SelectText;
use spell_letters::SpellLetters;

mod big_card;
mod select_image;
mod select_text;
mod spell_letters;

#[derive(Properties, PartialEq)]
pub struct Props {}

#[function_component]
pub fn Challenge(props: &Props) -> Html {
    let word_queue = use_context::<WordQueueContext>().unwrap();

    let current = word_queue.current();
    let fakes = word_queue.fakes();

    let target = DATA.words.get(current.text).unwrap();
    let target = &WordSet(current.text, target);

    let fakes = &[
        WordSet(fakes[0], DATA.words.get(fakes[0]).unwrap()),
        WordSet(fakes[1], DATA.words.get(fakes[1]).unwrap()),
        WordSet(fakes[2], DATA.words.get(fakes[2]).unwrap()),
    ];

    let on_success = Callback::from({
        let word_queue = word_queue.clone();
        let nonce = current.text;
        move |a| {
            word_queue.dispatch(WordQueueAction::Submit {
                attempts: a as i32,
                nonce,
            });
        }
    });

    let challenge = match current.level {
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
            // <div>{word_queue.all_as_vec().iter().take(35).map(|x|html!{<div>{x.level.to_string()}{x.text}{"\u{a0}"}</div>}).collect::<Html>()}</div>
            {challenge}
        </div>
    }
}
