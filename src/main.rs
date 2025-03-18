use rocket::serde::json::Json;
use registration_bot::{entity::interaction::Interaction, guards::key_guard::KeyGuard};

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!!!!!!!!!!!"
}

#[post("/interactions", data = "<interaction>")]
fn interactions(interaction: Json<Interaction<'_>>, _key_guard: KeyGuard) -> &'static str {
    let t = &interaction.application_id;
    println!("{}", t);

    "asdf"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![interactions])
}
