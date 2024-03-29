use std::env;
use poise::CreateReply;
use serenity::all::CreateEmbed;
use sysinfo::System;

use crate::{quote, url};
use crate::utils::{Context, Error};
use crate::utils::constant::*;


/// About Me
#[poise::command(slash_command)]
pub async fn about(ctx: Context<'_>) -> Result<(), Error> {
    let description = {
        let desc = env!("CARGO_PKG_DESCRIPTION");
        let poise_url = url!("Poise", "https://github.com/serenity-rs/poise");
        desc.replace("Poise", poise_url.as_str())
    };

    let command = {
        let commands = ctx.framework().options.commands.len();
        let sub_commands: usize = ctx.framework().options.commands
            .iter()
            .filter(|c| !c.subcommands.is_empty())
            .map(|c| c.subcommands.len())
            .sum();
        format!("`{}` Commands\n`{}` Sub Commands", commands, sub_commands)
    };
    let user = {
        let cache = ctx.cache();
        let servers = cache.guild_count();
        let users: usize = cache.user_count();
        format!("`{}` Servers\n`{}` Users", servers, users)
    };
    let platform = {
        let system = System::new();
        let os = System::name().unwrap();
        let project_version = env!("CARGO_PKG_VERSION");
        format!("OS `{}`\nBot Version `{}`", os, project_version)
    };
    let support = {
        let owner_id = ctx.http().get_current_application_info().await?.owner.unwrap().id;
        let url = format!("{}{}", USER_SEARCH_FROM_ID, owner_id);
        url!("Owner Info", url)
    };
    let source = {
        let gh_url = env!("CARGO_PKG_REPOSITORY");
        url!("Github", gh_url)
    };

    let embed = CreateEmbed::default()
        .title("About me")
        .description(quote!(description))
        .field("Commands", quote!(command), true)
        .field("Users", quote!(user), true)
        .field("Platform", quote!(platform), true)
        .field("Support", quote!(support), true)
        .field("Source", quote!(source), true)
        .color(COLOR_OKAY)
        .clone();

    ctx.send(CreateReply::default().embed(embed)).await?;

    Ok(())
}


#[poise::command(
    slash_command
)]
pub async fn release_note(ctx: Context<'_>) -> Result<(), Error> {
    let emb = CreateEmbed::new()
        .description("demo");


    ctx.send(CreateReply::default().embed(emb))
        .await?;

    Ok(())
}