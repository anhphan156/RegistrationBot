
use crate::discord::{interaction::Interaction, interaction_response::InteractionResponse};

#[rocket::async_trait]
pub trait ModalSubmit {
    fn modal_submit_init(&mut self, _interaction: &Interaction){

    }

    async fn modal_submit_action(&mut self) -> InteractionResponse {
        InteractionResponse::create_message(String::from("Under construction!"))
    }
}
