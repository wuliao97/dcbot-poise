mod apis;
mod commands;
mod data;
mod event;
mod utils;


use std::{env, error};

extern crate log;
extern crate pretty_env_logger;

use crate::data::Data;
use crate::event::event_handler;
use crate::commands::{
    general::{
        about::about,
        userinfo::{
            avatar,
            banner,
            user,
        },
        serverinfo::{
            server
        },
    },
    music::{
        spotify::spotify,
    },
    meonly::me_only,
};



#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>>{
    env::set_var("RUST_LOG", "error");
    if let Err(why) = dotenv::dotenv() {
        log::error!("Unable to find .env file: {}", why);
    }
    pretty_env_logger::init();

    let token = env::var("BOT_TOKEN").unwrap();

    if cfg!(debug_assertions) {
       log::warn!("Running Bot in Debug mode.");
    }

    let commands = vec![
        about(),
        avatar(),
        banner(),
        user(),
        server(),


        spotify(),


        me_only(),
    ];

    let framework = poise::Framework::builder()
        .token(token)
        .intents(poise::serenity_prelude::GatewayIntents::all())
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
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await.unwrap();
                Ok(Data {
                    //
                })
            })
        });

    if let Err(why) = framework.run_autosharded().await {
        log::error!("Client Error: {}", why);
    }

    Ok(())
}
