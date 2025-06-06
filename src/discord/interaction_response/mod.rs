mod message_component;
mod interaction_callback_data;

pub use self::{message_component::*, interaction_callback_data::*};
use serde::{Deserialize, Serialize};
use super::{embed::Embed, emoji::Emoji, interaction::Interaction};

#[derive(Deserialize, Serialize, derive_builder::Builder, Default)]
#[serde(crate = "rocket::serde")]
#[builder(setter(strip_option), default)]
pub struct InteractionResponse{
    #[serde(rename = "type")]
    response_type: u8,
    data: Option<InteractionCallbackData>
}

#[derive(Debug)]
pub enum IRStatus {
    AppIdNotFound,
    PatchFailed,
    PatchSuccess,
}

impl std::fmt::Display for IRStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AppIdNotFound => write!(f, "App ID not found"),
            Self::PatchSuccess => write!(f, "Patch succeeded"),
            Self::PatchFailed => write!(f, "Patched failed"),
        }
    }
}
impl std::error::Error for IRStatus {}

impl InteractionResponse {
    pub fn get_data(&self) -> Option<&InteractionCallbackData> {
        self.data.as_ref()
    }

    pub async fn send_follow_up_message(&self, interaction: &Interaction) -> Result<reqwest::Response, Box<dyn std::error::Error>> {
        let app_id = match std::env::var("APP_ID") {
            Ok(key) => key,
            _ => return Err(Box::new(IRStatus::AppIdNotFound)),
        };

        let token = interaction.token.clone().unwrap_or_default();

        let client = reqwest::Client::new();
        let url = format!("https://discord.com/api/v10/webhooks/{}/{}", app_id, token);
        let res = client.post(url).header("Content-Type", "application/json").json(&self.data).send().await?;

        Ok(res)
    }
    
    pub async fn edit_message(&self, interaction: &Interaction) -> Result<reqwest::Response, Box<dyn std::error::Error>> {
        let app_id = match std::env::var("APP_ID") {
            Ok(key) => key,
            _ => return Err(Box::new(IRStatus::AppIdNotFound)),
        };

        let message_id = interaction.message.clone().unwrap_or_default().id.unwrap_or_default();
        let token = interaction.token.clone().unwrap_or_default();

        let client = reqwest::Client::new();
        let url = format!("https://discord.com/api/v10/webhooks/{}/{}/messages/{}", app_id, token, message_id);
        let res = client.patch(url).header("Content-Type", "application/json").json(&self.data).send().await?;

        Ok(res)
    }

    pub fn pong() -> Self {
        InteractionResponse {
            response_type: 1,
            data: None
        }
    } // pong

    pub fn create_emphemeral_message(message: String) -> Self {
        let data = InteractionCallbackDataBuilder::default()
            .content(message)
            .flags(1 << 6)
            .build();

        let data = match data {
            Ok(d) => d,
            Err(e) => {
                crate::log_expression_debug!(e);
                InteractionCallbackData::default()
            },
        };

        InteractionResponse {
            response_type: 4,
            data: Some(data)
        }
    } // send_message

    pub fn create_message(message: String) -> Self {
        let data = InteractionCallbackDataBuilder::default()
            .content(message)
            .build();
            
        let data = match data {
            Ok(d) => d,
            Err(e) => {
                crate::log_expression_debug!(e);
                InteractionCallbackData::default()
            },
        };

        InteractionResponse {
            response_type: 4,
            data: Some(data)
        }
    } // send_message

    pub fn create_empty_message() -> Self {
        InteractionResponse {
            response_type: 4,
            data: None,
        }
    } // send_empty_message

    pub fn silent_defer() -> Self {
        InteractionResponse {
            response_type: 6,
            data: None,
        }
    } // silent_defer
}
