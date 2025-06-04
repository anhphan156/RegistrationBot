use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;
use crate::{discord::{interaction::{Interaction, InteractionMetadata}, interaction_response::InteractionResponse}, persistence::redis_storage::RedisStorage};
use super::{interactions::create_event::CreateEvent, InteractionProcessor};

pub type InteractionMap = HashMap<&'static str, InteractionEnum>;

macro_rules! define_interaction_enum {
    ( $($variant:ident),* $(,)? ) => {
        #[derive(Clone)]
        pub enum InteractionEnum {
            $($variant($variant))*
        }

        #[rocket::async_trait]
        impl InteractionProcessor for InteractionEnum {
            async fn application_command_action(&mut self, interaction: &Interaction) -> InteractionResponse {
                match self {
                    $(InteractionEnum::$variant(variant) => variant.application_command_action(interaction).await,)*
                }
            }

            async fn message_component_action(&mut self, interaction: &Interaction, parent_interaction: &InteractionMetadata) -> InteractionResponse {
                match self {
                    $(InteractionEnum::$variant(variant) => variant.message_component_action(interaction, parent_interaction).await,)*
                }
            }
        }

    };
}

define_interaction_enum!(CreateEvent);

pub fn generate_interaction_map() -> InteractionMap {
    let redis_storage = Arc::new(Mutex::new(RedisStorage::new()));

    let mut map: InteractionMap = HashMap::new();

    map.insert("create-event", InteractionEnum::CreateEvent(CreateEvent::new(redis_storage.clone())));

    map
}
