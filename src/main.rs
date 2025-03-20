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
        None => return Json(InteractionResponse {
            response_type: 4,
            data: Some(InteractionCallbackData {
                content: "Failed to parse interaction json",
                ..Default::default()
            })
        })
    };

    let t = interaction.interaction_type;
    let name = interaction.data.unwrap_or_default().name;

    // Ping
    if t == InteractionType::PING {
        return Json(InteractionResponse {
            response_type: 1,
            data: None
        })
    }

    if t == InteractionType::APPLICATIONCOMMAND && name == "test1" {
        return Json(InteractionResponse {
            response_type: 4,
            data: Some(InteractionCallbackData {
                content: "Testing buttons",
                embeds: Some(vec![
                    Embed {
                        title: Some("Buttons"),
                        // description: Some("Test description"),
                        ..Default::default()
                    }
                ])
            })
        });
    }

    let res = InteractionResponse {
        response_type: 4,
        data: Some(InteractionCallbackData {
            content: "let's fucking go",
            embeds: Some(vec![
                Embed {
                    title: Some("Test embed"),
                    // description: Some("Test description"),
                    ..Default::default()
                }
            ])
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
