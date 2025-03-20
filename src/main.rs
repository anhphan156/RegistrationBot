use registration_bot::discord::embed::Embed;
use registration_bot::discord::interaction::InteractionType;
use registration_bot::discord::interaction_response::{ActionRow, Component, InteractionCallbackData, InteractionResponse};
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
        None => return Json(InteractionResponse {
            response_type: 4,
            data: Some(InteractionCallbackData {
                content: "Failed to parse interaction json",
                ..Default::default()
            })
        })
    };

    let t = interaction.interaction_type;
    let name = interaction.data.unwrap_or_default().name.unwrap_or_default();

    // Ping
    if t == InteractionType::PING {
        return Json(InteractionResponse {
            response_type: 1,
            data: None
        })
    }

    // create-event command
    if t == InteractionType::APPLICATIONCOMMAND && name == "create-event" {
        return Json(InteractionResponse {
            response_type: 4,
            data: Some(InteractionCallbackData {
                content: "Testing buttons",
                embeds: Some(vec![
                    Embed {
                        title: Some("Buttons"),
                        ..Default::default()
                    }
                ]),
                action_rows: Some(vec![
                    ActionRow {
                        component_type: 1,
                        components: Some(vec![Component {
                            component_type: 2,
                            style: 1,
                            label: Some("label"),
                            custom_id: Some("my_button"),
                            // emoji: None,
                        }])
                    }
                ])
            })
        });
    }

    let res = InteractionResponse {
        response_type: 4,
        data: Some(InteractionCallbackData {
            content: "Command not found (maybe)",
            ..Default::default()
        })
    };

    Json(res)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![interactions])
}
