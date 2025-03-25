use serde::{Deserialize, Serialize};
use crate::utils::snowflake::Snowflake;

use super::{embed::Embed, emoji::Emoji};

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct InteractionResponse{
    #[serde(rename = "type")]
    pub response_type: u8,
    pub data: Option<InteractionCallbackData>
}

impl InteractionResponse {
    pub fn send_message(message: String) -> Self {
        InteractionResponse {
            response_type: 4,
            data: Some(InteractionCallbackData {
                content: Some(message),
                ..Default::default()
            })
        }
    } // send_message

    pub fn send_empty_message() -> Self {
        InteractionResponse {
            response_type: 4,
            data: None,
        }
    } // send_empty_message
}

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde", default)]
pub struct InteractionCallbackData {
    pub content: Option<String>,
    pub embeds: Option<Vec<Embed>>,
    pub flags: Option<u16>,
    #[serde(rename = "components")]
    pub action_rows: Option<Vec<ActionRow>>,
}

impl Default for InteractionCallbackData {
    fn default() -> Self {
        InteractionCallbackData { 
            content: Some("Default message. Perhaps you forgot to fill up an embed.".to_string()),
            flags: None,
            embeds: None,
            action_rows: None,
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct ActionRow {
    #[serde(rename = "type")]
    pub component_type: u8,
    pub components: Option<Vec<Component>>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Component {
    #[serde(rename = "type")]
    pub component_type: u8,
    pub style: u8,
    pub label: Option<String>,
    pub custom_id: Option<String>,
    pub emoji: Option<Emoji>,
}
