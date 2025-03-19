use serde::{Serialize, Deserialize};
use super::{user::User, aio::AuthorizingIntegrationOwners};

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Interaction<'r> {
    pub id: &'r str,
    pub app_permissions: &'r str,
    pub application_id: &'r str,
    pub authorizing_integration_owners: AuthorizingIntegrationOwners,
    pub entitlements: Vec<&'r str>,
    pub token: &'r str,
    #[serde(rename = "type")]
    pub interaction_type: u8,
    pub user: User<'r>,
    pub version: u8
}
