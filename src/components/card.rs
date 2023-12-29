use std::rc::Rc;

use implicit_clone::unsync::IString;
pub use yew::prelude::*;

use crate::{get_img_url, ImageVariant};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub image_id: IString,
    pub text: IString,
}

#[function_component]
pub fn Card(props: &Props) -> Html {
    let image_url: Rc<IString> = use_memo(
        |id: &IString| get_img_url(id, ImageVariant::Thumbnail).into(),
        props.image_id.clone(),
    );
    let backdrop_url: Rc<IString> = use_memo(
        |id: &IString| get_img_url(id, ImageVariant::ThumbnailBackdrop).into(),
        props.image_id.clone(),
    );

    html! {
        <div class="card" style={format!("background-image: url(\"{}\");", backdrop_url)}>
            <img src={image_url.as_ref()} />
            <span>{props.text.clone()}</span>
        </div>
    }
}
