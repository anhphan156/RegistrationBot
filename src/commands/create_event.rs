use std::fs;
use std::path::Path;
use rocket::serde::json;
use serde::{Deserialize, Serialize};

use super::Command;
use crate::discord::embed::{EmbedField, EmbedImage};
use crate::discord::interaction::InteractionType;
use crate::discord::{embed::Embed, emoji::Emoji, interaction::Interaction, interaction_response::{ActionRow, Component, InteractionCallbackData, InteractionResponse}};
use crate::utils::snowflake::Snowflake;

pub struct CreateEvent {
    pub interaction: Interaction,
    pub event_id: Option<Snowflake>,
}


#[derive(Deserialize, Serialize, Debug, Clone)]
struct Role {
    pub name: String,
    pub players: Vec<String>,
    pub emoji: String,
}

impl CreateEvent {
    fn persist_event<P: AsRef<Path>>(path: P, roles: &Vec<Role>){
        let roles = json::to_string(roles);
        match fs::write(path, roles.unwrap_or_default()) {
            _ => {}
        };
    }

}

impl Command for CreateEvent {
    fn action(&self) -> InteractionResponse {
        let event_file = format!("/tmp/registration-bot-{}.json", self.event_id.clone().unwrap_or_default());

        let mut roles = match fs::read_to_string(&event_file) {
            Ok(content) => json::from_str(&content).unwrap_or(vec![]),
            _ => vec![
                Role { name: "Tank".to_string(), players: vec![], emoji: String::from( "🤣")},
                Role { name: "DPS 1".to_string(), players: vec![], emoji: String::from( "Ⓜ️")},
                Role { name: "DPS 2".to_string(), players: vec![], emoji: String::from( "Ⓜ️")},
                Role { name: "DPS 3".to_string(), players: vec![], emoji: String::from( "Ⓜ️")},
                Role { name: "DPS 4".to_string(), players: vec![], emoji: String::from( "Ⓜ️")},
                Role { name: "DPS 5".to_string(), players: vec![], emoji: String::from( "Ⓜ️")},
                Role { name: "Healer".to_string(), players: vec![], emoji: String::from( "😴")},
            ]
        };

        let interaction_type = self.interaction.interaction_type;
        if interaction_type == InteractionType::MESSAGECOMPONENT {
            let data = self.interaction.data.clone().unwrap_or_default();
            let chosen_role_id : String = data.custom_id.unwrap_or_default().try_into().expect("");

            let member = self.interaction.member.clone().unwrap_or_default();
            let mut reacting_member : String = member.nick.unwrap_or_default().try_into().expect("Failed to parse reacting member");

            if reacting_member == "" {
                let user = self.interaction.clone().user;
                reacting_member = match user {
                    Some(u) => u.username,
                    None => String::from("Username not found")
                };
            }

            if let Some(i) = roles.iter().position(|x| x.name == chosen_role_id) {
                if !roles[i].players.contains(&reacting_member) {
                    roles[i].players.push(reacting_member);
                }
            };
        };

        let mut embed = roles_to_embed(&roles);
        embed.title = Some(String::from("Road anyone?"));
        embed.description = Some(String::from("Help me test the command yall!\nAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAanAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAanAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAanAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAanAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAanAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAanAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAanAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAanAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAa"));
        embed.thumbnail = Some(EmbedImage {
            url: "https://i.imgur.com/EVXo4CB.jpeg".to_string()
        });

        let embed2 = Embed {
            title: None,
            description: None,
            image: Some(EmbedImage {
                url: "https://i.imgur.com/z28A4yA.jpeg".to_string()
            }),
            ..Default::default()
        };

        CreateEvent::persist_event(event_file, &roles);

        let mut rows = generate_buttons(&roles);
        rows.append(&mut vec![ 
            ActionRow::new(1,vec![ 
                Component::new(2, 1)
                    .label(String::from("Cancel"))
                    .custom_id(String::from("Cancel"))
                    .emoji(Emoji {
                        id: None,
                        name: Some(String::from("❌"))
                    })
                    .build()
            ])
        ]);

        let data = InteractionCallbackData {
            content: None,
            embeds: Some(vec![ embed, embed2 ]),
            action_rows: Some(rows),
            ..Default::default()
        };

        let interaction_response = InteractionResponse::new()
            .response_type(4)
            .data(data)
            .build();

        return interaction_response;
    }
}

fn generate_buttons(roles: &Vec<Role>) -> Vec<ActionRow> {
    let role_count = roles.len();
    let row_count = role_count / 5 + 1;
    let rows = (0..row_count).map(|row| {
        let button_count = if row == row_count - 1 { role_count % 5 } else { 5 };
        let components = (0..button_count).map(|button| {
            let role_index = usize::min(role_count - 1, row * 5 + button);
            Component::new(2, 1)
                .label(format!("{}", roles[role_index].name))
                .custom_id(format!("{}", roles[role_index].name))
                .emoji(Emoji { id: None, name: Some(roles[role_index].emoji.clone()) })
                .build()
        }).collect();

        ActionRow::new(1, components)
    });

    rows.collect()
}

fn roles_to_embed(roles: &Vec<Role>) -> Embed {
    let fields = roles.iter().map(|role| EmbedField {
        name: format!("{}", role.name),
        value: {
            let players = role.players.join(", ");
            if players.is_empty() { "No one".to_string() } else { players }
        },
        inline: false,
    }).collect();

    Embed {
        fields: Some(fields),
        ..Default::default()
    }
}
