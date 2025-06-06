use serde::{Deserialize, Serialize};
use super::User;

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct Member {
    pub nick: Option<String>,
    pub user: Option<User>,
}
