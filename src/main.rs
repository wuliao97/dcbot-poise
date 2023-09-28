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
        // spotify::register,
    },
    meonly::me_only,
    //
    // test::{
    //     welcome,
    //     button::button,
    // }
};



#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>>{
    env::set_var("RUST_LOG", "error");
    if let Err(why) = dotenv::dotenv() {
        log::error!("Unable to find .env file: {}", why);
    }
    pretty_env_logger::init();

    let token = env::var("BOT_TOKEN").unwrap();

    // let http = Http::new(&token);
    // let (owners, _) = match http.get_current_application_info().await {
    //     Ok(info) => {
    //         let mut owners = HashSet::new();
    //         owners.insert(info.owner.id);
    //
    //         if let Some(team) = info.team {
    //             for member in team.members {
    //                 owners.insert(member.user.id);
    //             }
    //         }
    //         (owners, info.id)
    //     }
    //     Err(why) => {
    //         log::warn!("Couldn't access Application info: {:?}", why);
    //         log::warn!("Trying environment variable for Bot id.");
    //         let id = env::var("BOT_ID").unwrap();
    //         let bot_id = id.parse::<u64>().unwrap();
    //         (HashSet::new(), ApplicationId(bot_id))
    //     }
    // };

    if cfg!(debug_assertions) {
       log::warn!("Running Bot in Debug mode.");
    }

    // log::info!("Registering owners:");
    // for owner in owners {
    //     log::info!("    ・{}", owner);
    // }

    let commands = vec![
        about(),
        avatar(),
        banner(),
        user(),
        server(),


        spotify(),


        me_only(),
        // register(),
        // welcome(),
        // button(),
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
