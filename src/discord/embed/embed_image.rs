use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct EmbedImage {
    url: String,
}

impl EmbedImage {
    pub fn new(url: impl Into<String>) -> Self {
        EmbedImage { url: url.into() }
    }
}

