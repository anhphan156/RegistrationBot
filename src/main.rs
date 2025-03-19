use registration_bot::entity::interaction::Interaction;
use registration_bot::entity::interaction_response::{InteractionCallbackData, InteractionResponse};
use rocket::serde::json::Json;

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!!!!!!!!!!!"
}

#[post("/interactions", data = "<interaction>")]
fn interactions(interaction: Interaction) -> Json<InteractionResponse> {
    let t = &interaction.interaction_type;
    println!("{}", t);

    let res = InteractionResponse {
        id: "1",
        response_type: 4,
        data: Some(InteractionCallbackData {
            content: "hahaha"
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
