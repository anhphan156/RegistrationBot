// https://discord.com/developers/docs/components/reference#component-object

use serde::{Deserialize, Serialize};
use derive_builder::Builder;
use crate::discord::emoji::Emoji;

#[derive(Deserialize, Serialize, Clone)]
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

#[derive(Deserialize, Serialize, Clone, Default, Builder)]
#[builder(setter(strip_option), default)]
pub struct Component {
    #[serde(rename = "type")]
    component_type: u8,
    style: u8,
    label: Option<String>,
    custom_id: Option<String>,
    emoji: Option<Emoji>,
}
