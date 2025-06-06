use serde::{Deserialize, Serialize};
use crate::discord::embed::Embed;
use super::InteractionMetadata;


#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct Message {
    pub id: Option<String>,
    pub embeds: Option<Vec<Embed>>,
    #[serde(rename="interaction")]
    pub parent_interaction: Option<InteractionMetadata>,
}
