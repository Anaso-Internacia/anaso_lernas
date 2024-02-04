use std::{
    collections::{BTreeMap, HashMap, VecDeque},
    rc::Rc,
};

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
        select_language::SelectLanguage,
        vortaro::Vortaro,
    },
    get_img_url,
    word_queue::{WordQueue, WordQueueContext},
    ImageVariant, WordData, WordSet, DATA,
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

#[derive(Serialize, Deserialize)]
struct PlayerWordStatus {
    pub level: i32,
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
    let native_language = use_state(|| None::<&'static str>);
    let word_queue = use_reducer(WordQueue::new);

    if let Some(native_language) = *native_language {
        html! {
            <ContextProvider<WordQueueContext> context={word_queue}>
                <Challenge />
            </ContextProvider<WordQueueContext>>
        }
    } else {
        html! {
            <SelectLanguage on_selected={Callback::from(move |x| native_language.set(Some(x)))} />
        }
    }
}
