use serde::{Deserialize, Serialize};
use super::{embed::Embed, emoji::Emoji, interaction::Interaction};

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
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

impl InteractionResponse {
    pub fn new() -> InteractionResponseBuilder {
        InteractionResponseBuilder {
            response_type: 1,
            data: None,
        }
    } // new

    pub fn get_data<'r>(&'r self) -> &'r Option<InteractionCallbackData> {
        &self.data
    } // get_data

    pub async fn send_follow_up_message(&self, interaction: &Interaction) -> Result<IRStatus, IRStatus> {
        let app_id = match std::env::var("APP_ID") {
            Ok(key) => key,
            _ => return Err(IRStatus::AppIdNotFound),
        };

        let token = interaction.token.clone().unwrap_or_default();

        let client = reqwest::Client::new();
        let url = format!("https://discord.com/api/v10/webhooks/{}/{}", app_id, token);
        let res = client.post(url).header("Content-Type", "application/json").json(&self.data).send().await;

        if res.is_err() {
            return Err(IRStatus::PatchFailed);
        }

        Ok(IRStatus::PatchSuccess)
    }
    
    pub async fn edit_message(&self, interaction: &Interaction) -> Result<IRStatus, IRStatus> {
        let app_id = match std::env::var("APP_ID") {
            Ok(key) => key,
            _ => return Err(IRStatus::AppIdNotFound),
        };

        let message_id = interaction.message.clone().unwrap_or_default().id.unwrap_or_default();
        let token = interaction.token.clone().unwrap_or_default();

        let client = reqwest::Client::new();
        let url = format!("https://discord.com/api/v10/webhooks/{}/{}/messages/{}", app_id, token, message_id);
        let res = client.patch(url).header("Content-Type", "application/json").json(&self.data).send().await;

        if res.is_err() {
            return Err(IRStatus::PatchFailed);
        }

        Ok(IRStatus::PatchSuccess)
    }

    pub fn pong() -> Self {
        InteractionResponse {
            response_type: 1,
            data: None
        }
    } // pong

    pub fn create_emphemeral_message(message: String) -> Self {
        InteractionResponse {
            response_type: 4,
            data: Some(InteractionCallbackData {
                content: Some(message),
                flags: Some(1 << 6),
                ..Default::default()
            })
        }
    } // send_message

    pub fn create_message(message: String) -> Self {
        InteractionResponse {
            response_type: 4,
            data: Some(InteractionCallbackData {
                content: Some(message),
                ..Default::default()
            })
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

pub struct InteractionResponseBuilder {
    response_type: u8,
    data: Option<InteractionCallbackData>
}

impl InteractionResponseBuilder {
    pub fn response_type(&mut self, response_type: u8) -> &mut Self {
        self.response_type = response_type;
        self
    }
    pub fn data(&mut self, data: InteractionCallbackData) -> &mut Self {
        self.data = Some(data);
        self
    }

    pub fn build(&self) -> InteractionResponse {
        InteractionResponse {
            response_type: self.response_type,
            data: self.data.clone()
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(crate = "rocket::serde", default)]
pub struct InteractionCallbackData {
    content: Option<String>,
    embeds: Option<Vec<Embed>>,
    flags: Option<u16>,
    #[serde(rename = "components")]
    action_rows: Option<Vec<ActionRow>>,
    title: Option<String>,
    custom_id: Option<String>,
}

impl InteractionCallbackData {
    pub fn new() -> InteractionCallbackDataBuilder {
        InteractionCallbackDataBuilder {
            content: None,
            embeds: None,
            flags: None,
            action_rows: None,
            title: None,
            custom_id: None,
        }
    }
}

pub struct InteractionCallbackDataBuilder {
    content: Option<String>,
    embeds: Option<Vec<Embed>>,
    flags: Option<u16>,
    action_rows: Option<Vec<ActionRow>>,
    title: Option<String>,
    custom_id: Option<String>,
}

impl InteractionCallbackDataBuilder {
    pub fn content(&mut self, content: String) -> &mut Self {
        self.content = Some(content);
        self
    }
    pub fn embeds(&mut self, embeds: Vec<Embed>) -> &mut Self {
        self.embeds = Some(embeds);
        self
    }
    pub fn flags(&mut self, flags: u16) -> &mut Self {
        self.flags = Some(flags);
        self
    }
    pub fn action_rows(&mut self, action_rows: Vec<ActionRow>) -> &mut Self {
        self.action_rows = Some(action_rows);
        self
    }
    pub fn title(&mut self, title: impl Into<String>) -> &mut Self {
        self.title = Some(title.into());
        self
    }
    pub fn custom_id(&mut self, custom_id: impl Into<String>) -> &mut Self {
        self.custom_id = Some(custom_id.into());
        self
    }
    pub fn build(&self) -> InteractionCallbackData {
        InteractionCallbackData {
            content: self.content.clone(),
            embeds: self.embeds.clone(),
            flags: self.flags,
            action_rows: self.action_rows.clone(),
            custom_id: self.custom_id.clone(),
            title: self.title.clone(),
        }
    }
}

impl Default for InteractionCallbackData {
    fn default() -> Self {
        InteractionCallbackData { 
            content: Some("Default message. Perhaps you forgot to fill up an embed.".to_string()),
            flags: None,
            embeds: None,
            action_rows: None,
            title: None,
            custom_id: None,
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ActionRow {
    #[serde(rename = "type")]
    component_type: u8,
    components: Option<Vec<Component>>,
}

impl ActionRow {
    pub fn new(components: Vec<Component>) -> Self {
        ActionRow {
            component_type: 1,
            components: Some(components),
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Component {
    #[serde(rename = "type")]
    component_type: u8,
    style: u8,
    label: Option<String>,
    custom_id: Option<String>,
    emoji: Option<Emoji>,
}

impl Component {
    pub fn new(component_type: u8, style: u8) -> ComponentBuilder {
        ComponentBuilder {
            component_type,
            style,
            label: None,
            custom_id: None,
            emoji: None,
        }
    }
}

pub struct ComponentBuilder {
    component_type: u8,
    style: u8,
    label: Option<String>,
    custom_id: Option<String>,
    emoji: Option<Emoji>,
}

impl ComponentBuilder {
    pub fn custom_id(&mut self, custom_id: String) -> &mut Self {
        self.custom_id = Some(custom_id);
        self
    }
    pub fn emoji(&mut self, emoji: Emoji) -> &mut Self {
        self.emoji = Some(emoji);
        self
    }
    pub fn label(&mut self, label: String) -> &mut Self {
        self.label = Some(label);
        self
    }
    pub fn build(&self) -> Component {
        Component {
            component_type: self.component_type,
            style: self.style,
            label: self.label.clone(),
            custom_id: self.custom_id.clone(),
            emoji: self.emoji.clone(),
        }
    }
}
