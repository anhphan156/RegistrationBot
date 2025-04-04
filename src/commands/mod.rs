use std::{collections::HashMap, sync::{Arc, Mutex}};

use create_event::CreateEvent;

use crate::discord::{interaction::Interaction, interaction_response::InteractionResponse};

pub mod create_event;

pub trait Command {
    fn init(&mut self, interaction: &Interaction);
    fn action(&self) -> InteractionResponse;
}

type CommandBox = Box<dyn Command + Send + Sync>;

pub struct CommandHandler {
    commands: Mutex<HashMap<&'static str, CommandBox>>,
}

impl CommandHandler {
    pub fn new() -> CommandHandler {
        CommandHandler {
            commands: Mutex::new(HashMap::new()),
        }
    }
    pub fn add_command(&self, name: &'static str, command_object: CommandBox) {
        let mut command_map = self.commands.lock().unwrap();
        command_map.insert(name, command_object);
    }

    pub fn handle_application_command(&self, interaction: &Interaction) -> InteractionResponse {
        let command_name = interaction.get_command_name().unwrap_or_default();
        let mut command_map = self.commands.lock().unwrap();
        let mut command = match command_map.get_mut(command_name) {
            Some(c) => c,
            None => return InteractionResponse::send_message(String::from("Command not found")),
        };
        command.init(interaction);
        command.action()
    }

    pub fn handle_interactive_component(&self, interaction: &Interaction) -> InteractionResponse {
        let interaction = interaction.clone();

        // let command_name = match &interaction.message {
        //     Some(m) => match &m.parent_interaction {
        //         Some(data) => data.name.clone().unwrap_or_default(),
        //         None => String::new(),
        //     }
        //     None => String::new(),
        // };
        // let mut command_map = self.commands.lock().unwrap();
        // let command = match command_map.get_mut(command_name.as_str()) {
        //     Some(c) => c,
        //     None => return InteractionResponse::send_message(String::from("Command not found")),
        // };

        tokio::spawn(async move {
            let app_id = match std::env::var("APP_ID") {
                Ok(key) => key,
                _ => return InteractionResponse::send_message("App id not found".to_string()),
            };

            let message = interaction.message.clone().unwrap_or_default();
            let message_id : String = message.id.unwrap_or_default().try_into().expect("");
            let token : String = interaction.token.clone().unwrap_or_default().try_into().expect("");

            let parent_interaction_id = message.parent_interaction.unwrap_or_default().id.unwrap_or_default();

            // println!("{}", me.test);
            // command.init(&interaction);

            let command = CreateEvent::builder()
                .interaction(interaction)
                .event_id(parent_interaction_id)
                .event_time(0)
                .build();
            let interaction_response = command.action();
            let new_message = interaction_response.get_data();

            let client = reqwest::Client::new();
            let url = format!("https://discord.com/api/v10/webhooks/{}/{}/messages/{}", app_id, token, message_id);
            let _res = client.patch(url).header("Content-Type", "application/json").json(new_message).send().await;

            InteractionResponse::send_empty_message()
        });

        return InteractionResponse::silent_defer();
    }
}
