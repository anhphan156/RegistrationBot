use rocket::{async_trait, data::{self, FromData}, http::Status, request::{FromRequest, Outcome}, serde::Deserialize, Data, Request};

use super::{user::User, aio::AuthorizingIntegrationOwners};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Interaction<'r> {
    pub id: &'r str,
    pub app_permissions: &'r str,
    pub application_id: &'r str,
    pub authorizing_integration_owners: AuthorizingIntegrationOwners,
    pub entitlements: Vec<&'r str>,
    pub token: &'r str,
    #[serde(rename = "type")]
    pub r#type: u8,
    pub user: User<'r>,
    pub version: u8
}

#[derive(Debug)]
pub enum Error {
    A
}

#[async_trait]
impl<'r> FromData<'r> for Interaction<'r> {
    type Error = Error;

    async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> data::Outcome<'r, Self> {
        println!("asdfdjfklA");
        rocket::outcome::Outcome::Error((Status::NotFound, Error::A))
    }
}
