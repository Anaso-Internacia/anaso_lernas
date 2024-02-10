use yew::prelude::*;

use crate::{get_img_url, ImageVariant, WordData};

#[derive(Properties)]
pub struct Props {
    pub word: &'static str,
    pub data: &'static WordData<'static>,
    pub on_touch: Callback<usize>,
}

impl PartialEq for Props {
    fn eq(&self, other: &Self) -> bool {
        self.word == other.word
    }
}

#[function_component]
pub fn BigCard(props: &Props) -> Html {
    let Props {
        word,
        data,
        on_touch,
    } = props;

    let img = get_img_url(data.image[0], ImageVariant::Full);
    let bac = get_img_url(data.image[0], ImageVariant::ThumbnailBackdrop);

    html! {
        <div class="big-card" style={format!("background-image: url({})", bac)} onclick={let on_touch = on_touch.clone(); Callback::from(move |_| on_touch.emit(0))}>
            <img key={img.clone()} style="width: 80vh; word-break: break-all; max-width: 100vw; height: 80vh; max-height: 100vw; cursor: pointer; background-color: #eee;" src={img} />
            <span style="font-size: 4rem; font-weight: 700; text-shadow: 0 0 1.5rem white;">{word}</span>
        </div>
    }
}
