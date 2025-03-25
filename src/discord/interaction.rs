use rocket::{data::{FromData, Outcome as DataOutcome, ToByteUnit}, http::Status, serde::json::from_str, Data, Request};
use serde::{Serialize, Deserialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use crate::utils::{crypto::Crypto, snowflake::Snowflake};
use super::{embed::Embed, user::User};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Interaction {
    pub id: Snowflake,
    pub application_id: Snowflake,
    #[serde(rename = "type")]
    pub interaction_type: InteractionType,
    pub data: Option<InteractionData>,
    pub user: Option<User>,
    pub member: Option<Member>,
    pub token: Option<String>,
    pub message: Option<Message>,
}

#[derive(Debug)]
pub enum Error {
    BadHeader,
    BadBody,
    BadJson,
    BadSignature
}

#[rocket::async_trait]
impl<'r> FromData<'r> for Interaction {
    type Error = Error;

    async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> DataOutcome<'r, Self> {

        let timestamp : Option<&str> = req.headers().get_one("X-Signature-Timestamp");
        let signature : Option<&str> = req.headers().get_one("X-Signature-Ed25519");

        let (timestamp, signature) : (&str, &str) = match (timestamp, signature) {
            (Some(t), Some(s)) => (t,s),
            _ => return DataOutcome::Error((Status::Unauthorized, Error::BadHeader))
        };

        let body_str = match data.open(10.mebibytes()).into_string().await {
            Ok(bytes) => bytes.into_inner(),
            Err(_e) => return DataOutcome::Error((Status::BadRequest, Error::BadBody))
        };

        let message : &str = &format!("{}{}", timestamp, body_str);

        if Crypto::verify_key(message, signature) {
            rocket::outcome::Outcome::Success(from_str(&body_str).expect(""))
        }else {
            rocket::outcome::Outcome::Error((Status::Unauthorized, Error::BadSignature))
        }
    }
}

#[repr(u8)]
#[derive(Deserialize_repr, Serialize_repr, PartialEq, Eq, Debug, Clone, Copy)]
pub enum InteractionType {
    PING = 1,
    APPLICATIONCOMMAND = 2,
    MESSAGECOMPONENT = 3,
    APPLICATIONCOMMANDAUTOCOMPLE = 4,
    MODALSUBMIT = 5,
}

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct InteractionData {
    pub name: Option<String>,
    pub custom_id: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct Message {
    pub id: Option<String>,
    pub embeds: Option<Vec<Embed>>,
    #[serde(rename="interaction")]
    pub parent_interaction: Option<InteractionMetadata>,
}

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct Member {
    pub nick: Option<String>,
    pub user: Option<User>,
}

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct InteractionMetadata {
    pub id: Option<String>,
    pub user: Option<User>,
}
