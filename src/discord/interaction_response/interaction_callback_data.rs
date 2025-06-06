// https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-response-object-interaction-callback-data-structure

use serde::{Deserialize, Serialize};
use derive_builder::Builder;
use crate::discord::embed::Embed;
use super::ActionRow;

#[derive(Deserialize, Serialize, Clone, Default, Builder)]
#[serde(crate = "rocket::serde", default)]
#[builder(setter(strip_option), default)]
pub struct InteractionCallbackData {
    content: Option<String>,
    embeds: Option<Vec<Embed>>,
    flags: Option<u16>,
    #[serde(rename = "components")]
    action_rows: Option<Vec<ActionRow>>,
}
