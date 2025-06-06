use serde::{Deserialize, Serialize};
use super::CommandOption;

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct InteractionData {
    pub name: Option<String>,
    pub custom_id: Option<String>,
    pub options: Option<Vec<CommandOption>>,
}
