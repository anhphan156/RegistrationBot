use std::collections::HashMap;
use crate::discord::{interaction::Interaction, interaction_response::InteractionResponse};

pub mod create_event;

pub trait Command {
    fn init(&mut self, interaction: &Interaction);
    fn action(&self) -> InteractionResponse;
}

type CommandBox = Box<dyn Command + Sync + Send>;

pub struct CommandHandler {
    commands: HashMap<&'static str, CommandBox>,
}

impl CommandHandler {
    pub fn new() -> CommandHandler {
        CommandHandler {
            commands: HashMap::new(),
        }
    }
    pub fn add_command(&mut self, name: &'static str, command_object: CommandBox) {
        self.commands.insert(name, command_object);
    }

    pub fn handle_application_command(&mut self, interaction: &Interaction) -> InteractionResponse {
        let command_name = interaction.get_command_name().unwrap_or_default();
        let command = match self.commands.get_mut(command_name) {
            Some(c) => c,
            None => return InteractionResponse::send_message(String::from("Command not found")),
        };
        command.init(interaction);
        command.action()
    }

    pub async fn handle_interactive_component(&mut self, interaction: &Interaction) {
        let interaction = interaction.clone();

        let app_id = match std::env::var("APP_ID") {
            Ok(key) => key,
            _ => panic!("App id not found")
        };

        let message = interaction.message.clone().unwrap_or_default();
        let message_id : String = message.id.unwrap_or_default().try_into().expect("");
        let token : String = interaction.token.clone().unwrap_or_default().try_into().expect("");

        let parent_interaction_id = message.parent_interaction.unwrap_or_default().id.unwrap_or_default();

        let command_name = interaction.get_command_name().unwrap_or_default();
        let command = match self.commands.get_mut(command_name) {
            Some(c) => c,
            None => panic!("Command not found")
        };
        command.init(&interaction);
        let interaction_response = command.action();
        let new_message = interaction_response.get_data();

        let client = reqwest::Client::new();
        let url = format!("https://discord.com/api/v10/webhooks/{}/{}/messages/{}", app_id, token, message_id);
        let _res = client.patch(url).header("Content-Type", "application/json").json(new_message).send().await;
    }
}
