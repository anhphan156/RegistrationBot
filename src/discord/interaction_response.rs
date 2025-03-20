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

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde", default)]
pub struct InteractionCallbackData<'r> {
    pub content: &'r str,
    pub embeds: Option<Vec<Embed<'r>>>,
    #[serde(rename = "components")]
    pub action_rows: Option<Vec<ActionRow<'r>>>,
}

impl<'r> Default for InteractionCallbackData<'_> {
    fn default() -> Self {
        InteractionCallbackData { 
            content: "Default message. Perhaps you forgot to fill up an embed.",
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
    pub label: Option<&'r str>,
    pub custom_id: Option<&'r str>,
    pub emoji: Option<Emoji<'r>>,
}
