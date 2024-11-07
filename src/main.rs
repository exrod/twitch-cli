use config::Config;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use std::{process::Command, thread};
use twitch::{Token, User};

mod config;
mod twitch;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if !confy::get_configuration_file_path("squish", "squish")?.exists() {
        let username = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Username")
            .interact_text()?;
        let client_id = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Client ID")
            .interact_text()?;
        let access_token = Token::generate(&client_id)?;

        confy::store(
            "squish",
            "squish",
            Config {
                username,
                client_id,
                access_token,
            },
        )?;
    };

    let config = confy::load::<Config>("squish", "squish")?;
    if !Token::validate(&config.access_token, &config.username).await? {
        confy::store(
            "squish",
            "squish",
            Config {
                access_token: Token::generate(&config.client_id)?,
                ..config.clone()
            },
        )?;
    }

    let channels = User::live_channels(&config.username, &config.client_id, &config.access_token)
        .await?
        .into_iter()
        .map(|channel| channel.user_name)
        .collect::<Vec<String>>();

    let idx = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Pick a channel")
        .items(&channels)
        .interact()?;
    let channel = channels[idx].clone();
    let channel_url = format!("https://twitch.tv/{channel}");

    let mpv = thread::spawn(move || {
        Command::new("mpv").arg(channel_url).output().unwrap();
    });

    let chatterino = thread::spawn(move || {
        Command::new("chatterino")
            .args(["-c", &channel])
            .output()
            .unwrap();
    });

    mpv.join().unwrap();
    chatterino.join().unwrap();

    Ok(())
}