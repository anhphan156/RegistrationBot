use serde::{Deserialize, Serialize};
use crate::Snowflake;

#[derive(Deserialize, Serialize)]
pub struct Emoji<'r> {
    #[serde(borrow)]
    pub id: Option<Snowflake<'r>>,
    pub name: Option<&'r str>,
}
