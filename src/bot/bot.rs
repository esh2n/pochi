use std::collections::HashSet;

use serenity::async_trait;
use serenity::framework::standard::{
    help_commands,
    macros::{group, help},
    Args, CommandGroup, CommandResult, HelpOptions,
};
use serenity::framework::StandardFramework;
use serenity::model::id::{ChannelId, MessageId, GuildId};
use serenity::model::prelude::Activity;
use serenity::model::{channel::Message, gateway::Ready, id::UserId};
use serenity::prelude::{Client, Context, EventHandler};
use serenity::http::Http;

use crate::commands::{channels::*, neko::*, bo::*};
use crate::server::server::launch;
struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // initializer
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        ctx.set_activity(Activity::playing("^help")).await;
        launch().await;
    }

    async fn message_delete(
        &self,
        ctx: Context,
        channel_id: ChannelId,
        deleted_message_id: MessageId,
        _: Option<GuildId>) {
            let _ = channel_id
            .say(&ctx.http, format!("🤭 {}に作成されたメッセージは何者かによって削除されました。", deleted_message_id.created_at()))
            .await;
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
#[commands(neko, all_channels, bo)]
struct General;

#[async_trait]
pub trait DiscordBot {
    async fn init(&self, token: &str);
}

pub struct DiscordService;

#[async_trait]

impl DiscordBot for DiscordService {
    async fn init(&self, token: &str) {
        let http = Http::new_with_token(token);
        let bot_id = match http.get_current_application_info().await {
            Ok(info) => info.id,
            Err(why) => panic!("Could not access application info: {:?}", why),
        };
        let framework = StandardFramework::new()
            .configure(|c| c.with_whitespace(true).on_mention(Some(bot_id)).prefix("^")) // prefix
            .bucket("basic", |b| b.delay(2).time_span(10).limit(3)).await // delay 2 seconds, 3 req / 10 sec
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
}