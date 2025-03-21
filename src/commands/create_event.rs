use super::Command;
use crate::discord::interaction::InteractionType;
use crate::discord::{embed::Embed, emoji::Emoji, interaction::Interaction, interaction_response::{ActionRow, Component, InteractionCallbackData, InteractionResponse}};

pub struct CreateEvent<'r> {
    pub interaction: Interaction<'r>,
}

impl<'r> Command<'r> for CreateEvent<'_> {
    fn action(&self) -> InteractionResponse<'r> {

        let mut embed = Embed {
            title: Some("Buttons"),
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
                embeds: Some(vec![ embed ]),
                action_rows: Some(vec![
                    ActionRow {
                        component_type: 1,
                        components: Some(vec![
                            Component {
                                component_type: 2,
                                style: 1,
                                label: Some(format!("Tank")),
                                custom_id: Some(format!("Tank")),
                                emoji: Some(Emoji { id: None, name: Some("🫏"), }),
                            }
                        ])
                    },
                    ActionRow {
                        component_type: 1,
                        components: Some((0..5).map(|x| Component {
                            component_type: 2,
                            style: 1,
                            label: Some(format!("DPS {}", x)),
                            custom_id: Some(format!("DPS {}", x)),
                            emoji: Some(Emoji { id: None, name: Some("😆"), }),
                        }).collect())
                    },
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
                ]),
                ..Default::default()
            })
        };

        return interaction_response;
    }
}
