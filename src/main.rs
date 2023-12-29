mod app;
mod components;
mod hooks;

use std::collections::BTreeMap;

use app::App;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
struct WordData<'a> {
    #[serde(borrow)]
    pub image: Vec<&'a str>,
    #[serde(borrow, default)]
    pub alt: Vec<&'a str>,
    #[serde(borrow, default)]
    pub misses: Vec<&'a str>,
}

#[derive(Deserialize)]
struct Data<'a> {
    #[serde(borrow)]
    pub words: BTreeMap<&'a str, WordData<'a>>,
}

lazy_static::lazy_static! {
    static ref DATA: Data<'static> = ron::from_str(include_str!("./data.ron")).unwrap();
}

#[derive(Clone, Copy, Debug)]
struct WordSet(pub &'static str, pub &'static WordData<'static>);

impl PartialEq for WordSet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl From<(&'static str, &'static WordData<'static>)> for WordSet {
    fn from(value: (&'static str, &'static WordData<'static>)) -> Self {
        Self(value.0, value.1)
    }
}

enum ImageVariant {
    FourFour,
    Thumbnail,
    ThumbnailBackdrop,
    Full,
}

impl ImageVariant {
    fn as_str(&self) -> &'static str {
        match self {
            Self::FourFour => "AnasoLernas4x4",
            Self::Thumbnail => "AnasoThumbnail",
            Self::ThumbnailBackdrop => "AnasoThumbnailBackdrop",
            Self::Full => "AnasoLernasFull",
        }
    }
}

fn get_img_url(id: &str, variant: ImageVariant) -> String {
    [
        "https://imagedelivery.net/MRTPzGIpYfy00UVryjholQ/",
        id,
        "/",
        variant.as_str(),
    ]
    .concat()
}

fn main() {
    yew::Renderer::<App>::new().render();
}
