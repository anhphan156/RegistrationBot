use serde::{Deserialize, Serialize};
use derive_builder::Builder;

#[derive(Deserialize, Serialize, Debug, Clone, Builder)]
pub struct EmbedFooter {
    text: String,
    
    // #[builder(default = "None")]
    // icon_url: Option<String>,
}
