use implicit_clone::unsync::IString;
use rand::{seq::SliceRandom, thread_rng, Rng};
use yew::prelude::*;

use crate::{
    components::full_square::{FullSquare, TextOrImage},
    hooks::use_shuffled_words::use_shuffled_words,
    WordSet,
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub target: WordSet,
    pub fakes: [WordSet; 3],
    pub use_misses: bool,
    pub on_success: Callback<usize>,
}

#[function_component]
pub fn SelectText(props: &Props) -> Html {
    let Props {
        target,
        fakes,
        use_misses,
        on_success,
    } = props;

    let fails = use_state(|| [false; 4]);

    use_effect_with((*target, *fakes), {
        let fails = fails.clone();
        move |_| {
            fails.set([false; 4]);
        }
    });

    let (correct, full_sort) = use_shuffled_words(*target, *fakes);

    let corners = use_memo((correct, full_sort), move |_| {
        let mut corners: [TextOrImage; 4] = full_sort
            .iter()
            .map(|w| TextOrImage::Text(w.0.into()))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        if !*use_misses {
            for (i, miss) in target.1.misses.iter().enumerate().take(4) {
                if i != correct {
                    corners[i] = TextOrImage::Text(IString::Static(miss));
                }
            }
        } else {
            let correct_word = full_sort[correct];
            let mut misses: Vec<IString> = Vec::new();

            // Remove a letter
            for _ in 0..2 {
                let mut chars = correct_word.0.chars().collect::<Vec<_>>();
                let rnd = thread_rng().gen_range(0..chars.len() - 1);
                chars.remove(rnd);
                misses.push(chars.iter().collect::<String>().into());
            }

            // Add a letter
            for _ in 0..2 {
                let mut chars = correct_word.0.chars().collect::<Vec<_>>();
                let rnd = thread_rng().gen_range(0..chars.len() - 1);
                chars.insert(
                    rnd,
                    *['a', 'e', 'i', 'o', 'u'].choose(&mut thread_rng()).unwrap(),
                );
                misses.push(chars.iter().collect::<String>().into());
            }

            // Change a letter
            for _ in 0..2 {
                let mut chars = correct_word.0.chars().collect::<Vec<_>>();
                let rnd = thread_rng().gen_range(0..chars.len() - 1);
                let new_char = match chars[rnd] {
                    'a' => *['e', 'i', 'o'].choose(&mut thread_rng()).unwrap(),
                    'e' => *['a', 'i', 'o'].choose(&mut thread_rng()).unwrap(),
                    'i' => *['a', 'e', 'u'].choose(&mut thread_rng()).unwrap(),
                    'o' => *['u', 'u', 'a'].choose(&mut thread_rng()).unwrap(),
                    'u' => *['o', 'o', 'e'].choose(&mut thread_rng()).unwrap(),
                    's' => 't',
                    't' => 's',
                    _ => *['a', 'e', 'i', 'o', 'u', 's', 't']
                        .choose(&mut thread_rng())
                        .unwrap(),
                };
                chars[rnd] = new_char;
                misses.push(chars.iter().collect::<String>().into());
            }

            misses.shuffle(&mut thread_rng());

            for i in 0..4 {
                if i != correct {
                    corners[i] = TextOrImage::Text(misses[i].clone());
                }
            }
        }
        corners
    });

    let corners = corners.as_ref().clone();

    let on_select = Callback::from({
        let on_success = on_success.clone();
        let fails = fails.clone();
        move |i: usize| {
            if i == correct {
                let num_fails = fails.into_iter().filter(|x| *x).count();
                on_success.emit(num_fails);
            } else {
                let mut nf = *fails;
                nf[i] = true;
                fails.set(nf);
            }
        }
    });
    html! {
        <FullSquare
            center={TextOrImage::Image(target.1.image[0])}
            corners={corners}
            on_select={on_select}
            fails={*fails}
        />
    }
}
