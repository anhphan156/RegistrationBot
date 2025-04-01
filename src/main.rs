use registration_bot::commands::create_event::CreateEvent;
use registration_bot::commands::Command;
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
fn interactions<'r>(interaction: Interaction) -> Json<InteractionResponse> {
    println!("{:?}", interaction);
    let interaction_type = interaction.interaction_type;
    let data = interaction.data.clone().unwrap_or_default();
    let name = data.name.unwrap_or_default();

    // Ping
    if interaction_type == InteractionType::PING {
        return Json(InteractionResponse::pong())
    }

    // create-event command
    if interaction_type == InteractionType::APPLICATIONCOMMAND && name == "create-event" {
        let event_id = interaction.id.clone();
        let command = CreateEvent::new()
            .interaction(interaction)
            .event_id(event_id)
            .event_time({
                let time = RegistrationTime::utc_to_unix("3/25/2025 10:00 am".to_string());
                match time {
                    Ok(t) => t,
                    Err(e) => return Json(InteractionResponse::send_message(format!("Bad datetime: {e}")))
                }
            })
            .build();

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

            let event_id = message.parent_interaction.unwrap_or_default().id.unwrap_or_default();

            let command = CreateEvent::new()
                .interaction(interaction)
                .event_id(event_id)
                .event_time(0)
                .build();
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
        .mount("/", routes![index])
        .mount("/", routes![interactions])
}
