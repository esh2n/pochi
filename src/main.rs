mod commands;
mod server;
mod bot;

use std::{env};
use dotenv::dotenv;
use bot::bot::*;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var("SECRET_TOKEN").expect("cannot read expected token.");
    let bot = DiscordService {};
    bot.init(&token).await;
}