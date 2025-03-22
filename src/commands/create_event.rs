use super::Command;
use crate::discord::embed::EmbedImage;
use crate::discord::interaction::InteractionType;
use crate::discord::{embed::Embed, emoji::Emoji, interaction::Interaction, interaction_response::{ActionRow, Component, InteractionCallbackData, InteractionResponse}};

pub struct CreateEvent<'r> {
    pub interaction: Interaction<'r>,
}

fn generate_buttons<'r>(roles: Vec<(String, &'static str, u32)>) -> Vec<ActionRow<'r>> {
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
                    label: Some(format!("{}", roles[role_index].0)),
                    custom_id: Some(format!("{}", roles[role_index].0)),
                    emoji: Some(Emoji { id: None, name: Some(roles[role_index].1), }),
                }
            }).collect()) 
        },
    }).collect();

    rows
}

impl<'r> Command<'r> for CreateEvent<'_> {
    fn action(&self) -> InteractionResponse<'r> {

        let mut rows = generate_buttons(vec![
            ("Main Tank".to_string(), "🤣",3),
            ("Sub Tank".to_string(), "🤣",3),
            ("DPS 1".to_string(), "Ⓜ️", 3),
            ("DPS 2".to_string(), "Ⓜ️", 3),
            ("DPS 3".to_string(), "Ⓜ️", 3),
            ("DPS 4".to_string(), "Ⓜ️", 3),
            ("DPS 5".to_string(), "Ⓜ️", 3),
            ("DPS 6".to_string(), "Ⓜ️", 3),
            ("DPS 7".to_string(), "Ⓜ️", 3),
            ("DPS 8".to_string(), "Ⓜ️", 3),
            ("DPS 9".to_string(), "Ⓜ️", 3),
            ("Healer 1".to_string(), "😴", 3),
            ("Healer 2".to_string(), "😴", 3),
        ]);

        rows.append(&mut vec![ 
            ActionRow {
                component_type: 1,
                components: Some(vec![
                    Component {
                        component_type: 2,
                        style: 1,
                        label: Some("Cancel".to_string()),
                        custom_id: Some("cancel".to_string()),
                        emoji: Some(Emoji { id: None, name: Some("❌"), }),
                    },
                ])
            },
        ]);

        let mut embed = Embed {
            title: Some("Buttons"),
            thumbnail: Some(EmbedImage {
                url: "https://i.imgur.com/EVXo4CB.jpeg".to_string()
            }),
            ..Default::default()
        };

        let embed2 = Embed {
            title: None,
            description: None,
            image: Some(EmbedImage {
                url: "https://i.imgur.com/z28A4yA.jpeg".to_string()
            }),
            ..Default::default()
        };

        let interaction_type = self.interaction.interaction_type;
        if interaction_type == InteractionType::MESSAGECOMPONENT {
            let data = self.interaction.data.unwrap_or_default();
            let component_id : String = data.custom_id.unwrap_or_default().try_into().expect("");

            let member = self.interaction.member.unwrap_or_default();
            let reacting_member : String = member.nick.unwrap_or_default().try_into().expect("");

            embed.description = Some(format!("{} clicked on {}", reacting_member, component_id));
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
