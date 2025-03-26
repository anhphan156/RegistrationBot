use serde::{Deserialize, Serialize};

use super::Command;
use crate::discord::embed::{EmbedField, EmbedImage};
use crate::discord::interaction::InteractionType;
use crate::discord::{embed::Embed, emoji::Emoji, interaction::Interaction, interaction_response::{ActionRow, Component, InteractionCallbackData, InteractionResponse}};
use crate::persistence::file_storage::FileStorage;
use crate::persistence::Persistence;
use crate::utils::snowflake::Snowflake;
use crate::utils::timestamp::RegistrationTime;

pub struct CreateEvent {
    pub interaction: Interaction,
    pub event_id: Option<Snowflake>,
    pub event_time: Option<i64>,
}

impl CreateEvent {
    fn get_reacting_player(&self) -> String {
        let member = self.interaction.member.clone().unwrap_or_default();
        let mut reacting_member : String = member.nick.unwrap_or_default().try_into().expect("Failed to parse reacting member");
        if reacting_member == "" {
            let user = self.interaction.clone().user;
            reacting_member = match user {
                Some(u) => u.username,
                None => String::from("Username not found")
            };
        }
        return reacting_member;
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Role {
    pub name: String,
    pub players: Vec<String>,
    pub emoji: String,
}

impl Command for CreateEvent {
    fn action(&self) -> InteractionResponse {
        let event_file = format!("/tmp/registration-bot-{}.json", self.event_id.clone().unwrap_or_default());
        let event_storage = FileStorage::new(&event_file);

        let mut roles = match event_storage.retrieve_json::<Vec<Role>>() {
            Ok(content) => content,
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

        if self.interaction.interaction_type == InteractionType::MESSAGECOMPONENT {
            let data = self.interaction.data.clone().unwrap_or_default();
            let button_id : String = data.custom_id.unwrap_or_default().try_into().expect("");
            let reacting_member = self.get_reacting_player();

            if button_id == "Cancel" {
                player_cancel(&reacting_member, &mut roles);
            }else if button_id == "Pregear" {
                println!("pregearing");
            } else { // roles button
                player_pick_role(&reacting_member, &button_id, &mut roles);
            }
        };
        let _ = event_storage.persist_json(&roles); // TODO: handle errors

        let utc_timestamp = RegistrationTime::unix_to_utc(self.event_time.unwrap_or_default());
        let unix_timestamp = self.event_time.unwrap_or_default().to_string();
        let description_embed = Embed::new()
            .thumbnail(EmbedImage::new("https://i.imgur.com/EVXo4CB.jpeg"))
            .title("Event title goes here")
            .fields(vec![
                EmbedField::new("Event Info:", format!("📅 Local time: <t:{unix_timestamp}:F>\n📅 UTC time: {utc_timestamp}\n⏰ In : <t:{unix_timestamp}:R>"), false),
                EmbedField::new("Description:", "description goes here", false),
            ])
            .build();
            
        let roles_embed = Embed::new() 
            .fields(roles_to_embedfields(&roles))
            .build();

        let picture_embed = Embed::new()
            // .image(EmbedImage::new("https://i.imgur.com/z28A4yA.jpeg")) // dev
            .image(EmbedImage::new("https://i.imgur.com/RiB0TBM.jpeg")) // presentation
            .build();

        let mut rows = generate_buttons(&roles);
        rows.append(&mut vec![ 
            ActionRow::new(vec![ 
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

        let data = InteractionCallbackData::new() 
            .embeds(vec![ description_embed, roles_embed, picture_embed ])
            .action_rows(rows)
            .build();

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

        ActionRow::new(components)
    });

    rows.collect()
}

fn roles_to_embedfields(roles: &Vec<Role>) -> Vec<EmbedField> {
    roles.iter().map(|role| EmbedField::new(
        format!("{} {}", role.emoji.clone(), role.name.clone()),
        {
            let players = role.players.join(", ");
            if players.is_empty() { "No one".to_string() } else { players }
        },
        false
    )).collect()
}

fn player_cancel(player: &str, roles: &mut Vec<Role>) {
    for role in roles {
        role.players.retain(|x| x != player);
    }
}

fn player_pick_role(player: &str, role_id: &str, roles: &mut Vec<Role>) {
    if let Some(i) = roles.iter().position(|x| x.name == role_id) {
        if !roles[i].players.contains(&String::from(player)) {
            roles[i].players.push(String::from(player));
        }
    };
}
