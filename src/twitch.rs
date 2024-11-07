
use dialoguer::{theme::ColorfulTheme, Password};
use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct UserData {
    pub id: String,
}

#[derive(Deserialize)]
pub struct User {
    pub data: Vec<UserData>,
}

impl User {
    pub async fn live_channels(
        username: &String,
        client_id: &String,
        token: &String,
    ) -> Result<Vec<Channel>, Box<dyn std::error::Error>> {
        let client = Client::new();
        let user = client
            .get(format!(
                "https://api.twitch.tv/helix/users?login={username}"
            ))
            .header("Authorization", format!("Bearer {token}"))
            .header("Client-id", client_id)
            .send()
            .await?
            .json::<Self>()
            .await?;
        let id = user.data[0].id.to_string();

        let live_channels = client
            .get(format!(
                "https://api.twitch.tv/helix/streams/followed?user_id={id}"
            ))
            .header("Authorization", format!("Bearer {token}"))
            .header("Client-id", client_id)
            .send()
            .await?
            .json::<Followed>()
            .await?
            .data;

        Ok(live_channels)
    }
}

#[derive(Deserialize)]
pub struct Channel {
    pub user_name: String,
    // pub game_name: String,
    // pub title: String,
    // pub viewer_count: u32,
    // pub started_at: String,
    // pub thumbnail_url: String,
}

#[derive(Deserialize)]
struct Followed {
    data: Vec<Channel>,
}

#[derive(Deserialize)]
pub struct TokenValidation {
    pub login: String,
    pub scopes: Vec<String>,
    pub expires_in: i32,
}

#[derive(Deserialize)]
pub struct Token {
    pub access_token: String,
}

impl Token {
    pub async fn validate(
        token: &String,
        username: &String,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let client = Client::new();
        let res = client
            .get("https://id.twitch.tv/oauth2/validate")
            .header("Authorization", format!("OAuth {token}"))
            .send()
            .await?;

        let status = res.status();
        if !status.is_success() {
            return Ok(false);
        }
        let res = res.json::<TokenValidation>().await?;

        // expires in a week or less
        if res.expires_in <= 604_800
            || !res.scopes.contains(&"user:read:follows".to_string())
            || !res.login.eq(username)
        {
            return Ok(false);
        }

        Ok(true)
    }

    pub fn generate(client_id: &String) -> Result<String, Box<dyn std::error::Error>> {
        let url = format!("https://id.twitch.tv/oauth2/authorize?response_type=token&client_id={client_id}&redirect_uri=https://twitchscopes.com/auth&scope=user%3Aread%3Afollows");
        let access_token = Password::with_theme(&ColorfulTheme::default())
            .with_prompt(format!("Access Token \x1b[0;3;90m{url}\x1b[0m"))
            .interact()?;

        Ok(access_token)
    }
<<<<<<< HEAD
}
=======
}
>>>>>>> f5d9c85 (initial commit)
