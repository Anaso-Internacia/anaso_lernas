use yew::prelude::*;

use crate::{get_img_url, ImageVariant};

#[derive(Clone, Debug, PartialEq, Hash)]
pub enum TextOrImage {
    Text(AttrValue),
    Image(&'static str),
}
#[derive(Properties, PartialEq)]
pub struct Props {
    pub center: TextOrImage,
    pub corners: [TextOrImage; 4],
    pub fails: [bool; 4],
    pub on_select: Callback<usize>,
}

#[function_component]
pub fn FullSquare(props: &Props) -> Html {
    let Props {
        center,
        corners,
        fails,
        on_select,
    } = props;

    let corners = corners
        .iter()
        .enumerate()
        .map(|(i, t)| match t {
            TextOrImage::Text(t) => {
                let e = if fails[i] {
                    "color: #f87171; transform: scale(0.9);"
                } else {
                    "color: black;"
                };
                html! {
                    <div
                        style="width: 50vmin; height: 50vmin; display: flex; justify-content: center; align-items: center;"
                        onclick={let on_select = on_select.clone(); move |_| on_select.emit(i)}
                    >
                        <span style={format!("font-size: 6vmin; cursor: pointer; font-weight: 700; {e}")}>{t}</span>
                    </div>
                }
            }
            TextOrImage::Image(t) => {
                let e = if fails[i] {
                    "filter: saturate(0%) opacity(0.5);"
                } else {
                    "filter: saturate(100%) opacity(1.0);"
                };
                html! {
                    <img
                        key={*t}
                        src={get_img_url(t, ImageVariant::FourFour)}
                        style={["width: 50vmin; height: 50vmin; cursor: pointer; transition: filter 0.1s linear;", e].concat()}
                        onclick={let on_select = on_select.clone(); move |_| on_select.emit(i)}
                    />
                }
            }
        })
        .collect::<Html>();

    let (center, bg) = match center {
        TextOrImage::Text(t) => (
            html! {
                <span style="color: black; font-size: 7vmin; font-weight: 700;" class="border-text">{t}</span>
            },
            String::new(),
        ),
        TextOrImage::Image(t) => (
            html! {
                <img
                    key={*t}
                    src={get_img_url(t, ImageVariant::FourFour)}
                    style="width: 40vmin; height: 40vmin;"
                />
            },
            get_img_url(t, ImageVariant::ThumbnailBackdrop),
        ),
    };

    html! {
        <div class="full-square" style={format!("background-image: url({})", bg)}>
            {corners}
            <div style="position: absolute; pointer-events: none; top: 0; bottom: 0; left: 0; right: 0; display: flex; justify-content: center; align-items: center;">
                {center}
            </div>
        </div>
    }
}
