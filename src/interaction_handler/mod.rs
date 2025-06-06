pub mod interactions;
pub mod interaction_table;

use std::{collections::HashMap, sync::Arc};
use interaction_table::InteractionMap;
use crate::discord::{interaction::{Interaction, InteractionMetadata}, interaction_response::{IRStatus, InteractionResponse}};

#[rocket::async_trait]
pub trait InteractionProcessor: Sync + Send {
    async fn application_command_action(&mut self, _interaction: &Interaction) -> InteractionResponse {
        InteractionResponse::create_message(String::from("Under construction!"))
    }

    async fn message_component_action(&mut self, _interaction: &Interaction, _parent_interaction: &InteractionMetadata) -> InteractionResponse {
        InteractionResponse::create_message(String::from("Under construction!"))
    }
}

pub struct InteractionHandler {
    interaction_map: Arc<InteractionMap>
}

impl InteractionHandler {
    pub fn new(interaction_map: Arc<InteractionMap>) -> InteractionHandler {
        InteractionHandler {
            interaction_map, 
        }
    }

    pub async fn handle_application_command(&mut self, interaction: &Interaction) -> InteractionResponse {
        let interaction_name = interaction.get_command_name().map_or("", |x| x);
        let mut command = match self.interaction_map.get(interaction_name) {
            Some(c) => c.clone(),
            None => return InteractionResponse::create_message(String::from("Command not found")),
        };

        command.application_command_action(interaction).await
    }

    pub async fn handle_message_component(&mut self, interaction: &Interaction) -> InteractionResponse {
        let parent_interaction : &InteractionMetadata = match interaction.get_interaction_metadata() {
            Some(pi) => pi,
            None => {
                crate::log_expression!("parent interaction not found");
                return InteractionResponse::create_emphemeral_message(String::from("Interaction failed"));
            },
        };

        let command_name = interaction.get_command_name().map_or("", |x| x);
        let mut command = match self.interaction_map.get(command_name) {
            Some(c) => c.clone(),
            None => {
                crate::log_expression!("command not found");
                return InteractionResponse::create_emphemeral_message(String::from("Interaction failed"));
            },
        };

        match command.message_component_action(interaction, parent_interaction).await.edit_message(interaction).await {
            Ok(_) => InteractionResponse::create_empty_message(),
            Err(e) => {
                crate::log_expression_debug!(e);
                InteractionResponse::create_emphemeral_message(String::from("Interaction failed"))
            }
        }
    }
}
