mod commands;

use std::{collections::HashSet, env};
use dotenv::dotenv;

use serenity::async_trait;
use serenity::framework::standard::{
    help_commands,
    macros::{group, help},
    Args, CommandGroup, CommandResult, HelpOptions,
};
use serenity::framework::StandardFramework;
use serenity::model::{channel::Message, gateway::Ready, id::UserId};
use serenity::prelude::{Client, Context, EventHandler};

use commands::{channels::*, neko::*};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // initializer
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[help] // help command
#[individual_command_tip = "help"]
#[strikethrough_commands_tip_in_guild = ""]
async fn my_help(
    ctx: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let _ = help_commands::with_embeds(ctx, msg, args, help_options, groups, owners).await;
    Ok(())
}

#[group]
#[commands(neko, all_channels)]
struct General;


#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var("SECRET_TOKEN").expect("cannot read expected token.");
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~")) // prefix
        .help(&MY_HELP) // add help command
        .group(&GENERAL_GROUP); // add general

    let mut client = Client::builder(&token)
        .event_handler(Handler) // add event handler
        .framework(framework) // add command
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
