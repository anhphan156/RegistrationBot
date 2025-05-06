use serde::{Deserialize, Serialize};
use crate::discord::embed::EmbedField;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Role {
    pub name: String,
    #[serde(default)]
    pub players: Vec<String>,
    pub emoji: String,
}

impl Role {
    pub async fn fetch_role_from_url(url: &str) -> Result<Vec<Role>, Box<dyn std::error::Error>> {

        use rocket::serde::json;

        let text : String = reqwest::get(url).await?.text().await?.into();
        let roles : Vec<Role> = match json::from_str(&text) {
            Ok(r) => r,
            Err(e) => {
                println!("{}", e);
                return Err(Box::new(e));
            },
        };

        Ok(roles)
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
}
