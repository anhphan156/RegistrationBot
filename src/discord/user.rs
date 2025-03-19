use rocket::serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize,Serialize)]
#[serde(crate = "rocket::serde")]
pub struct User<'r> {
    id: &'r str,
    username: &'r str,
    discriminator: &'r str,
    global_name: &'r str,
    avatar: &'r str,
    bot: bool,
    system: bool,
    avatar_decoration_data: Option<&'r str>,
    clan: Option<bool>,
    collectibles: Option<bool>,
    primary_guild: Option<&'r str>,
    public_flags: u8,
}
