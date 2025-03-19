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
                content: "Failed to parse interaction json"
            })
        })
    };

    let t = &interaction.interaction_type;

    if *t == 1 {
        return Json(InteractionResponse {
            response_type: 1,
            data: None
        })
    }

    let res = InteractionResponse {
        response_type: 4,
        data: Some(InteractionCallbackData {
            content: "Received"
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
