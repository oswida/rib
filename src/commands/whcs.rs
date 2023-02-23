use crate::common::assets::get_asset_string;
use crate::common::dir::{
    app_file_exists, create_app_dir, delete_file, read_from_file, write_to_file,
};
use crate::common::discord::text_button;
use crate::rpg::wfrp::{wfrp_find_stat_pl, WfrpChar, WfrpCoreStat};
use serde_json::json;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::http::CacheHttp;
use serenity::model::application::interaction::InteractionResponseType;
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::str::FromStr;
use std::time::Duration;

pub fn charsheet_exists(name: String, id: String) -> (bool, String) {
    create_app_dir(&format!("cs/{}", id)).expect("Cannot create user dir");
    let cs_name = &format!("cs/{}/{}.json", id, name);
    (app_file_exists(cs_name), cs_name.to_string())
}

#[command("whcs")]
#[description("Warhammer Fantasy Roleplay 4ed charsheets mgmt")]
#[sub_commands(whcs_create, whcs_show, whcs_set, whcs_del)]
async fn whcs(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "WHCS").await?;
    Ok(())
}

#[command("create")]
#[aliases("cr")]
#[description("Create new charsheet")]
#[num_args(1)]
#[help_available]
async fn whcs_create(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let name = args.rest();
    let id = msg.author.id.to_string();
    let (cs_exists, cs_name) = charsheet_exists(name.to_string(), id);
    if cs_exists {
        msg.channel_id
            .say(&ctx.http, &format!("Charsheet for {} already exists", name))
            .await?;
        return Ok(());
    }

    let c = WfrpChar::new(name, &msg.author.name);
    let tpl = get_asset_string("wfrp/cs_pl.handlebars");
    write_to_file(&cs_name, &json!(c).to_string()).expect("Cannot save charsheet to file");

    let cresp = c.print(&tpl);

    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.content(cresp).flags(MessageFlags::EPHEMERAL)
            // .embed(|e| {
            //     e.title("This is a title")
            //         .description("This is a description")
            //         .image("attachment://ferris_eyes.png")
            //         .fields(vec![
            //             ("This is the first field", "This is a field body", true),
            //             ("This is the second field", "Both fields are inline", true),
            //         ])
            //         .field(
            //             "This is the third field",
            //             "This is not an inline field",
            //             false,
            //         )
            //         .footer(|f| f.text("This is a footer"))
            //         // Add a timestamp for the current time
            //         // This also accepts a rfc3339 Timestamp
            //         .timestamp(Timestamp::now())
            // })
            // .add_file("./ferris_eyes.png")
        })
        .await
        .unwrap();

    Ok(())
}

#[command("show")]
#[aliases("sh")]
#[description("Show charsheet")]
#[num_args(1)]
#[help_available]
async fn whcs_show(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let name = args.rest();
    let id = msg.author.id.to_string();
    let (cs_exists, cs_name) = charsheet_exists(name.to_string(), id);
    if !cs_exists {
        msg.channel_id
            .say(&ctx.http, &format!("Charsheet for {} does not exist", name))
            .await?;
        return Ok(());
    }

    let data = read_from_file(&cs_name).expect("Cannot read from file");
    let v: WfrpChar = serde_json::from_str(&data).expect("Cannot deserialize charsheet");
    let tpl = get_asset_string("wfrp/cs_pl.handlebars");
    let cresp = v.print(&tpl);

    let _response = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.content(cresp).flags(MessageFlags::EPHEMERAL)
        })
        .await;
    Ok(())
}

#[command("set")]
#[description("Set any value in charsheet")]
#[usage(
    r#"!whcs set <character name> <trait> <name> <value1> <value2>,
where:
    <character name>  name of charsheet
    <trait> can be "stat", "skill","talent", "xp", "hero", "luck"
    <name>  trait name
    <value1>, <value2>  values for a specific trait"#
)]
#[help_available]
async fn whcs_set(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let name = args.parse::<String>().expect("Charsheet name is required");
    args.advance();
    let id = msg.author.id.to_string();
    let (cs_exists, cs_name) = charsheet_exists(name.to_string(), id);
    if !cs_exists {
        msg.channel_id
            .say(&ctx.http, &format!("Charsheet for {} does not exist", name))
            .await?;
        return Ok(());
    }
    let trt = args.parse::<String>().expect("Trait type is required");
    args.advance();
    let trt_name = args.parse::<String>().expect("Trait name is required");
    args.advance();
    let mut trt_values = args.rest().split_whitespace();

    let data = read_from_file(&cs_name).expect("Cannot read from file");
    let mut v: WfrpChar = serde_json::from_str(&data).expect("Cannot deserialize charsheet");

    match trt.as_str() {
        "stat" => {
            let key = wfrp_find_stat_pl(trt_name.as_str());
            let rec: &mut WfrpCoreStat = v.stats.get_mut(&key).unwrap();
            let tt = trt_values.next().expect("Stat parameter required");
            let tv = trt_values.next().expect("Stat value required");
            match tt {
                "base" => {
                    rec.base = u8::from_str(tv).expect("Bad stat base name");
                }
                _ => {
                    println!("unknown stat {} {} {}", key, tt, tv,)
                }
            }
        }
        _ => {
            println!("unknown trait {}", trt)
        }
    }

    write_to_file(&cs_name, &json!(v).to_string()).expect("Cannot write to file");

    let tpl = get_asset_string("wfrp/cs_pl.handlebars");
    let cresp = v.print(&tpl);
    let _response = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.content(cresp).flags(MessageFlags::EPHEMERAL)
        })
        .await;

    Ok(())
}

#[command("del")]
#[aliases("d")]
#[description("Delete charsheet")]
#[num_args(1)]
#[help_available]
async fn whcs_del(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let name = args.rest();
    let id = msg.author.id.to_string();
    let (cs_exists, cs_name) = charsheet_exists(name.to_string(), id);
    if !cs_exists {
        msg.channel_id
            .say(&ctx.http, &format!("Charsheet for {} does not exist", name))
            .await?;
        return Ok(());
    }

    let response = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.content(format!("Deleting charsheet for {}. Please confirm.", name))
                .flags(MessageFlags::EPHEMERAL)
                .components(|c| {
                    c.create_action_row(|r| {
                        r.add_button(text_button("Cancel"));
                        r.add_button(text_button("Delete"))
                    })
                })
        })
        .await
        .unwrap();

    let interaction = match response
        .await_component_interaction(&ctx)
        .timeout(Duration::from_secs(30))
        .await
    {
        Some(x) => x,
        None => {
            response.reply(&ctx, "Timed out").await.unwrap();
            response.delete(&ctx.http()).await.unwrap();
            return Ok(());
        }
    };

    if let "Delete" = interaction.data.custom_id.as_str() {
        delete_file(cs_name.as_str()).expect("Cannot delete charsheet");
        interaction
            .create_interaction_response(&ctx.http, |r| {
                r.kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|d| d.ephemeral(true).content("Charsheet deleted"))
            })
            .await
            .unwrap();
    } else {
        interaction
            .create_interaction_response(&ctx.http, |r| {
                r.kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|d| d.ephemeral(true).content("Operation cancelled"))
            })
            .await
            .unwrap();
    }
    response.delete(&ctx.http()).await.unwrap();
    Ok(())
}
