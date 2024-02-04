pub use yew::prelude::*;

use crate::{
    get_img_url,
    word_queue::{WordQueueAction, WordQueueContext},
    ImageVariant, WordSet, DATA,
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
pub fn Challenge(_props: &Props) -> Html {
    let word_queue = use_context::<WordQueueContext>().unwrap();

    let current = word_queue.current();
    let fakes = word_queue.fakes();
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
    // use_interval(
    //     {
    //         let on_success = on_success.clone();
    //         move || on_success.emit(0)
    //     },
    //     10,
    // );

    let target = DATA.words.get(current.text).unwrap();
    let target = &WordSet(current.text, target);

    let fakes = &[
        WordSet(fakes[0], DATA.words.get(fakes[0]).unwrap()),
        WordSet(fakes[1], DATA.words.get(fakes[1]).unwrap()),
        WordSet(fakes[2], DATA.words.get(fakes[2]).unwrap()),
    ];

    let preload_images = {
        let fakes = word_queue
            .next_fakes()
            .into_iter()
            .filter_map(|text| DATA.words.get(text))
            .map(|x| x.image[0]);
        [DATA.words.get(word_queue.next().text).unwrap().image[0]]
            .into_iter()
            .chain(fakes)
            .map(|img_id| get_img_url(img_id, ImageVariant::Full))
            .map(|url| {
                html! {
                    <img style="display: block; visibility: hidden; position: fixed;" src={url} />
                }
            })
            .collect::<Html>()
    };

    let stats_bar = {
        let stat_things = word_queue
            .stats()
            .into_iter()
            .enumerate()
            .rev()
            .map(|(i, w)| {
                let bg = match i {
                    0 => "#fff",
                    1 => "#fda4af",
                    2 => "#f0abfc",
                    3 => "#c4b5fd",
                    4 => "#93c5fd",
                    5 => "#7dd3fc",
                    6 => "#67e8f9",
                    _ => "#4ade80",
                };
                html! {
                    <div style={format!("flex-grow: {}; background-color: {}", w, bg)} />
                }
            })
            .collect::<Html>();
        html! {
            <div style="z-index: 100000; display:flex; height: 10px; align-items: stretch; position: absolute; bottom: 0; left: 0; width: 100%">
                {stat_things}
            </div>
        }
    };

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
            {preload_images}
            {challenge}
            {stats_bar}
        </div>
    }
}
