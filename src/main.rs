use std::fs::File;
use std::io::BufReader;

use handler::{SlashCommandContainer, SlashCommandHandler};
use serde::{Deserialize, Serialize};
use serenity::prelude::*;

use crate::commands::buki_roulette::command::BukiRoulette;
use crate::commands::main_power::command::MainPowerUp;
use crate::commands::schedule::command::Schedule;
use crate::commands::takashi::command::Takashi;

pub mod commands;
mod handler;

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = get_token("config.json").expect("Err トークンが見つかりません");
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILDS | GatewayIntents::GUILD_VOICE_STATES;

    // Create a new instance of the Client, logging in as a bot. This will
    // automatically prepend your bot token with "Bot ", which is a requirement
    // by Discord for bot users.
    let mut client = Client::builder(&token, intents)
        .event_handler(SlashCommandHandler::new(
            SlashCommandContainer::default()
                .add_command(MainPowerUp)
                .add_command(Schedule::default())
                .add_command(Takashi)
                .add_command(BukiRoulette),
        ))
        .await
        .expect("Err creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

#[derive(Serialize, Deserialize)]
struct Token {
    token: String,
}

fn get_token(file_name: &str) -> serde_json::Result<String> {
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);
    let t: Token = serde_json::from_reader(reader).unwrap();
    Ok(t.token)
}
