use std::{
    collections::HashMap,
    path::PathBuf,
    process::exit,
    sync::{Arc, LazyLock, Mutex},
};

use config::Config;
use serenity::all::GatewayIntents;

pub mod audit;
pub mod commands;
pub mod config;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    ensure_env_vars();

    let client = serenity::Client::builder(
        get_discord_token(),
        GatewayIntents::GUILD_MESSAGES
            | GatewayIntents::GUILD_MESSAGE_REACTIONS
            | GatewayIntents::GUILD_MEMBERS
            | GatewayIntents::MESSAGE_CONTENT,
    )
    .event_handler(commands::CommandHandler)
    .event_handler(audit::AuditHandler)
    .await;
    if client.is_err() {
        println!("Failed to create client: {:?}", client.err().unwrap());
        exit(1);
    }
    let mut client = client.unwrap();

    let invite_url = format!(
        "https://discord.com/api/oauth2/authorize?client_id={}&permissions=8&scope=bot%20applications.commands",
        client.http.get_current_application_info().await.unwrap().id
    );

    println!("Invite URL: {}", invite_url);
    println!("Starting client...");
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
    println!("Client closed.");
}

const DISCORD_TOKEN: &str = "DISCORD_TOKEN";
const GUILD_ID: &str = "GUILD_ID";

pub fn ensure_env_vars() {
    if std::env::var(DISCORD_TOKEN).is_err() {
        println!("{} is not set", DISCORD_TOKEN);
        exit(1);
    }
    if std::env::var(GUILD_ID).is_err() {
        println!("{} is not set", GUILD_ID);
        exit(1);
    }
    if std::env::var("CONFIG_PATH").is_err() {
        println!("CONFIG_PATH is not set");
        exit(1);
    }
}

pub fn get_discord_token() -> String {
    std::env::var(DISCORD_TOKEN).unwrap()
}

pub fn get_guild_id() -> u64 {
    std::env::var(GUILD_ID).unwrap().parse().unwrap()
}

pub fn get_config(guild: u64) -> Arc<Config> {
    static CONFIGS: LazyLock<Mutex<HashMap<u64, Arc<Config>>>> =
        LazyLock::new(|| Mutex::new(HashMap::new()));

    let mut configs = CONFIGS.lock().expect("failed to lock CONFIGS");
    configs.entry(guild).or_insert_with(|| {
        let path = PathBuf::from(std::env::var("CONFIG_PATH").unwrap());
        let path = path.join(format!("{}.json", guild));
        if !path.exists() {
            let config = Config::new(guild);
            config.save();
        }
        let data = std::fs::read_to_string(path).unwrap();
        Arc::new(serde_json::from_str(&data).unwrap())
    });

    configs.get(&guild).unwrap().clone()
}
