use rocket::{fairing::{Fairing, Info, Kind}, Request, Response};

pub struct ButtonInteraction;

#[rocket::async_trait]
impl Fairing for ButtonInteraction {
    fn info(&self) -> Info {
        Info {
            name: "Button interaction",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, req: &'r Request<'_>, _res: &mut Response<'r>) {

    }
}
