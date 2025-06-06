mod embed_field;
mod embed_footer;
mod embed_image;

pub use self::{embed_footer::*, embed_field::EmbedField, embed_image::EmbedImage};
use serde::{Deserialize, Serialize};
use derive_builder::Builder;

#[derive(Deserialize, Serialize, Debug, Clone, Builder)]
#[builder(setter(strip_option), default)]
pub struct Embed {
    title: Option<String>,
    #[serde(rename = "type")]
    embed_type: Option<String>,
    description: Option<String>,
    url: Option<String>,
    color: Option<u32>,
    image: Option<EmbedImage>,
    thumbnail: Option<EmbedImage>,
    fields: Option<Vec<EmbedField>>,
    footer: Option<EmbedFooter>,
}

impl Default for Embed {
    fn default() -> Self {
        Embed {
            title: Some(String::from("Embed Title")),
            embed_type: Some(String::from("rich")),
            description: None,
            url: None,
            color: Some(15606357),
            image: None,
            thumbnail: None,
            fields: None,
            footer: None,
        }
    }
}
