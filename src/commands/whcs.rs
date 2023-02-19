use caith::Roller;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::http::CacheHttp;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;

#[command("whcs")]
#[description("Warhammer Fantasy Roleplay 4ed charsheets mgmt")]
#[sub_commands(whcs_create)]
async fn whcs(ctx: &Context, msg: &Message) -> CommandResult {
    // let result = Roller::new("d10").unwrap().roll().unwrap();
    // let username = msg.author_nick(ctx.http()).await.unwrap();
    // let response = MessageBuilder::new()
    //     .push_bold_safe(username)
    //     .push_italic_safe(" d10 ")
    //     .push(" 🎲 ")
    //     .push(result.to_string().replace("`", ""))
    //     .build();
    // msg.channel_id.say(&ctx.http, response).await?;
    msg.channel_id.say(&ctx.http, "WHCS").await?;
    Ok(())
}

#[command("create")]
#[description("Create new charsheet")]
#[num_args(1)]
#[help_available]
async fn whcs_create(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let name = args.rest();
    let response = MessageBuilder::new().push("creating ").push(name).build();
    msg.channel_id.say(&ctx.http, response).await?;
    Ok(())
}
