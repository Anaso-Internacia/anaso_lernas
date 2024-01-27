use implicit_clone::unsync::IString;
use rand::{seq::SliceRandom, thread_rng};
use yew::prelude::*;

use crate::{
    components::full_square::{FullSquare, TextOrImage},
    get_img_url,
    hooks::use_shuffled_words::use_shuffled_words,
    ImageVariant, WordSet,
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

    use_effect_with_deps(
        {
            let fails = fails.clone();
            move |_| {
                fails.set([false; 4]);
            }
        },
        (*target, *fakes),
    );

    let (correct, full_sort) = use_shuffled_words(*target, *fakes);

    let mut corners: [TextOrImage; 4] = full_sort
        .iter()
        .map(|w| TextOrImage::Text(w.0.into()))
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    if *use_misses {
        for (i, miss) in target.1.misses.iter().enumerate().take(4) {
            if i != correct {
                corners[i] = TextOrImage::Text(IString::Static(miss));
            }
        }
    }

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
