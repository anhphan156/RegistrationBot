pub mod interactions;

use std::{collections::HashMap, sync::Arc};
use crate::discord::{interaction::{Interaction, InteractionMetadata}, interaction_response::{IRStatus, InteractionResponse}};

#[rocket::async_trait]
pub trait InteractionProcessor: Sync + Send {
    fn clone_box(&self) -> Box<dyn InteractionProcessor>;

    async fn application_command_action(&mut self, _interaction: &Interaction) -> InteractionResponse {
        InteractionResponse::create_message(String::from("Under construction!"))
    }

    async fn message_component_action(&mut self, _interaction: &Interaction, _parent_interaction: &InteractionMetadata) -> InteractionResponse {
        InteractionResponse::create_message(String::from("Under construction!"))
    }
}

impl Clone for Box<dyn InteractionProcessor>{
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

pub type InteractionMap = HashMap<&'static str, Box<dyn InteractionProcessor>>;

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

    pub async fn handle_message_component(&mut self, interaction: &Interaction) -> Result<IRStatus, IRStatus> {
        let parent_interaction : &InteractionMetadata = match interaction.message.as_ref().and_then(|x| x.parent_interaction.as_ref()) {
            Some(pi) => pi,
            None => return Err(IRStatus::PatchFailed),
        };

        let command_name = interaction.get_command_name().map_or("", |x| x);
        let mut command = match self.interaction_map.get(command_name) {
            Some(c) => c.clone(),
            None => return Err(IRStatus::PatchFailed),
        };

        command.message_component_action(interaction, parent_interaction).await.edit_message(interaction).await
    }
}
