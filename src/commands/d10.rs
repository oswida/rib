use caith::Roller;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::http::CacheHttp;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;

#[command]
#[aliases("dt")]
async fn d10(ctx: &Context, msg: &Message) -> CommandResult {
    let result = Roller::new("d10").unwrap().roll().unwrap();
    let username = msg.author_nick(ctx.http()).await.unwrap();
    let response = MessageBuilder::new()
        .push_bold_safe(username)
        .push_italic_safe(" d10 ")
        .push(" 🎲 ")
        .push(result.to_string().replace("`", ""))
        .build();
    msg.channel_id.say(&ctx.http, response).await?;

    Ok(())
}
