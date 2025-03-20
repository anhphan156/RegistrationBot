use std::collections::HashMap;

use registration_bot::discord::embed::Embed;
use registration_bot::discord::emoji::Emoji;
use registration_bot::discord::interaction::InteractionType;
use registration_bot::discord::interaction_response::{ActionRow, Component, InteractionCallbackData, InteractionResponse};
use registration_bot::request::raw_body::RawBody;
use rocket::serde::json::{self, Json};
use rocket::Request;

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!!!!!!!!!!!"
}

#[post("/interactions", data = "<body>")]
async fn interactions<'r>(body: RawBody) -> Json<InteractionResponse<'r>> {
    let interaction = match body.json() {
        Some(i) => i,
        None => return Json(InteractionResponse {
            response_type: 4,
            data: Some(InteractionCallbackData {
                content: "Failed to parse interaction json".to_string(),
                ..Default::default()
            })
        })
    };

    let t = interaction.interaction_type;
    let data = interaction.data.unwrap_or_default();
    let name = data.name.unwrap_or_default();

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
                content: "Testing buttons".to_string(),
                embeds: Some(vec![
                    Embed {
                        title: Some("Buttons"),
                        ..Default::default()
                    }
                ]),
                action_rows: Some(vec![
                    ActionRow {
                        component_type: 1,
                        components: Some(vec![
                            Component {
                                component_type: 2,
                                style: 1,
                                label: Some("Tank"),
                                custom_id: Some("Tank Button"),
                                emoji: Some(Emoji { id: None, name: Some("😆"), }),
                            },
                            Component {
                                component_type: 2,
                                style: 1,
                                label: Some("DPS"),
                                custom_id: Some("dpsbtn"),
                                emoji: Some(Emoji { id: None, name: Some("❤️"), }),
                            },
                            Component {
                                component_type: 2,
                                style: 1,
                                label: Some("Healer"),
                                custom_id: Some("healerbtn"),
                                emoji: Some(Emoji { id: None, name: Some("🔥"), }),
                            },
                        ])
                    }
                ])
            })
        });
    }

    // Handle requests from interactive components
    if t == InteractionType::MESSAGECOMPONENT {
        let component_id = data.custom_id.unwrap_or_default();

        let message = interaction.message.unwrap_or_default();
        let message_id = message.id.unwrap_or_default();

        let token = interaction.token.unwrap_or_default();

        let app_id = match std::env::var("APP_ID") {
            Ok(key) => key,
            _ => return Json(InteractionResponse {
                response_type: 4,
                data: Some(InteractionCallbackData {
                    content: format!("App id not found"),
                    ..Default::default()
                }),
            }),
        };

        let new_message = InteractionResponse {
            response_type: 4,
            data: Some(InteractionCallbackData {
                content: format!("Someone clicked on {}", component_id),
                ..Default::default()
            })
        };
        let new_message = "{'body': {'content': 'haha'}}";

        let client = reqwest::Client::new();
        let url = format!("https://discord.com/api/v10/webhooks/{}/{}/messages/{}", app_id, token, message_id);
        println!();
        println!("{}", url);
        println!();
        // let mut new_message = HashMap::new();
        // new_message.insert("content", "haha");
        // let url = "https://00fb-2607-fea8-d22-9a00-727d-16eb-6490-bdf3.ngrok-free.app";
        // let res = client.patch(url).header("Content-Type", "application/json").json(&new_message).send();



        // match res {
        //     Ok(r) => {
        //         if r.status().is_success() {
        //             println!("Patch sucess");
        //         }else {
        //             println!("Patch failed: {}", r.status());
        //         }
        //     },
        //     Err(e) => {
        //         println!("Error: {}", e);
        //     }
        // };

        return Json(InteractionResponse {
            response_type: 4,
            data: Some(InteractionCallbackData {
                content: format!("Someone clicked on {}", component_id),
                ..Default::default()
            })
        });
    }

    let res = InteractionResponse {
        response_type: 4,
        data: Some(InteractionCallbackData {
            content: "Command not found (maybe)".to_string(),
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
