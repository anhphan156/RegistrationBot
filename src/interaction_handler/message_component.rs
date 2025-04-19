use crate::discord::{interaction::{Interaction, InteractionMetadata}, interaction_response::InteractionResponse};

#[rocket::async_trait]
pub trait MessageComponent {
    fn message_component_init(&mut self, _interaction: &Interaction, _parent_interaction: &InteractionMetadata){

    }

    async fn message_component_action(&self) -> InteractionResponse {
        InteractionResponse::create_message(String::from("Under construction!"))
    }
}
