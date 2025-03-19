use rocket::serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct AuthorizingIntegrationOwners {

}
