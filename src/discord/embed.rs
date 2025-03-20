use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Embed<'r> {
    pub title: Option<&'r str>,
    #[serde(rename = "type")]
    pub embed_type: Option<&'r str>,
    pub description: Option<&'r str>,
    pub url: Option<&'r str>,
    pub color: Option<u32>,
}

impl<'r> Default for Embed<'_> {
    fn default() -> Self {
        let lorem = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";
        Embed {
            title: Some("Embed Title"),
            embed_type: Some("rich"),
            description: Some(lorem),
            url: None,
            color: Some(15606357),
        }
    }
}
