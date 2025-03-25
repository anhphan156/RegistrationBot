use serde::{Deserialize, Serialize};
use super::{embed::Embed, emoji::Emoji};

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct InteractionResponse{
    #[serde(rename = "type")]
    response_type: u8,
    data: Option<InteractionCallbackData>
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

    pub fn pong() -> Self {
        InteractionResponse {
            response_type: 1,
            data: None
        }
    } // pong

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

// impl ActionRow {
//     pub fn new(component_type: u8, components: Option<>)
// }

#[derive(Deserialize, Serialize, Clone)]
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
