mod discord;
mod interaction_handler;
mod utils;
mod persistence;

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use interaction_handler::{InteractionMap, InteractionHandler, interactions::create_event::CreateEvent};
use discord::interaction::{Interaction, InteractionType};
use discord::interaction_response::InteractionResponse;
use persistence::redis_storage::RedisStorage;
use utils::timestamp::RegistrationTime;
use rocket::serde::json::Json;
use rocket::State;

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> String {
    let t = RegistrationTime::utc_to_unix("5/8/1994 8:00 am".to_string());

    match t {
        Ok(r) => return r.to_string(),
        Err(e) => return e.to_string()
    }
}

#[post("/interactions", data = "<interaction>")]
async fn interactions(interaction: Interaction, command_map: &State<Arc<InteractionMap>>) -> Json<InteractionResponse> {

    let mut interaction_handler = InteractionHandler::new(command_map.inner().clone());

    match interaction.interaction_type {
        InteractionType::PING => return Json(InteractionResponse::pong()),
        InteractionType::APPLICATIONCOMMAND => return Json(interaction_handler.handle_application_command(&interaction).await),
        InteractionType::MESSAGECOMPONENT => {
            tokio::spawn(async move {
                let status = interaction_handler.handle_message_component(&interaction).await;
                match status {
                    Ok(s) => println!("Message component handler: {:?}", s),
                    Err(s) => {
                        println!("Failed to edit message {:?}", s);
                        let _ = InteractionResponse::create_emphemeral_message(String::from("Hell yeah"))
                            .send_follow_up_message(&interaction).await;
                    },
                }
            });
            return Json(InteractionResponse::silent_defer())
        },
        _ => return Json(InteractionResponse::create_message(String::from("Unimplemented interaction")))
    }
}

#[launch]
fn rocket() -> _ {
    let redis_storage = Arc::new(Mutex::new(RedisStorage::new()));

    let mut application_commands: InteractionMap = HashMap::new();
    application_commands.insert("create-event", Box::new(CreateEvent::new(redis_storage.clone())));

    rocket::build()
        .manage(Arc::new(application_commands))
        .mount("/", routes![index])
        .mount("/", routes![interactions])
}
