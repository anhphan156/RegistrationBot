use registration_bot::comands::create_event::CreateEvent;
use registration_bot::comands::Command;
use registration_bot::discord::embed::Embed;
use registration_bot::discord::interaction::InteractionType;
use registration_bot::discord::interaction_response::{InteractionCallbackData, InteractionResponse};
use registration_bot::request::raw_body::RawBody;
use rocket::serde::json::Json;

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!!!!!!!!!!!"
}

#[post("/interactions", data = "<body>")]
fn interactions<'r>(body: RawBody) -> Json<InteractionResponse<'r>> {
    let interaction = match body.json() {
        Some(i) => i,
        None => return Json(InteractionResponse::send_message("Failed to parse interaction json".to_string()))
    };

    let interaction_type = interaction.interaction_type;
    let data = interaction.data.unwrap_or_default();
    let name = data.name.unwrap_or_default();

    // Ping
    if interaction_type == InteractionType::PING {
        return Json(InteractionResponse {
            response_type: 1,
            data: None
        })
    }

    // create-event command
    if interaction_type == InteractionType::APPLICATIONCOMMAND && name == "create-event" {
        let command = CreateEvent {
            interaction,
        };
        return Json(command.action());
    }

    // Handle requests from interactive components
    if interaction_type == InteractionType::MESSAGECOMPONENT {

        let interaction = interaction.clone();
        let message = interaction.message.unwrap_or_default();
        let message_id : String = message.id.unwrap_or_default().try_into().expect("");

        let member = interaction.member.unwrap_or_default();
        let reacting_member : String = member.nick.unwrap_or_default().try_into().expect("");

        let data = interaction.data.unwrap_or_default();
        let component_id : String = data.custom_id.unwrap_or_default().try_into().expect("");
        let component_id_2 = component_id.clone();

        let token : String = interaction.token.unwrap_or_default().try_into().expect("");

        tokio::spawn(async move {
            let app_id = match std::env::var("APP_ID") {
                Ok(key) => key,
                _ => return Json(InteractionResponse {
                    response_type: 4,
                    data: Some(InteractionCallbackData {
                        content: Some(format!("App id not found")),
                        ..Default::default()
                    }),
                }),
            };

            let new_message = InteractionCallbackData {
                content: Some(format!("{} clicked on {}", reacting_member, component_id)),
                embeds: Some(vec![Embed {
                    title: Some("Let's go"),
                    description: Some(&component_id),
                    ..Default::default()
                }]),
                ..Default::default()
            };
            // let mut new_message = HashMap::new();
            // new_message.insert("content", component_id);

            let client = reqwest::Client::new();
            let url = format!("https://discord.com/api/v10/webhooks/{}/{}/messages/{}", app_id, token, message_id);
            let _res = client.patch(url).header("Content-Type", "application/json").json(&new_message).send().await;

            Json(InteractionResponse::send_empty_message())
        });

        return Json(InteractionResponse {
            response_type: 4,
            data: Some(InteractionCallbackData {
                content: Some(format!("Registered {}", component_id_2)),
                flags: Some(1 << 6),
                ..Default::default()
            })
        });
    }

    Json(InteractionResponse::send_message("Command not found (maybe)".to_string()))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![interactions])
}
