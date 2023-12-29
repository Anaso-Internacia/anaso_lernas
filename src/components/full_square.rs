use yew::prelude::*;

#[derive(Debug, PartialEq, Hash)]
pub enum TextOrImage {
    Text(AttrValue),
    Image(AttrValue),
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
                html! {
                    <div
                        style="width: 50vmin; height: 50vmin; display: flex; justify-content: center; align-items: center;"
                        onclick={let on_select = on_select.clone(); move |_| on_select.emit(i)}
                    >
                        <span>{t}</span>
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
                        src={t}
                        style={["width: 50vmin; height: 50vmin; transition: filter 0.2s linear;", e].concat()}
                        onclick={let on_select = on_select.clone(); move |_| on_select.emit(i)}
                    />
                }
            }
        })
        .collect::<Html>();

    let center = match center {
        TextOrImage::Text(t) => {
            html! {
                <span style="color: black; font-size: 6vmin; font-weight: 600;" class="border-text">{t}</span>
            }
        }
        TextOrImage::Image(t) => {
            html! {
                <img src={t} style="width: 40vmin; height: 40vmin;" />
            }
        }
    };

    html! {
        <div style="margin:auto; width: 100vmin; height: 100vmin; position: relative; display: grid; grid-template-columns: repeat(2, 1fr);">
            {corners}
            <div style="position: fixed; pointer-events: none; top: 0; bottom: 0; left: 0; right: 0; display: flex; justify-content: center; align-items: center;">
                {center}
            </div>
        </div>
    }
}
