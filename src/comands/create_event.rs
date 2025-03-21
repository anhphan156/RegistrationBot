use super::Command;
use crate::discord::{embed::Embed, emoji::Emoji, interaction::Interaction, interaction_response::{ActionRow, Component, InteractionCallbackData, InteractionResponse}};

pub struct CreateEvent<'r> {
    pub interaction: Interaction<'r>,
}

impl<'r> Command<'r> for CreateEvent<'_> {
    fn action(&self) -> InteractionResponse<'r> {
        return InteractionResponse {
            response_type: 4,
            data: Some(InteractionCallbackData {
                content: None,
                embeds: Some(vec![
                    Embed {
                        title: Some("Buttons"),
                        ..Default::default()
                    }
                ]),
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
    }
}
