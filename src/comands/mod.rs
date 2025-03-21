use crate::discord::interaction_response::InteractionResponse;

pub mod create_event;

pub trait Command<'r> {
    fn action(&self) -> InteractionResponse<'r>;
}
