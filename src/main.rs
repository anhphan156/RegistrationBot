use registration_bot::commands::create_event::CreateEvent;
use registration_bot::commands::CommandHandler;
use registration_bot::discord::interaction::{Interaction, InteractionType};
use registration_bot::discord::interaction_response::InteractionResponse;
use registration_bot::utils::timestamp::RegistrationTime;
use rocket::serde::json::Json;

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
async fn interactions<'r>(interaction: Interaction) -> Json<InteractionResponse> {

    let mut command_handler = CommandHandler::new();
    command_handler.add_command("create-event", Box::new(CreateEvent::new()));

    match interaction.interaction_type {
        InteractionType::PING => return Json(InteractionResponse::pong()),
        InteractionType::APPLICATIONCOMMAND => return Json(command_handler.handle_application_command(&interaction)),
        InteractionType::MESSAGECOMPONENT => {
            tokio::spawn(async move {
                command_handler.handle_interactive_component(&interaction).await
            });
            return Json(InteractionResponse::silent_defer())
        },
        _ => return Json(InteractionResponse::send_message(String::from("Unimplemented interaction")))
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![interactions])
}
