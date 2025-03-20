use rocket::{http::Status,  Request};
use rocket::data::{Data, FromData, Outcome as DataOutcome, ToByteUnit};
use rocket::serde::json::from_str;
use ed25519_dalek::{Signature, VerifyingKey, PUBLIC_KEY_LENGTH, SIGNATURE_LENGTH};
use hex;
use crate::discord::interaction::Interaction;

#[derive(Debug)]
pub enum Error {
    BadHeader,
    BadBody,
    BadJson,
    BadSignature
}

pub struct RawBody {
    body_str: String,
}

impl<'r> RawBody {
    pub fn json(&'r self) -> Option<Interaction<'r>> {
        // match from_str::<Interaction<'r>>(&self.body_str) {
        match from_str::<Interaction>(&self.body_str) {
            Ok(parsed) => Some(parsed),
            Err(e) => {
                println!("Error: {}", e);
                println!("RawBody: {}", self.body_str);
                return None;
            }
        }
    }
}

#[rocket::async_trait]
impl<'r> FromData<'r> for RawBody {
    type Error = Error;

    async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> DataOutcome<'r, Self> {

        let timestamp : Option<&str> = req.headers().get_one("X-Signature-Timestamp");
        let signature : Option<&str> = req.headers().get_one("X-Signature-Ed25519");

        let (timestamp, signature) : (&str, &str) = match (timestamp, signature) {
            (Some(t), Some(s)) => (t,s),
            _ => return DataOutcome::Error((Status::Unauthorized, Error::BadHeader))
        };

        let raw_interaction = RawBody {
            body_str: match data.open(10.mebibytes()).into_string().await {
                Ok(bytes) => bytes.into_inner(),
                Err(_e) => return DataOutcome::Error((Status::BadRequest, Error::BadBody))
            },
        };

        let message : &str = &format!("{}{}", timestamp, raw_interaction.body_str);

        if verify_key(message, signature) {
            rocket::outcome::Outcome::Success(raw_interaction)
        }else {
            rocket::outcome::Outcome::Error((Status::Unauthorized, Error::BadSignature))
        }
    }
}

fn verify_key(message: &str, signature: &str)-> bool {
    let public_key = match std::env::var("PUBLIC_KEY") {
        Ok(key) => key,
        Err(_) => return false
    };
    let public_key : [u8; PUBLIC_KEY_LENGTH] = hex::decode(public_key)
        .expect("Failed to decode public key")
        .try_into()
        .expect("Invalid public key byte length");

    if let Ok(public_key) = VerifyingKey::from_bytes(&public_key) {
        let signature : [u8; SIGNATURE_LENGTH] = hex::decode(signature)
            .expect("Failed to decode signature")
            .try_into()
            .expect("Invalid signature byte length");
        let signature = Signature::from_bytes(&signature);

        match public_key.verify_strict(message.as_bytes(), &signature) {
            Ok(_) => return true,
            Err(e) => {
                println!("Error: {}",e); 
                return false;
            }
        }
    }

    false
}
