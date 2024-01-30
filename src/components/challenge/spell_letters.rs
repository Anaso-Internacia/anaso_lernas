use std::thread;

use rand::{seq::SliceRandom, thread_rng};
use yew::prelude::*;

use crate::{get_img_url, ImageVariant, WordData};

#[derive(Properties)]
pub struct Props {
    pub word: &'static str,
    pub data: &'static WordData<'static>,
    pub extra_letters: usize,
    pub on_success: Callback<usize>,
}

impl PartialEq for Props {
    fn eq(&self, other: &Self) -> bool {
        self.word == other.word
    }
}

const EXTRA_LETTERS: [char; 27] = [
    'a', 'b', 'c', 'ĉ', 'd', 'e', 'f', 'g', 'ĝ', 'h', 'i', 'j', 'ĵ', 'k', 'l', 'm', 'n', 'o', 'p',
    'r', 's', 'ŝ', 't', 'u', 'ŭ', 'v', 'z',
];

#[function_component]
pub fn SpellLetters(props: &Props) -> Html {
    let Props {
        word,
        data,
        extra_letters,
        on_success,
    } = props;

    let chars = use_state(Vec::<char>::new);
    let fill_state = use_state(Vec::<char>::new);
    let randomized_letters = use_state(Vec::<char>::new);
    let letters_used = use_state(|| 0u64);
    let fails = use_state(|| 0u64);

    use_effect_with(*word, {
        let chars = chars.clone();
        let fill_state = fill_state.clone();
        let randomized_letters = randomized_letters.clone();
        let letters_used = letters_used.clone();
        let extra_letters = *extra_letters;
        let fails = fails.clone();
        move |w: &&str| {
            let mut chars_ = w.chars().collect::<Vec<_>>();
            chars.set(chars_.clone());
            for _ in 0..extra_letters {
                chars_.push(*EXTRA_LETTERS.choose(&mut thread_rng()).unwrap_or(&'a'))
            }
            // chars_.shuffle(&mut thread_rng());
            chars_.sort();

            letters_used.set(0);
            fails.set(0);
            fill_state.set(Vec::new());
            randomized_letters.set(chars_);
        }
    });

    let img = get_img_url(data.image[0], ImageVariant::Full);
    let bac = get_img_url(data.image[0], ImageVariant::ThumbnailBackdrop);

    let on_letter_select = Callback::from({
        let chars = chars.clone();
        let fill_state = fill_state.clone();
        let letters_used = letters_used.clone();
        let fails = fails.clone();
        let on_success = on_success.clone();
        move |(i, c)| {
            if c == chars[fill_state.len()] {
                let mut new_fill_state = Vec::clone(&*fill_state);
                new_fill_state.push(c);
                if new_fill_state.len() == chars.len() {
                    on_success.emit(fails.count_ones() as usize);
                }
                fill_state.set(new_fill_state);
                let new_letters_used = *letters_used | (1u64 << i);
                letters_used.set(new_letters_used);
            } else {
                let new_fails = *fails | (1 << fill_state.len());
                fails.set(new_fails);
            }
        }
    });

    let letters = fill_state
        .iter()
        .chain(Some('%').iter().cycle())
        .copied()
        .take(chars.len())
        .enumerate()
        .map(|(i, c)| {
            let class = if *fails & (1 << i) != 0 {
                "fail".to_string()
            } else {
                String::new()
            };

            let (c, class) = if c == '%' {
                (' ', class + " empty")
            } else {
                (c, class)
            };

            html! {
                <span class={class}>{c}</span>
            }
        })
        .collect::<Html>();

    let letter_bank = randomized_letters
        .iter()
        .copied()
        .enumerate()
        .map(|(i, c)| {
            if *letters_used & (1 << i) == 0 {
                html! {
                    <span onclick={let on_letter_select = on_letter_select.clone(); move |_| on_letter_select.emit((i, c))}>{c}</span>
                }
            } else {
                html! {
                    <span class="done"></span>
                }
            }
        })
        .collect::<Html>();

    html! {
        <div class="big-card spell-letters" style={format!("background-image: url({})", bac)}>
            <img key={img.as_str()} style="aspect-ratio: 1 / 1; width: 60vh; max-width: 100%; max-height: 60%; background-color: #eee;" src={img} />
            <div class="bank">
                {letters}
            </div>
            <div class="bank selectable">
                {letter_bank}
            </div>
        </div>
    }
}
