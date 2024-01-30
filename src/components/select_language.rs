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
pub struct Props {}

#[function_component]
pub fn SelectLanguage(props: &Props) -> Html {
    let top_text = use_state(|| GREETINGS[0]);
    let counter = use_counter(0);

    use_interval(
        {
            let top_text = top_text.clone();
            let counter = counter.clone();
            move || {
                top_text.set(GREETINGS[*counter as usize % GREETINGS.len()]);
                counter.increase();
            }
        },
        2000,
    );

    html! {
        <div class="select-language">
            <h2><i>{*top_text}</i></h2>
            <LangButt
                img_id={AttrValue::from("b00cd3a4-625a-4133-d22d-5049d2fa3200")}
                name={AttrValue::from("English")}
            />
            <LangButt
                img_id={AttrValue::from("8ce49019-4382-474a-7e42-6c93f59fc900")}
                name={AttrValue::from("Français")}
            />
            <LangButt
                img_id={AttrValue::from("3614c679-2729-45f5-71c7-2db42fe3b900")}
                name={AttrValue::from("Deutsch")}
            />
            <LangButt
                img_id={AttrValue::from("2470e130-473d-4428-631c-b6f638efcf00")}
                name={AttrValue::from("Русский")}
            />
            <LangButt
                img_id={AttrValue::from("336f375b-a7f5-43f3-e3c6-db629c613000")}
                name={AttrValue::from("Español")}
            />
            <LangButt
                img_id={AttrValue::from("b5a9d97a-5b92-43bf-fda9-3f05e8788e00")}
                name={AttrValue::from("Português")}
            />
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct LangButtProps {
    img_id: AttrValue,
    name: AttrValue,
}

#[function_component]
fn LangButt(props: &LangButtProps) -> Html {
    let LangButtProps { img_id, name } = props;

    html! {
        <div
            class="language-button"
            style={format!("background-image: url(https://imagedelivery.net/MRTPzGIpYfy00UVryjholQ/{}/AnasoThumbnailBackdrop)", img_id)}
        >
            <div>
                <img src={format!("https://imagedelivery.net/MRTPzGIpYfy00UVryjholQ/{}/AnasoThumbnail", img_id)} />
                <span>{name}</span>
                <span/>
            </div>
        </div>
    }
}
