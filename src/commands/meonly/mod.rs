use poise::serenity_prelude::{Member, Mentionable};
use serenity::model::id::GuildId;

use crate::{quote, url};
use crate::utils::{Context, Error};

#[poise::command(
prefix_command,
hide_in_help,
owners_only,
rename = "src",
subcommands("list", "server", "members"),
)]
pub async fn me_only(
    _: Context<'_>
) -> Result<(), Error> {
    Ok(())
}


#[poise::command(
prefix_command,
hide_in_help,
owners_only,
)]
pub async fn list(ctx: Context<'_>) -> Result<(), Error> {
    let cache = ctx.cache().clone();
    let guilds = cache.guilds();

    let guild_sets = guilds
        .iter()
        .map(|guild| format!("{} - {}", guild.to_string(), guild.name(&ctx.cache()).unwrap()))
        .collect::<Vec<String>>()
        .join("\n");

    ctx.send(|send| {
        send.embed(|embed| {
            embed.description(guild_sets)
                .footer(|f| f.text(format!("Total Servers: {}", guilds.len())))
        })
    }).await.unwrap();


    Ok(())
}

#[poise::command(
prefix_command,
hide_in_help,
owners_only,
)]
pub async fn server(
    ctx: Context<'_>,
    id: u64,
) -> Result<(), Error> {
    let guild = ctx.cache().guild(GuildId(id)).unwrap_or(ctx.guild().unwrap());

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

    ctx.send(|send| {
        send.embed(|e| {
            e.description(format!("**{}**'s info", &guild.name))
                .field("Owner", owner, true)
                .field("Role", roles, true)
                .field("Creation", creation, true)
                .field("Member", members, true)
                .field("Channel", channels, true)
                .field("Emoji", emojis, true);
            if let Some(urls) = urls {
                e.field("URLs", quote!(urls), false);
            }
            e
        })
    }).await.unwrap();

    Ok(())
}

#[poise::command(
prefix_command,
hide_in_help,
owners_only,
)]
pub async fn members(
    ctx: Context<'_>,
    id: u64,
) -> Result<(), Error> {
    let guild = ctx.cache().guild(GuildId(id)).unwrap_or(ctx.guild().unwrap());

    let material = guild.members.values().cloned().collect::<Vec<Member>>();
    let member_list = material
        .iter()
        .map(|member| format!("{} - {}", member.user.id, member.display_name()))
        .collect::<Vec<String>>()
        .join("\n");

    ctx.send(|send| {
        send.embed(|e| {
            e.description(member_list)
        })
    }).await.unwrap();

    Ok(())
}