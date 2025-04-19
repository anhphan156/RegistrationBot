use crate::discord::{interaction::Interaction, interaction_response::InteractionResponse};

#[rocket::async_trait]
pub trait ApplicationCommand {
    fn application_command_init(&mut self, _interaction: &Interaction) {

    }

    async fn application_command_action(&self) -> InteractionResponse {
        InteractionResponse::create_message(String::from("Under construction!"))
    }
}
