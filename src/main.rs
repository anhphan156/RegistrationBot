use registration_bot::interaction_handler::interactions::create_event::CreateEvent;
use registration_bot::interaction_handler::InteractionHandler;
use registration_bot::discord::interaction::{Interaction, InteractionType};
use registration_bot::discord::interaction_response::InteractionResponse;
use registration_bot::persistence::redis_storage::RedisStorage;
use registration_bot::utils::timestamp::RegistrationTime;
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
async fn interactions<'r>(interaction: Interaction, redis_storage: &State<RedisStorage>) -> Json<InteractionResponse> {

    let mut command_handler = InteractionHandler::new();
    command_handler.add_interaction("create-event", Box::new(CreateEvent::new()));

    match interaction.interaction_type {
        InteractionType::PING => return Json(InteractionResponse::pong()),
        InteractionType::APPLICATIONCOMMAND => return Json(command_handler.handle_slash_command(&interaction)),
        InteractionType::MESSAGECOMPONENT => {
            tokio::spawn(async move {
                let status = command_handler.handle_message_component(&interaction).await;
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
    let redis_storage = RedisStorage::new();

    rocket::build()
        .manage(redis_storage)
        .mount("/", routes![index])
        .mount("/", routes![interactions])
}
