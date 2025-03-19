use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct InteractionResponse<'r> {
    pub id: &'r str,
    #[serde(rename = "type")]
    pub response_type: u8,
    pub data: Option<InteractionCallbackData<'r>>
}

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct InteractionCallbackData<'r> {
    pub content: &'r str
}
