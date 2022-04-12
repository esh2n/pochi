use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use rand::Rng;


#[command]
#[description = "募集"]
async fn bo(ctx: &Context, msg: &Message) -> CommandResult {
    let channel = ctx.http.get_channel(*msg.channel_id.as_u64()).await?;
    let sg = SunfishGenerator::new();
    let sunfish = sg.repl();
    msg.channel_id
        .say(&ctx.http, format!("
@here
{}さんが{}で募集しています。
{}
", msg.author.mention(), channel.mention(), sunfish))
        .await?;
    Ok(())
}

struct SunfishGenerator {
}

impl SunfishGenerator {

    fn new() -> Self {
        SunfishGenerator{}
    }

    fn repl(&self) -> String {
        let sunfishes  = [
".　　　　　　　　　　　　/^i
.　　　　　　　　　　　  /:::::|
.　　　　　　　　　 ＿＿/::::::::|
.　 　 　 　 ,. ‐’ ´::::::::::::::::::::::::::ヽ:.､
.　　　, ‐’´:::::::::::::::::::::::::::::::::::::::::::ヽ:＼
.　　（:::(ｏ):::::::／i:::::::::::::::::::::::::::::::::i::::::i
.　　 ヽ　　　　￣ :::::::::::::::::::::::::::::|::::::l
.　　　 ＼　　　　　   ::::::::::::::::::ｉ::::::i
.　　　　　`‐ ､　　　　　　 ::::::/::／
.　　　 　 　 　 ｀ ー– ､…….::／ ‘´
.　　　　　　　　 　 　i:::::::|
.　 　 　 　 　 　 　 　  i:::::::!
.　　　　　　　　　　　ヽ:_|".to_string(),
".          　　 _
.          　　｜ヽ＿＿
.          　 _/　　　 ＼
.          　(ﾐ 　　 ◎ -ヽ
.     　     (ﾐ　(三 (ﾟдﾟ)
.     　     (ﾐ 　　(ﾉ　 /)
.     　      ~＼　＿＿／
.     　　     　L/∪∪".to_string(),
".               ／ﾍﾉ＼
.             /  ＠      |
.             |  ¨ ¨   |
.           ／__ﾆﾆﾆﾆ_ ＼".to_string(),
"う〜　　　　　  　∧
マンボウ！　　　 /  |
. 　　　　　　　/　 |
.         　.-･” ”￣ ￣　　　ヽ
.     ／　　　　　　　　三\\
.     ₎　o　/三　　　　 　三|
.     ヽ　　￣　　　　　 三/
.         “･- ., _　　　　　ノ
.              　    　￣￣\\　 |
. 　　　　　　　   \\　|
.　　　　　　　　  V".to_string(),
        ];
        let mut rng = rand::thread_rng();
        let i = rng.gen_range(0, sunfishes.len());
        sunfishes[i].to_owned()
    }
}