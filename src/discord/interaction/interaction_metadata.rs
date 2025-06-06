use serde::{Deserialize, Serialize};
use super::User;

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct InteractionMetadata {
    pub id: Option<String>,
    pub user: Option<User>,
    pub name: Option<String>,
}
