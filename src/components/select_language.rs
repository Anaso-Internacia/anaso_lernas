use yew::prelude::*;
use yew_hooks::prelude::*;

static GREETINGS: &[&'static str] = &[
    "Lernu Esperanton!",
    "Learn Esperanto!",
    "Apprenez l'espéranto!",
    "¡Aprende esperanto!",
    "Учите эсперанто!",
    "Lerne Esperanto!",
    "Aprenda esperanto!",
];

#[derive(Properties, PartialEq)]
pub struct Props {
    pub on_selected: Callback<&'static str>,
}

#[function_component]
pub fn SelectLanguage(props: &Props) -> Html {
    let Props { on_selected } = props;
    let butts = [
        (
            "b00cd3a4-625a-4133-d22d-5049d2fa3200",
            "English",
            "Learn Esperanto!",
            "en",
        ),
        (
            "8ce49019-4382-474a-7e42-6c93f59fc900",
            "Français",
            "Apprenez l'espéranto!",
            "fr",
        ),
        (
            "3614c679-2729-45f5-71c7-2db42fe3b900",
            "Deutsch",
            "Lerne Esperanto!",
            "de",
        ),
        (
            "2470e130-473d-4428-631c-b6f638efcf00",
            "Русский",
            "Учите эсперанто!",
            "ru",
        ),
        (
            "336f375b-a7f5-43f3-e3c6-db629c613000",
            "Español",
            "¡Aprende esperanto!",
            "es",
        ),
        (
            "b5a9d97a-5b92-43bf-fda9-3f05e8788e00",
            "Português",
            "Aprenda esperanto!",
            "pt",
        ),
    ];

    let top_text = use_state(|| GREETINGS[0]);
    let counter = use_counter(0);
    let selected_language = use_state(|| None::<usize>);

    use_interval(
        {
            let top_text = top_text.clone();
            let counter = counter.clone();
            let selected_language = selected_language.clone();
            move || {
                if selected_language.is_none() {
                    top_text.set(GREETINGS[*counter as usize % GREETINGS.len()]);
                    counter.increase();
                }
            }
        },
        2000,
    );

    let butt_components = butts
        .iter()
        .enumerate()
        .map(|(i, butt)| {
            let selected_language = selected_language.clone();
            let on_selected = on_selected.clone();
            let top_text = top_text.clone();
            let text = butt.2;
            let lang_code = butt.3;
            html! {
                <LangButt
                    img_id={AttrValue::from(butt.0)}
                    name={AttrValue::from(butt.1)}
                    is_selected={Some(i) == *selected_language}
                    on_click={
                        if Some(i) == *selected_language {
                            Callback::from(move |_| on_selected.emit(lang_code))
                        } else {
                            Callback::from(move |_| {
                                selected_language.set(Some(i));
                                top_text.set(text);
                            })
                        }
                    }
                />
            }
        })
        .collect::<Html>();

    html! {
        <div class="select-language">
            <h2 class="top-text"><i>{*top_text}</i></h2>
            {butt_components}
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct LangButtProps {
    img_id: AttrValue,
    name: AttrValue,
    is_selected: bool,
    on_click: Callback<()>,
}

#[function_component]
fn LangButt(props: &LangButtProps) -> Html {
    let LangButtProps {
        img_id,
        name,
        is_selected,
        on_click,
    } = props;

    html! {
        <div
            class={if *is_selected {"language-button selected"} else {"language-button"}}
            style={format!("background-image: url(https://imagedelivery.net/MRTPzGIpYfy00UVryjholQ/{}/AnasoThumbnailBackdrop)", img_id)}
        >
            <div onclick={let on_click = on_click.clone(); move |_| {on_click.emit(())}}>
                <img src={format!("https://imagedelivery.net/MRTPzGIpYfy00UVryjholQ/{}/AnasoThumbnail", img_id)} />
                <span>{name}</span>
                <span class={if *is_selected {"go"} else {"go-placeholder"}}>{"▷"}</span>
            </div>
        </div>
    }
}
