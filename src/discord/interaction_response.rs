use serde::{Deserialize, Serialize};
use super::embed::Embed;

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
}

impl<'r> Default for InteractionCallbackData<'_> {
    fn default() -> Self {
        InteractionCallbackData { 
            content: "",
            embeds: None
        }
    }
}
