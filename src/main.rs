use registration_bot::commands::create_event::CreateEvent;
use registration_bot::commands::Command;
use registration_bot::discord::interaction::{Interaction, InteractionType};
use registration_bot::discord::interaction_response::InteractionResponse;
use registration_bot::utils::snowflake::SnowflakeGenerator;
use rocket::serde::json::Json;
use rocket::State;

#[macro_use] extern crate rocket;

#[get("/")]
fn index(snowflake: &State<SnowflakeGenerator>) -> String {
    snowflake.generate()
}

#[post("/interactions", data = "<interaction>")]
fn interactions<'r>(interaction: Interaction) -> Json<InteractionResponse> {
    let interaction_type = interaction.interaction_type;
    let data = interaction.data.clone().unwrap_or_default();
    let name = data.name.unwrap_or_default();

    // Ping
    if interaction_type == InteractionType::PING {
        return Json(InteractionResponse::pong())
    }

    // create-event command
    if interaction_type == InteractionType::APPLICATIONCOMMAND && name == "create-event" {
        let event_id = Some(interaction.id.clone());
        let command = CreateEvent {
            interaction,
            event_id,
        };
        return Json(command.action());
    }

    // Handle requests from interactive components
    if interaction_type == InteractionType::MESSAGECOMPONENT {
        tokio::spawn(async move {
            let app_id = match std::env::var("APP_ID") {
                Ok(key) => key,
                _ => return Json(InteractionResponse::send_message("App id not found".to_string())),
            };

            let message = interaction.message.clone().unwrap_or_default();
            let message_id : String = message.id.unwrap_or_default().try_into().expect("");
            let token : String = interaction.token.clone().unwrap_or_default().try_into().expect("");

            let event_id = message.parent_interaction.unwrap_or_default().id;

            let command = CreateEvent {
                interaction,
                event_id,
            };
            let interaction_response = command.action();
            let new_message = interaction_response.get_data();

            let client = reqwest::Client::new();
            let url = format!("https://discord.com/api/v10/webhooks/{}/{}/messages/{}", app_id, token, message_id);
            let _res = client.patch(url).header("Content-Type", "application/json").json(new_message).send().await;

            Json(InteractionResponse::send_empty_message())
        });

        return Json(InteractionResponse::silent_defer());
    }

    Json(InteractionResponse::send_message("Command not found (maybe)".to_string()))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(SnowflakeGenerator::new())
        .mount("/", routes![index])
        .mount("/", routes![interactions])
}
