use std::{collections::HashMap, sync::Arc};

use crate::discord::{interaction::Interaction, interaction_response::{IRStatus, InteractionResponse}};
use message_component::MessageComponent;
use application_command::ApplicationCommand;

pub mod interactions;
mod message_component;
mod application_command;

pub trait InteractionProcessor: ApplicationCommand + MessageComponent {
    fn clone_box(&self) -> Box<dyn InteractionProcessor + Sync + Send>;
}

pub type ApplicationCommandBox = Box<dyn InteractionProcessor + Sync + Send>;
pub type CommandMap = HashMap<&'static str, ApplicationCommandBox>;

impl Clone for ApplicationCommandBox {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

pub struct InteractionHandler {
    command_map: Arc<CommandMap>
}

impl InteractionHandler {
    pub fn new(command_map: Arc<CommandMap>) -> InteractionHandler {
        InteractionHandler {
            command_map, 
        }
    }

    pub async fn handle_slash_command(&mut self, interaction: &Interaction) -> InteractionResponse {
        let command_name = interaction.get_command_name().map_or("", |x| x);
        let mut command = match self.command_map.get(command_name) {
            Some(c) => c.clone(),
            None => return InteractionResponse::create_message(String::from("Command not found")),
        };

        command.application_command_init(interaction);
        command.application_command_action().await
    }

    pub async fn handle_message_component(&mut self, interaction: &Interaction) -> Result<IRStatus, IRStatus> {
        let message = match interaction.message.as_ref() {
            Some(msg) => msg,
            None => return Err(IRStatus::PatchFailed),
        };

        let parent_interaction = match message.parent_interaction.as_ref() {
            Some(pi) => pi,
            None => return Err(IRStatus::PatchFailed),
        };

        let command_name = interaction.get_command_name().map_or("", |x| x);
        let mut command = match self.command_map.get(command_name) {
            Some(c) => c.clone(),
            None => return Err(IRStatus::PatchFailed),
        };

        command.message_component_init(interaction, parent_interaction);
        command.message_component_action().await.edit_message(&interaction).await
    }
}
