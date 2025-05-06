use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use super::role::Role;

#[derive(Builder, Deserialize, Serialize, Default)]
pub struct EventData {
    #[builder(default = "0")]
    event_time: i64,

    #[builder(default = "vec![]")]
    event_roles: Vec<Role>,
}

impl EventData {
    pub fn get_time(&self) -> i64 {
        self.event_time
    }

    pub fn set_roles(&mut self, roles: &[Role]) {
        self.event_roles = roles.to_vec();
    }

    pub fn get_roles(&self) -> &Vec<Role> {
        self.event_roles.as_ref()
    }

    pub fn get_roles_mut(&mut self) -> &mut Vec<Role> {
        self.event_roles.as_mut()
    }
}
