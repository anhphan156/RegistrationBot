use std::collections::HashMap;
use crate::discord::{interaction::Interaction, interaction_response::InteractionResponse};
use message_component::MessageComponent;
use application_command::ApplicationCommand;

pub mod message_component;
pub mod application_command;
pub mod interactions;

pub trait InteractionHandle: ApplicationCommand + MessageComponent {}

type ApplicationCommandBox = Box<dyn InteractionHandle + Sync + Send>;

pub struct InteractionHandler {
    application_commands: HashMap<&'static str, ApplicationCommandBox>,
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

    pub fn handle_slash_command(&mut self, interaction: &Interaction) -> InteractionResponse {
        let command_name = interaction.get_command_name().unwrap_or_default();
        let command = match self.application_commands.get_mut(command_name) {
            Some(c) => c,
            None => return InteractionResponse::send_message(String::from("Command not found")),
        };
        command.application_command_init(interaction);
        command.application_command_action()
    }

    pub async fn handle_message_component(&mut self, interaction: &Interaction) {
        let message = interaction.message.clone().unwrap_or_default();
        let parent_interaction = message.parent_interaction.unwrap_or_default();

        let command_name = interaction.get_command_name().unwrap_or_default();
        let command = match self.application_commands.get_mut(command_name) {
            Some(c) => c,
            None => panic!("Command not found")
        };
        command.message_component_init(&interaction, &parent_interaction);
        // make this function return an error
        let _ = command.message_component_action().edit_message(&interaction).await;
    }
}
