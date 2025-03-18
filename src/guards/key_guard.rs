use rocket::{async_trait, http::Status, request::{FromRequest, Outcome}, serde::Deserialize, Request};

pub struct KeyGuard;

#[async_trait]
impl<'r> FromRequest<'r> for KeyGuard {
    type Error = ();
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let timestamp = request.headers().get_one("X-Signature-Timestamp");
        let signature = request.headers().get_one("X-Signature-Ed25519");

        // match (timestamp, signature) {
        //     (Some(_), Some(_)) => Outcome::Success(KeyGuard),
        //     _ => Outcome::Error((Status::Unauthorized, ()))
        // }
        Outcome::Success(KeyGuard)
    }
}
