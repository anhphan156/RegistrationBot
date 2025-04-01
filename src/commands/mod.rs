use crate::discord::interaction_response::InteractionResponse;

pub mod create_event;

pub trait Command {
    fn action(&self) -> InteractionResponse;
}

pub struct CommandHandler {

}
