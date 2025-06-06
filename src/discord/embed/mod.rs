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
        let lorem = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.".to_string();
        Embed {
            title: Some(String::from("Embed Title")),
            embed_type: Some(String::from("rich")),
            description: Some(lorem),
            url: None,
            color: Some(15606357),
            image: None,
            thumbnail: None,
            fields: None,
            footer: None,
        }
    }
}
