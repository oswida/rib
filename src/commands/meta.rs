use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "Pong!").await?;
    msg.channel_id
        .send_message(ctx, |m| {
            m.flags(MessageFlags::EPHEMERAL);
            m.content("This is PONG")
        })
        .await
        .unwrap();
    Ok(())
}
