use std::path::Path;

use caith::Roller;
use pdf_form::Form;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::http::CacheHttp;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;

use crate::common::assets::copy_asset;
use crate::common::dir::{app_file_exists, create_app_dir, get_app_path, write_to_file};
use crate::rpg::wfrp::WfrpChar;

#[command("whcs")]
#[description("Warhammer Fantasy Roleplay 4ed charsheets mgmt")]
#[sub_commands(whcs_create, whcs_show)]
async fn whcs(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "WHCS").await?;
    Ok(())
}

#[command("create")]
#[description("Create new charsheet")]
#[num_args(1)]
#[help_available]
async fn whcs_create(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let name = args.rest();
    let id = msg.author.id.to_string();
    create_app_dir(&format!("cs/{}", id));
    let cs_name = &format!("cs/{}/{}.yaml", id, name);
    let cs_exists = app_file_exists(cs_name);
    if cs_exists {
        msg.channel_id
            .say(&ctx.http, &format!("Charsheet for {} already exists", name))
            .await?;
        return Ok(());
    }

    let c = WfrpChar::new(name, &msg.author.name);
    let cresp = c.print();
    //copy_asset("wfrp/karta_pl.pdf", cs_name);

    // let response = MessageBuilder::new()
    //     .push("Charsheet for ")
    //     .push_bold(name)
    //     .push(" created.\n")
    //     .push(cresp)
    //     .build();

    let response = msg
        .channel_id
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
        .await;

    Ok(())
}

#[command("show")]
#[description("Show charsheet")]
#[num_args(1)]
#[help_available]
async fn whcs_show(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let name = args.rest();
    let id = msg.author.id.to_string();
    create_app_dir(&format!("./cs/{}", id));
    let cs_name = &format!("./cs/{}/{}.pdf", id, name);
    let cs_exists = app_file_exists(cs_name);
    if !cs_exists {
        msg.channel_id
            .say(&ctx.http, &format!("Charsheet for {} does not exist", name))
            .await?;
        return Ok(());
    }

    println!("Loading {}", cs_name);
    let pth = get_app_path(cs_name);
    let form_result = Form::load("/home/oskar/Coding/rib/cs/690261275410038804/ala.pdf");
    match form_result {
        Ok(form) => {
            // Get all types of the form fields (e.g. Text, Radio, etc) in a Vector
            let field_types = form.get_all_types();
            // Print the types
            for tp in field_types {
                println!("{:?}", tp);
            }
        }
        Err(e) => println!("{}", e.to_string()),
    }

    let response = MessageBuilder::new()
        .push("Charsheet for ")
        .push_bold(name)
        .push(" shown.")
        .build();
    msg.channel_id.say(&ctx.http, response).await?;
    Ok(())
}
