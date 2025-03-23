use super::Command;
use crate::discord::embed::{EmbedField, EmbedImage};
use crate::discord::interaction::InteractionType;
use crate::discord::{embed::Embed, emoji::Emoji, interaction::Interaction, interaction_response::{ActionRow, Component, InteractionCallbackData, InteractionResponse}};

pub struct CreateEvent {
    pub interaction: Interaction,
}

struct Role {
    pub name: String,
    pub players: Vec<String>,
    pub emoji: &'static str,
}

impl CreateEvent {
    fn persist_event(&self){

    }
}

impl Command for CreateEvent {
    fn action(&self) -> InteractionResponse {
        let mut roles = vec![
                Role { name: "Tank".to_string(), players: vec![], emoji: "🤣"},
                Role { name: "DPS 1".to_string(), players: vec![], emoji: "Ⓜ️"},
                Role { name: "DPS 2".to_string(), players: vec![], emoji: "Ⓜ️"},
                Role { name: "DPS 3".to_string(), players: vec![], emoji: "Ⓜ️"},
                Role { name: "DPS 4".to_string(), players: vec![], emoji: "Ⓜ️"},
                Role { name: "DPS 5".to_string(), players: vec![], emoji: "Ⓜ️"},
                Role { name: "Healer".to_string(), players: vec![], emoji: "😴"},
            ];

        let mut rows = generate_buttons(&roles);
        rows.append(&mut vec![ 
            ActionRow {
                component_type: 1,
                components: Some(vec![
                    Component {
                        component_type: 2,
                        style: 1,
                        label: Some("Cancel".to_string()),
                        custom_id: Some("cancel".to_string()),
                        emoji: Some(Emoji { id: None, name: Some(String::from("❌")), }),
                    },
                ])
            },
        ]);

        let interaction_type = self.interaction.interaction_type;
        if interaction_type == InteractionType::MESSAGECOMPONENT {
            let data = self.interaction.data.clone().unwrap_or_default();
            let chosen_role_id : String = data.custom_id.unwrap_or_default().try_into().expect("");

            let member = self.interaction.member.clone().unwrap_or_default();
            let reacting_member : String = member.nick.unwrap_or_default().try_into().expect("");

            if let Some(i) = roles.iter().position(|x| x.name == chosen_role_id) {
                roles[i].players.push(reacting_member);
            };
        };

        let mut embed = roles_to_embed(&roles);
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

        let interaction_response = InteractionResponse {
            response_type: 4,
            data: Some(InteractionCallbackData {
                content: None,
                embeds: Some(vec![ embed, embed2 ]),
                action_rows: Some(rows),
                ..Default::default()
            })
        };

        return interaction_response;
    }
}

fn generate_buttons(roles: &Vec<Role>) -> Vec<ActionRow> {
    let role_count = roles.len();
    let row_count = role_count / 5 + 1;
    let rows = (0..row_count).map(|row| ActionRow {
        component_type: 1,
        components: {
            let button_count = if row == row_count - 1 { role_count % 5 } else { 5 };
            Some((0..button_count).map(|button| {
                let role_index = usize::min(role_count - 1, row * 5 + button);
                Component {
                    component_type: 2,
                    style: 1,
                    label: Some(format!("{}", roles[role_index].name)),
                    custom_id: Some(format!("{}", roles[role_index].name)),
                    emoji: Some(Emoji { id: None, name: Some(String::from(roles[role_index].emoji)), }),
                }
            }).collect()) 
        },
    }).collect();

    rows
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
        title: Some(String::from("roles")),
        fields: Some(fields),
        ..Default::default()
    }
}
