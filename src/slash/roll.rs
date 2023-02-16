use std::fmt::Display;

use caith::Roller;
use serenity::builder::CreateApplicationCommand;

use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use serenity::utils::MessageBuilder;

pub fn run(options: &[CommandDataOption]) -> String {
    let notation = options
        .get(0)
        .expect("Dice notation is required")
        .value
        .as_ref()
        .unwrap()
        .as_str()
        .unwrap();

    let result = Roller::new(notation).unwrap().roll().unwrap();
    MessageBuilder::new()
        .push_italic_safe(notation)
        .push(" 🎲 ")
        .push(result.to_string().replace("`", ""))
        .build()
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("roll")
        .description("Dice roll command")
        .create_option(|opt| {
            opt.name("notation")
                .description("dice notation like 3d6, 4dF")
                .kind(CommandOptionType::String)
                .required(true)
        })
}
