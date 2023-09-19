mod apis;
mod commands;
mod data;
mod utils;


use std::env;
use std::collections::HashSet;

use log::{info, warn, error};
use poise::serenity_prelude::Http;
use poise::serenity_prelude::ApplicationId;

use crate::data::Data;
use crate::commands::{
    general::{
        about::about,
        userinfo::{
            avatar,
            banner,
            user,
        }
    },
    music::{
        spotify::spotify
    },

    test::{
        welcome,
    }
};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    if let Err(why) = dotenv::dotenv() {
        error!("Unable to find .env file: {}", why);
    }

    pretty_env_logger::init();

    let token = env::var("BOT_TOKEN").unwrap();
    let http = Http::new(&token);

    let (owners, _) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            if let Some(team) = info.team {
                for member in team.members {
                    owners.insert(member.user.id);
                }
            }
            (owners, info.id)
        }
        Err(why) => {
            warn!("Couldn't access Application info: {:?}", why);
            warn!("Trying environment variable for Bot id.");
            let id = env::var("BOT_ID").unwrap();
            let bot_id = id.parse::<u64>().unwrap();
            (HashSet::new(), ApplicationId(bot_id))
        }
    };

    if cfg!(debug_assertions) {
        warn!("Running Bot in Debug mode.");
    }

    info!("Registering owners:");
    for owner in owners {
        info!("    ・{}", owner);
    }

    let commands = vec![
        about(),
        avatar(),
        banner(),
        user(),
        spotify(),

        welcome(),

    ];

    let framework = poise::Framework::builder()
        .token(token)
        .intents(poise::serenity_prelude::GatewayIntents::all())
        .options(poise::FrameworkOptions {
            commands,
            // on_error: (),
            // pre_command: (),
            // post_command: (),
            // command_check: None,
            // skip_checks_for_owners: false,
            // allowed_mentions: None,
            // reply_callback: None,
            // manual_cooldowns: false,
            // require_cache_for_guild_check: false,
            // event_handler: (),
            // listener: (),
            // prefix_options: PrefixFrameworkOptions {},
            // owners,
            // __non_exhaustive: (),
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await.unwrap();
                Ok(Data {})
            })
        });

    if let Err(why) = framework.run_autosharded().await {
        error!("Client Error: {}", why);
    }


    Ok(())
}
