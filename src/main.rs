use std::fs::File;
use std::io::BufReader;

use handler::Handler;
use serde::{Deserialize, Serialize};
use serenity::prelude::*;

pub mod commands;
mod handler;

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = get_token("config.json").expect("Err トークンが見つかりません");
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::empty();

    // Create a new instance of the Client, logging in as a bot. This will
    // automatically prepend your bot token with "Bot ", which is a requirement
    // by Discord for bot users.
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
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
