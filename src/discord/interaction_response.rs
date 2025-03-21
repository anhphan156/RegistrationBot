use serde::{Deserialize, Serialize};
use super::{embed::Embed, emoji::Emoji};

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct InteractionResponse<'r>{
    #[serde(rename = "type")]
    pub response_type: u8,
    #[serde(borrow)]
    pub data: Option<InteractionCallbackData<'r>>
}

impl<'r> InteractionResponse <'_> {
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
pub struct InteractionCallbackData<'r> {
    pub content: Option<String>,

    #[serde(borrow)]
    pub embeds: Option<Vec<Embed<'r>>>,

    pub flags: Option<u16>,

    #[serde(rename = "components")]
    pub action_rows: Option<Vec<ActionRow<'r>>>,
}

impl<'r> Default for InteractionCallbackData<'_> {
    fn default() -> Self {
        InteractionCallbackData { 
            content: Some("Default message. Perhaps you forgot to fill up an embed.".to_string()),
            flags: None,
            embeds: None,
            action_rows: None,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct ActionRow<'r> {
    #[serde(rename = "type")]
    pub component_type: u8,
    #[serde(borrow)]
    pub components: Option<Vec<Component<'r>>>,
}

#[derive(Deserialize, Serialize)]
pub struct Component<'r> {
    #[serde(rename = "type")]
    pub component_type: u8,
    pub style: u8,
    pub label: Option<String>,
    pub custom_id: Option<String>,
    #[serde(borrow)]
    pub emoji: Option<Emoji<'r>>,
}
