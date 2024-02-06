mod apis;
mod commands;
mod data;
mod event;
mod utils;

use std::{env, error};
use tokio::sync::Mutex;
use poise::serenity_prelude as serenity;

extern crate log;
extern crate pretty_env_logger;

use crate::data::Data;
use crate::event::event_handler;
use crate::utils::db::get_pool;
use crate::commands::{
    exam,
    general,
    music::{
        spotify,
    },
    game::{
        starrail,
        dokkan
    },
    meonly,
};



#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    if let Err(why) = dotenv::dotenv() {
        log::error!("Unable to find .env file: {}", why);
    }
    env::set_var("RUST_LOG", "error");
    pretty_env_logger::init();

    if cfg!(debug_assertions) {
        log::warn!("Running Bot in Debug mode.");
    }

    let commands = vec![
        exam::fe::fe(),

        general::about::about(),
        general::userinfo::avatar(),
        general::userinfo::banner(),
        general::userinfo::user(),
        general::serverinfo::server(),

        spotify::spotify(),

        starrail::command::starrail(),
        dokkan::command::dokkanbattle(),

        meonly::me_only(),
    ];

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands,
            event_handler: |_ctx, event, _framework, _data| {
                Box::pin(event_handler(_ctx, event, _framework, _data))
            },
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("s;".into()),
                ..Default::default()
            },
            ..Default::default()
        })
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await.unwrap();
                Ok(Data {
                    pool: Mutex::new(get_pool().await?),
                    mihoyo: Mutex::new(miHoYo_API::client::Client::default())
                })
            })
        })
        .build();

    let token = env::var("BOT_TOKEN")?;
    let intents = serenity::GatewayIntents::all();

    let mut client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await?;

    if let Err(why) = client.start_autosharded().await {
        log::error!("Client Error: {}", why);
    }

    Ok(())
}
