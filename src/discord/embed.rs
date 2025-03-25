use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Embed {
    title: Option<String>,
    #[serde(rename = "type")]
    embed_type: Option<String>,
    description: Option<String>,
    url: Option<String>,
    color: Option<u32>,
    image: Option<EmbedImage>,
    thumbnail: Option<EmbedImage>,
    fields: Option<Vec<EmbedField>>,
}

impl Embed {
    pub fn new() -> EmbedBuilder {
        EmbedBuilder {
            title: None,
            embed_type: None,
            description: None,
            url: None,
            color: Some(15606357),
            image: None,
            thumbnail: None,
            fields: None,
        }
    }
}

pub struct EmbedBuilder {
    title: Option<String>,
    embed_type: Option<String>,
    description: Option<String>,
    url: Option<String>,
    color: Option<u32>,
    image: Option<EmbedImage>,
    thumbnail: Option<EmbedImage>,
    fields: Option<Vec<EmbedField>>,
}

impl EmbedBuilder {
    pub fn title(&mut self, title: impl Into<String>) -> &mut Self {
        self.title = Some(title.into());
        self
    }
    pub fn embed_type(&mut self, embed_type: impl Into<String>) -> &mut Self {
        self.embed_type = Some(embed_type.into());
        self
    }
    pub fn description(&mut self, description: impl Into<String>) -> &mut Self {
        self.description = Some(description.into());
        self
    }
    pub fn url(&mut self, url: String) -> &mut Self {
        self.url = Some(url);
        self
    }
    pub fn color(&mut self, color: u32) -> &mut Self {
        self.color = Some(color);
        self
    }
    pub fn image(&mut self, image: EmbedImage) -> &mut Self {
        self.image = Some(image);
        self
    }
    pub fn thumbnail(&mut self, thumbnail: EmbedImage) -> &mut Self {
        self.thumbnail = Some(thumbnail);
        self
    }
    pub fn fields(&mut self, fields: Vec<EmbedField>) -> &mut Self {
        self.fields = Some(fields);
        self
    }

    pub fn build(&self) -> Embed {
        Embed {
            title: self.title.clone(),
            embed_type: self.embed_type.clone(),
            description: self.description.clone(),
            url: self.url.clone(),
            color: self.color,
            image: self.image.clone(),
            thumbnail: self.thumbnail.clone(),
            fields: self.fields.clone()
        }
    }
}

impl Default for Embed {
    fn default() -> Self {
        let lorem = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.".to_string();
        Embed {
            title: Some(String::from("Embed Title")),
            embed_type: Some(String::from("rich")),
            description: Some(lorem),
            url: None,
            color: Some(15606357),
            image: None,
            thumbnail: None,
            fields: None,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct EmbedImage {
    url: String,
}

impl EmbedImage {
    pub fn new(url: impl Into<String>) -> Self {
        EmbedImage { url: url.into() }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct EmbedField {
    name: String,
    value: String,
    inline: bool,
}

impl EmbedField {
    pub fn new(name: impl Into<String>, value: impl Into<String>, inline: bool) -> Self {
        EmbedField { 
            name: name.into(),
            value: value.into(),
            inline,
        }
    }
}
