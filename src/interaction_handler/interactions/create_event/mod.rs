mod role;
mod event_data;

use std::sync::Arc;
use event_data::{EventData, EventDataBuilder};
use tokio::sync::Mutex;

use role::*;
use crate::interaction_handler::message_component::MessageComponent;
use crate::interaction_handler::{ApplicationCommand, InteractionProcessor};
use crate::discord::embed::{EmbedField, EmbedImage};
use crate::discord::{embed::Embed, emoji::Emoji, interaction::Interaction, interaction_response::{ActionRow, Component, InteractionCallbackData, InteractionResponse}};
use crate::persistence::redis_storage::RedisStorage;
use crate::persistence::Persistence;
use crate::utils::snowflake::Snowflake;
use crate::utils::timestamp::RegistrationTime;

pub struct CreateEvent {
    interaction: Option<Interaction>,
    redis_storage: Arc<Mutex<RedisStorage>>,
    event_data: Option<EventData>,
    event_id: Option<Snowflake>,
}

impl CreateEvent {
    pub fn new(redis_storage: Arc<Mutex<RedisStorage>>) -> CreateEvent{
        CreateEvent { 
            interaction: None,
            event_id: None,
            event_data: None,
            redis_storage,
        }
    }

    fn generate_event_embed(&self, roles: &[Role]) -> InteractionResponse {
        let unix_timestamp = self.event_data.as_ref().expect("Event data not found").get_time();
        let utc_timestamp = RegistrationTime::unix_to_utc(unix_timestamp);
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

        interaction_response
    }
}

impl InteractionProcessor for CreateEvent {}

#[rocket::async_trait]
impl ApplicationCommand for CreateEvent {
    fn application_command_init(&mut self, interaction: &Interaction) {
        let time = RegistrationTime::utc_to_unix("3/25/2025 10:00 am".to_string()).unwrap_or_default();
        let event_data = EventDataBuilder::default()
            .event_time(time)
            .build()
            .unwrap();
        self.interaction = Some(interaction.clone());
        self.event_data = Some(event_data);
        self.event_id = Some(interaction.id.clone());
    }
    async fn application_command_action(&mut self) -> InteractionResponse {
        let event_data = self.event_data.as_mut().expect("Event data not found in create-event interaction");
        let redis_storage = self.redis_storage.lock().await;

        let roles = vec![
            Role { name: "Tank".to_string(), players: vec![], emoji: String::from( "🤣")},
            Role { name: "DPS 1".to_string(), players: vec![], emoji: String::from( "Ⓜ️")},
            Role { name: "DPS 2".to_string(), players: vec![], emoji: String::from( "Ⓜ️")},
            Role { name: "DPS 3".to_string(), players: vec![], emoji: String::from( "Ⓜ️")},
            Role { name: "DPS 4".to_string(), players: vec![], emoji: String::from( "Ⓜ️")},
            Role { name: "DPS 5".to_string(), players: vec![], emoji: String::from( "Ⓜ️")},
            Role { name: "Healer".to_string(), players: vec![], emoji: String::from( "😴")},
        ];
        event_data.set_roles(&roles);

        let event_id = self.event_id.as_ref().map_or("", |x| x);
        let _ = redis_storage.persist_json(&event_id, &event_data).await;

        self.generate_event_embed(&roles)
    }
}

#[rocket::async_trait]
impl MessageComponent for CreateEvent {
    fn message_component_init(&mut self, interaction: &Interaction, parent_interaction: &crate::discord::interaction::InteractionMetadata){
        let event_id = parent_interaction.id.clone().unwrap_or_default();
        let time = RegistrationTime::utc_to_unix("3/25/2025 10:00 am".to_string()).unwrap_or_default();

        let event_data = EventDataBuilder::default()
            .event_time(time)
            .build()
            .unwrap();
        self.interaction = Some(interaction.clone());
        self.event_data = Some(event_data);
        self.event_id = Some(event_id);
    }

    async fn message_component_action(&mut self) -> InteractionResponse {
        let interaction = self.interaction.as_ref().unwrap();
        let redis_storage = self.redis_storage.lock().await;

        let event_id = self.event_id.as_ref().map_or("", |x| x);
        let mut event_data = match redis_storage.retrieve_json::<EventData>(&event_id).await {
            Ok(ed) => ed,
            Err(e) => {
                println!("{:?}", e);
                return InteractionResponse::create_emphemeral_message(String::from("Event corrupted"));
            }
        };

        let data = interaction.data.clone().unwrap_or_default();
        let button_id : String = data.custom_id.unwrap_or_default().try_into().expect("");
        let reacting_member = interaction.get_interacted_member();

        if button_id == "Cancel" {
            player_cancel(&reacting_member, event_data.get_roles_mut());
        }else if button_id == "Pregear" {
            println!("pregearing");
        } else { // roles button
            player_pick_role(&reacting_member, &button_id, event_data.get_roles_mut());
        }

        let _ = redis_storage.persist_json(&event_id, &event_data).await;

        self.generate_event_embed(event_data.get_roles())
    }
}

fn generate_buttons(roles: &[Role]) -> Vec<ActionRow> {
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
