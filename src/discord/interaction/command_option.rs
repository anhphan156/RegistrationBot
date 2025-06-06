use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct CommandOption {
    pub name: Option<String>,

    #[serde(rename="type")]
    pub option_type: Option<u8>,
    pub value: Option<rocket::serde::json::Value>,
}
