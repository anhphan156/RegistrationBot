use rocket::{async_trait, http::Status,  Request};
use rocket::data::{Data, FromData, Outcome as DataOutcome, ToByteUnit};
use rocket::serde::json::{from_slice, Json, from_str, from_value, json};
use serde::{Serialize, Deserialize};
use super::{user::User, aio::AuthorizingIntegrationOwners};
use ed25519_dalek::{Signature, VerifyingKey, PUBLIC_KEY_LENGTH, SIGNATURE_LENGTH};
use hex;

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Interaction<'r> {
    pub id: &'r str,
//     pub app_permissions: &'r str,
//     pub application_id: &'r str,
//     pub authorizing_integration_owners: AuthorizingIntegrationOwners,
//     pub entitlements: Vec<&'r str>,
//     pub token: &'r str,
    #[serde(rename = "type")]
    pub interaction_type: u8,
    // pub user: User<'r>,
    // pub version: u8
}

pub struct ValidatedJson<T>(pub T);

#[derive(Debug)]
pub enum Error {
    BadHeader,
    BadBody,
    BadJson,
    BadSignature
}

#[async_trait]
impl<'r, T: Deserialize<'r> + Serialize> FromData<'r> for ValidatedJson<T> {
    type Error = Error;

    async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> DataOutcome<'r, Self> {
        let timestamp : Option<&str> = req.headers().get_one("X-Signature-Timestamp");
        let signature : Option<&str> = req.headers().get_one("X-Signature-Ed25519");

        let (timestamp, signature) : (&str, &str) = match (timestamp, signature) {
            (Some(t), Some(s)) => (t,s),
            _ => return DataOutcome::Error((Status::Unauthorized, Error::BadHeader))
        };

        let body = match data.open(10.mebibytes()).into_string().await {
            Ok(bytes) => bytes.into_inner(),
            Err(_e) => return DataOutcome::Error((Status::BadRequest, Error::BadBody))
        };

        let json: T = match from_str::<T>("{\"type\": 1, \"id\": \"1\"}") {
            Ok(parsed) => parsed,
            Err(_) => return DataOutcome::Error((Status::BadRequest, Error::BadJson)),
        };

        let message : &str = &format!("{}{}", timestamp , body);

        // let json = from_slice::<T>(b"{\"type\": 1, \"id\": \"1\"}").expect("");
        // let json = from_value(json!(body)).expect("Failed to parse JSON");


        // let json = match Json::<T>::from_data(req, Data::from_data(req, a)).await {
        //     DataOutcome::Success(json) => json,
        //     DataOutcome::Error(_) => return DataOutcome::Error((Status::BadRequest, Error::BADJSON)),
        //     DataOutcome::Forward(f) => return DataOutcome::Forward(f)
        // };

        // let raw_data = rocket::serde::json::to_string(&json.0).unwrap() + timestamp;
        // println!("{}", raw_data);

        if verify_key(message, signature) {
        // if true {
            rocket::outcome::Outcome::Success(ValidatedJson(json))
            // rocket::outcome::Outcome::Error((Status::NotFound, Error::BADJSON))
        }else {
            rocket::outcome::Outcome::Error((Status::NotFound, Error::BadSignature))
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
