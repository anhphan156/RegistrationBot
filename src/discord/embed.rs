use serde::{Deserialize, Serialize};

use super::interaction::Interaction;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Embed {
    pub title: Option<String>,
    #[serde(rename = "type")]
    pub embed_type: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
    pub color: Option<u32>,
    pub image: Option<EmbedImage>,
    pub thumbnail: Option<EmbedImage>,
    pub fields: Option<Vec<EmbedField>>,
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
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct EmbedImage {
    pub url: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct EmbedField {
    pub name: String,
    pub value: String,
    pub inline: bool,
}
