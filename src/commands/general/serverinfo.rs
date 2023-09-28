use poise::serenity_prelude::Member;
use poise::serenity_prelude::Mentionable;

use crate::{quote, url};
use crate::utils::{Context, Error};
use crate::utils::constant::*;


/// Display info of the Server
#[poise::command(slash_command, guild_only)]
pub async fn server(ctx: Context<'_>) -> Result<(), Error> {
    let guild = ctx.guild().unwrap();

    let owner = quote!(&guild.owner_id.mention());
    let roles = quote!(format!("**{}**", &guild.roles.len()));
    let creation = {
        let date = &ctx.guild_id().unwrap().created_at().timestamp();
        quote!(format!("<t:{}:R>", date))
    };
    let members = {
        let members = &guild.members.values().cloned().collect::<Vec<Member>>();
        let bots = members.iter().cloned().filter(|m| m.user.bot).count();
        let users = (members.len() - 1) - bots;
        quote!(format!("**{}** User | **{}** Bot", users, bots))
    };
    let channels = {
        let channels: Vec<_> = guild.channels.values().cloned().filter(|c| c.clone().guild().is_some()).collect();
        let texts = channels.iter().cloned().filter(|c| c.clone().guild().unwrap().kind.name().eq("text")).count();
        let voices = channels.iter().cloned().filter(|c| c.clone().guild().unwrap().kind.name().eq("voice")).count();
        quote!(format!("**{}** Text | **{}** Voice", texts, voices))
    };
    let emojis = {
        let emojis = &guild.emojis.len();
        let stickers = &guild.stickers.len();
        quote!(format!("**{}** Emoji | **{}** Sticker", emojis, stickers))
    };
    let urls: Option<String> = {
        let mut urls: Vec<String> = Vec::new();

        if let Some(icon_url) = guild.icon_url().clone() {
            urls.push(url!("Icon", icon_url))
        };

        if let Some(banner_url) = guild.banner_url().clone() {
            urls.push(url!("Banner", banner_url))
        };

        if let Some(splash_url) = guild.splash_url().clone() {
            urls.push(url!("Splash", splash_url))
        };

        if !urls.is_empty(){
            Some(urls.join(", "))
        } else {
            None
        }
    };

    // dbg!(&guild);
    // let boost: Option<String> = {
    //
    //
    //     if guild.premium_subscription_count > 0 {
    //     }
    // };
    let footer = {
        let id = &guild.id;
        format!("ID: {}", id)
    };


    ctx.send(|c| {
        c.embed(|e| {
            e.description(format!("**{}**'s info", &guild.name))
                .field("Owner", owner, true)
                .field("Role", roles, true)
                .field("Creation", creation, true)
                .field("Member", members, true)
                .field("Channel", channels, true)
                .field("Emoji", emojis, true)
                .footer(|f| f.text(footer))
                .color(COLOR_OKAY);

            if let Some(urls) = urls {
                e.field("URLs", quote!(urls), false);
            }

            e
        })
    }).await.unwrap();


    Ok(())
}