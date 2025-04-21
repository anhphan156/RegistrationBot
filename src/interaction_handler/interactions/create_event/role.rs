use serde::{Deserialize, Serialize};
use crate::discord::embed::EmbedField;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Role {
    pub name: String,
    pub players: Vec<String>,
    pub emoji: String,
}

pub fn fetch_role_from_url(url: &str) {

}

pub fn roles_to_embedfields(roles: &[Role]) -> Vec<EmbedField> {
    roles.iter().map(|role| EmbedField::new(
        format!("{} {}", role.emoji.clone(), role.name.clone()),
        {
            let players = role.players.join(", ");
            if players.is_empty() { "No one".to_string() } else { players }
        },
        false
    )).collect()
}
