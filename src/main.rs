mod discord;
mod interaction_handler;
mod utils;
mod persistence;

use std::sync::Arc;
use interaction_handler::interaction_table::{generate_interaction_map, InteractionMap};
use interaction_handler::InteractionHandler;
use discord::interaction::{Interaction, InteractionType};
use discord::interaction_response::InteractionResponse;
use persistence::redis_storage::RedisStorage;
use utils::timestamp::RegistrationTime;
use rocket::serde::json::Json;
use rocket::State;

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello"
}

#[post("/interactions", data = "<interaction>")]
async fn interactions(interaction: Interaction, command_map: &State<Arc<InteractionMap>>) -> Json<InteractionResponse> {

    let mut interaction_handler = InteractionHandler::new(command_map.inner().clone());

    match interaction.interaction_type {
        InteractionType::PING => return Json(InteractionResponse::pong()),
        InteractionType::APPLICATIONCOMMAND => return Json(interaction_handler.handle_application_command(&interaction).await),
        InteractionType::MESSAGECOMPONENT => {
            tokio::spawn(async move {
                let result = interaction_handler.handle_message_component(&interaction).await;
                if let Some(_) = result.get_data() {
                    if let Err(e) = result.send_follow_up_message(&interaction).await {
                        crate::log_expression_debug!(e);
                    }
                }
            });
            return Json(InteractionResponse::silent_defer())
        },
        _ => return Json(InteractionResponse::create_message(String::from("Unimplemented interaction")))
    }
}

#[launch]
fn rocket() -> _ {
    let application_commands = generate_interaction_map();

    rocket::build()
        .manage(Arc::new(application_commands))
        .mount("/", routes![index])
        .mount("/", routes![interactions])
}
