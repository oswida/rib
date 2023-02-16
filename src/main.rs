mod bot;
mod commands;
mod slash;

use std::collections::HashSet;
use std::env;

use bot::db::{close_database, init_database};
use bot::handler::{Handler, ShardManagerContainer, GENERAL_GROUP};
use serenity::framework::StandardFramework;
use serenity::http::Http;
use serenity::prelude::GatewayIntents;
use serenity::Client;
use tracing::error;

#[tokio::main]
async fn main() {
    // This will load the environment variables located at `./.env`, relative to
    // the CWD. See `./.env.example` for an example on how to structure this.
    dotenv::dotenv().expect("Failed to load .env file");

    tracing_subscriber::fmt::init();

    let token = env::var("DC_TOKEN").expect("Expected a token in the environment");
    let prefix = env::var("DC_PREFIX").expect("Expected a bot prefix in the environment");
    let http = Http::new(&token);

    // We will fetch your bot's owners and id
    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);
            (owners, info.id)
        }
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    // Create the framework
    let framework = StandardFramework::new()
        .configure(|c| c.owners(owners).prefix(prefix))
        .group(&GENERAL_GROUP);

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(&token, intents)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
    }

    let shard_manager = client.shard_manager.clone();
    let database = init_database();

    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Could not register ctrl+c handler");
        shard_manager.lock().await.shutdown_all().await;
        close_database(database);
    });

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}
