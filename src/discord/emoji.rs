use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Emoji {
    pub id: Option<String>,
    pub name: Option<String>,
}
