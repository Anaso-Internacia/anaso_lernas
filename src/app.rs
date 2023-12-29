use std::{collections::BTreeMap, rc::Rc};

use implicit_clone::unsync::{IArray, IString};
use rand::seq::{IteratorRandom, SliceRandom};
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::{
    components::{
        challenge::Challenge,
        full_square::{FullSquare, TextOrImage},
        vortaro::Vortaro,
    },
    get_img_url, ImageVariant, WordData, WordSet, DATA,
};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[function_component(App)]
pub fn app() -> Html {
    // let words = DATA
    //     .words
    //     .iter()
    //     .map(|(name, data)| (IString::Static(name), IString::Static(data.image[0])))
    //     .collect::<IArray<_>>();
    //
    // html! {
    //     <Vortaro words={words} />
    // }

    let difficulty = use_state(|| 0);

    let mut corner_words = DATA
        .words
        .iter()
        .choose_multiple(&mut rand::thread_rng(), 4);

    corner_words.shuffle(&mut rand::thread_rng());

    let target = corner_words.pop().unwrap();
    let target = WordSet(target.0, target.1);

    let fakes: [_; 3] = corner_words.try_into().unwrap();
    let fakes = [
        WordSet(fakes[0].0, fakes[0].1),
        WordSet(fakes[1].0, fakes[1].1),
        WordSet(fakes[2].0, fakes[2].1),
    ];

    html! {
        <Challenge
            target={target}
            fakes={fakes}
            on_success={let difficulty = difficulty.clone(); Callback::from(move |_| {difficulty.set(*difficulty + 1)})}
            difficulty={*difficulty}
        />
    }
}
