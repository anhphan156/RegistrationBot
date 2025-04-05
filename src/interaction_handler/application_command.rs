use crate::discord::{interaction::Interaction, interaction_response::InteractionResponse};

pub trait ApplicationCommand {
    fn application_command_init(&mut self, _interaction: &Interaction) {

    }

    fn application_command_action(&self) -> InteractionResponse {
        InteractionResponse::create_message(String::from("Under construction!"))
    }
}
