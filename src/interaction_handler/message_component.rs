use crate::discord::{interaction::{Interaction, InteractionMetadata}, interaction_response::InteractionResponse};

pub trait MessageComponent {
    fn message_component_init(&mut self, _interaction: &Interaction, _parent_interaction: &InteractionMetadata){

    }

    fn message_component_action(&self) -> InteractionResponse {
        InteractionResponse::send_message(String::from("Under construction!"))
    }
}
