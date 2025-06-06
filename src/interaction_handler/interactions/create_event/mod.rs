mod role;
mod event_data;

use std::collections::HashSet;
use std::sync::Arc;
use event_data::{EventData, EventDataBuilder};
use tokio::sync::Mutex;
use role::Role;
use crate::discord::interaction_response::InteractionResponseBuilder;
use crate::interaction_handler::InteractionProcessor;
use crate::discord::{embed::{EmbedBuilder, EmbedImage, EmbedFooterBuilder, EmbedField}, emoji::Emoji, interaction::Interaction, interaction_response::{ActionRow, ComponentBuilder, InteractionCallbackDataBuilder, InteractionResponse}};
use crate::persistence::redis_storage::RedisStorage;
use crate::utils::timestamp::RegistrationTime;

#[derive(Clone)]
pub struct CreateEvent {
    redis_storage: Arc<Mutex<RedisStorage>>,
    event_data: EventData,
}

impl CreateEvent {
    pub fn new(redis_storage: Arc<Mutex<RedisStorage>>) -> CreateEvent{
        CreateEvent { 
            event_data: EventData::default(),
            redis_storage,
        }
    }

    fn generate_event_embed(&self) -> InteractionResponse {
        let roles = self.event_data.get_roles();
        let unix_timestamp = self.event_data.get_time();
        let utc_timestamp = RegistrationTime::unix_to_utc(unix_timestamp);

        let mut description_fields = vec![
            EmbedField::new("Event Info:", format!("📅 Local time: <t:{unix_timestamp}:F>\n📅 UTC time: {utc_timestamp}\n⏰ In : <t:{unix_timestamp}:R>"), false),
            EmbedField::new("Description:", "description goes here", false),
        ];
        description_fields.append(&mut Role::roles_to_embedfields(&roles));

        let description_embed = EmbedBuilder::default()
            .thumbnail(EmbedImage::new("https://i.imgur.com/EVXo4CB.jpeg"))
            .title("Event title goes here".into())
            .fields(description_fields)
            .build()
            .unwrap();
            
        let picture_embed = EmbedBuilder::default()
            // .image(EmbedImage::new("https://i.imgur.com/z28A4yA.jpeg")) // dev
            .image(EmbedImage::new("https://i.imgur.com/RiB0TBM.jpeg")) // presentation
            .footer(EmbedFooterBuilder::default().text(format!("Unique Signups: {}", unique_signups(&roles))).build().unwrap())
            .build()
            .unwrap();

        let mut rows = generate_buttons(&roles);
        rows.append(&mut vec![ 
            ActionRow::new(vec![ 
                ComponentBuilder::default()
                    .component_type(2)
                    .style(2)
                    .label(String::from("Pregear"))
                    .custom_id(String::from("Pregear"))
                    .emoji(Emoji {
                        id: None,
                        name: Some(String::from("👍"))
                    })
                    .build()
                    .unwrap(),
                ComponentBuilder::default()
                    .component_type(2)
                    .style(4)
                    .label(String::from("Cancel"))
                    .custom_id(String::from("Cancel"))
                    .emoji(Emoji {
                        id: None,
                        name: Some(String::from("✖️"))
                    })
                    .build()
                    .unwrap()
            ])
        ]);

        let data = match InteractionCallbackDataBuilder::default() 
            .embeds(vec![ description_embed, picture_embed ])
            .action_rows(rows)
            .build() {
                Ok(d) => d,
                Err(e) => {
                    crate::log_expression_debug!(e);
                    return InteractionResponse::create_emphemeral_message(String::from("Failed to initialize event."));
                }
            };

        return match InteractionResponseBuilder::default()
            .response_type(4)
            .data(data)
            .build() {
                Ok(r) => r,
                Err(e) => {
                    crate::log_expression_debug!(e);
                    return InteractionResponse::create_emphemeral_message(String::from("Failed to initialize event."));
                }
            };
    }
}

#[rocket::async_trait]
impl InteractionProcessor for CreateEvent {
    async fn application_command_action(&mut self, interaction: &Interaction) -> InteractionResponse {
        let time = RegistrationTime::utc_to_unix("5/07/2025 10:00 am".to_string()).unwrap_or_default();

        let event_data = EventDataBuilder::default()
            .event_time(time)
            .build();

        self.event_data = match event_data {
            Ok(ed) => ed,
            Err(e) => {
                crate::log_expression_debug!(e);
                return InteractionResponse::create_emphemeral_message(String::from("Failed to initialize event."));
            }
        };

        let redis_storage = self.redis_storage.lock().await;

        let roles_template_url = match interaction.get_string_option_value_by_name("template") {
            Some(url) => url,
            None => return InteractionResponse::create_emphemeral_message(String::from("Failed to parse url from interaction")),
        };
        let roles = match Role::fetch_role_from_url(&roles_template_url).await {
            Ok(r) => r,
            Err(e) => {
                crate::log_expression!(e);
                return InteractionResponse::create_emphemeral_message(format!("Failed to fetch role template: {}", e));
            }
        };
        self.event_data.set_roles(&roles);

        let _ = redis_storage.persist_json(&interaction.id, &self.event_data).await;

        self.generate_event_embed()
    }

    async fn message_component_action(&mut self, interaction: &Interaction, parent_interaction: &crate::discord::interaction::InteractionMetadata) -> InteractionResponse {
        let redis_storage = self.redis_storage.lock().await;

        let event_id = parent_interaction.id.as_ref().map_or("", |x| x);
        self.event_data = match redis_storage.retrieve_json::<EventData>(&event_id).await {
            Ok(ed) => ed,
            Err(e) => {
                crate::log_expression_debug!(e);
                return InteractionResponse::create_emphemeral_message(String::from("Event corrupted"));
            }
        };

        let reacting_member = interaction.get_interacted_member().unwrap_or("Interacted user not found");
        match interaction.get_button_id() {
            Some("Cancel") => player_cancel(&reacting_member, self.event_data.get_roles_mut()),
            Some("Pregear") => {crate::log_expression!("pregearing");},
            Some(id) => player_pick_role(&reacting_member, id, self.event_data.get_roles_mut()),
            None => {crate::log_expression!("unknown button");}
        }

        let _ = redis_storage.persist_json(&event_id, &self.event_data).await;

        self.generate_event_embed()
    }
}

fn generate_buttons(roles: &[Role]) -> Vec<ActionRow> {
    let role_count = roles.len();
    let row_count = role_count / 5 + 1;
    let rows = (0..row_count).map(|row| {
        let button_count = if row == row_count - 1 { role_count % 5 } else { 5 };
        let components = (0..button_count).map(|button| {
            let role_index = usize::min(role_count - 1, row * 5 + button);
            ComponentBuilder::default()
                .component_type(2)
                .style(1)
                .label(format!("{}", roles[role_index].name))
                .custom_id(format!("{}", roles[role_index].name))
                .emoji(Emoji { id: None, name: Some(roles[role_index].emoji.clone()) })
                .build()
                .unwrap()
        }).collect();

        ActionRow::new(components)
    }).take(if role_count % 5 == 0 {row_count - 1} else {row_count});

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

fn unique_signups(roles: &Vec<Role>) -> usize {
    roles
        .iter()
        .flat_map(|x| x.players.iter())
        .collect::<HashSet<_>>()
        .len()
}
