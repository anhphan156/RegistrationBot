use std::collections::HashMap;
use crate::discord::{interaction::Interaction, interaction_response::{IRStatus, InteractionResponse}};
use message_component::MessageComponent;
use application_command::ApplicationCommand;

pub mod interactions;
mod message_component;
mod application_command;

pub trait InteractionProcessor: ApplicationCommand + MessageComponent {}

type ApplicationCommandBox = Box<dyn InteractionProcessor + Sync + Send>;

pub struct InteractionHandler {
    application_commands: HashMap<&'static str, ApplicationCommandBox>,
}

#[macro_export]
macro_rules! add_interaction {
    ($handler:expr, $(($name:expr, $interaction:expr)),*) => {
        $($handler.add_interaction($name, Box::new($interaction));)*
    };
}

impl InteractionHandler {
    pub fn new() -> InteractionHandler {
        InteractionHandler {
            application_commands: HashMap::new(),
        }
    }

    pub fn add_interaction(&mut self, name: &'static str, command_object: ApplicationCommandBox) {
        self.application_commands.insert(name, command_object);
    }

    pub async fn handle_slash_command(&mut self, interaction: &Interaction) -> InteractionResponse {
        let command_name = interaction.get_command_name().unwrap_or_default();
        let command = match self.application_commands.get_mut(command_name) {
            Some(c) => c,
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


        let command_name = interaction.get_command_name().unwrap_or_default();
        let command = match self.application_commands.get_mut(command_name) {
            Some(c) => c,
            None => return Err(IRStatus::PatchFailed),
        };

        command.message_component_init(interaction, parent_interaction);
        command.message_component_action().await.edit_message(&interaction).await
    }
}
