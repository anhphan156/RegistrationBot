use serde::{Serialize, Deserialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use crate::utils::snowflake::Snowflake;
use super::{embed::Embed, user::User};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Interaction {
    pub id: Snowflake,
    pub application_id: Snowflake,
    #[serde(rename = "type")]
    pub interaction_type: InteractionType,
    pub data: Option<InteractionData>,
    pub user: Option<User>,
    // #[serde(skip)]
    // pub guild: UnimplementedDS,
    // #[serde(skip)]
    // pub guild_id: Option<Snowflake<'r>>,
    // #[serde(skip)]
    // pub channel: UnimplementedDS,
    // #[serde(skip)]
    // pub channel_id: Option<Snowflake<'r>>,
    // #[serde(skip)]
    pub member: Option<Member>,
    // #[serde(skip)]
    // pub user: UnimplementedDS,
    pub token: Option<String>,
    // #[serde(skip)]
    // pub version: u8,
    // #[serde(skip)]
    pub message: Option<Message>,
    // #[serde(skip)]
    // pub app_permissions: Option<&'r str>,
    // #[serde(skip)]
    // pub locale: Option<&'r str>,
    // #[serde(skip)]
    // pub guild_locale: Option<&'r str>,
    // #[serde(skip)]
    // pub entitlements: UnimplementedDS, //Vec<&'r str>,
    // #[serde(skip)]
    // pub authorizing_integration_owners: UnimplementedDS, // AuthorizingIntegrationOwners,
    // #[serde(skip)]
    // pub context: UnimplementedDS,
}

// impl Default for Interaction<'_> {
//     fn default() -> Self {
//         Interaction {
//             id: "-1",
//             application_id: "-1",
//             interaction_type: InteractionType::PING,
//             guild_id: Some("-1"),
//             channel_id: Some("-1"),
//             token: "-1",
//             version: 0,
//             app_permissions: Some("-1"),
//             locale: Some("-1"),
//             guild_locale: Some("-1"),
//         }
//         // Interaction {
//         //     id: "-1",
//         //     application_id: "-1",
//         //     interaction_type: InteractionType::PING,
//         //     data: 0,
//         //     guild: 0,
//         //     guild_id: Some("-1"),
//         //     channel: 0,
//         //     channel_id: Some("-1"),
//         //     member: 0,
//         //     user: 0,
//         //     token: "-1",
//         //     version: 0,
//         //     message: 0,
//         //     app_permissions: Some("-1"),
//         //     locale: Some("-1"),
//         //     guild_locale: Some("-1"),
//         //     entitlements: 0,
//         //     authorizing_integration_owners: 0,
//         //     context: 0
//         // }
//     }
// }

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
    // id: Snowflake<'r>,
    pub name: Option<String>,
    // #[serde(rename = "type")]
    // r#type: u8,
    // resolved: Option<u8>, //
    // option: Option<Vec<u8>>, //
    // guild_id: Snowflake<'r>,
    // target_id: Snowflake<'r>,
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
