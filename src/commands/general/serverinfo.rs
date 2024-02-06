use poise::CreateReply;
use poise::serenity_prelude::Member;
use poise::serenity_prelude::Mentionable;
use serenity::all::{CreateEmbed, CreateEmbedFooter};

use crate::{quote, url};
use crate::utils::{Context, Error};
use crate::utils::constant::*;


/// Display info of the Server
#[poise::command(
    slash_command,
    guild_only
)]
pub async fn server(ctx: Context<'_>) -> Result<(), Error> {
    // IDK way how to avoid to use clone()
    let guild = ctx.guild().unwrap().clone();
    let owner = quote!(&guild.owner_id.mention());
    let roles = quote!(format!("**{}**", &guild.roles.len()));
    let creation = {
        let date = &ctx.guild_id().unwrap().created_at().timestamp();
        quote!(format!("<t:{}:R>", date))
    };
    let members = {
        let members = &guild.members.values().cloned().collect::<Vec<Member>>();
        let bots = members.iter().filter(|m| m.user.bot).count();
        let users = (members.len() - 1) - bots;
        quote!(format!("**{}** User | **{}** Bot", users, bots))
    };
    let channels = {
        let channels = guild.channels
            .values()
            .filter(|c| c.guild(ctx.cache()).is_some())
            .collect::<Vec<_>>();
        let texts = channels.iter().filter(|c| c.kind.name().eq("text")).count();
        let voices = channels.iter().filter(|c| c.kind.name().eq("voice")).count();
        quote!(format!("**{}** Text | **{}** Voice", texts, voices))
    };
    let emojis = {
        let emojis = guild.emojis.len();
        let stickers = guild.stickers.len();
        quote!(format!("**{}** Emoji | **{}** Sticker", emojis, stickers))
    };

    let footer = format!("ID: {}", &guild.id);

    let mut embed = CreateEmbed::default()
        .description(format!("**{}**'s info", &guild.name))
            .field("Owner", owner, true)
            .field("Role", roles, true)
            .field("Creation", creation, true)
            .field("Member", members, true)
            .field("Channel", channels, true)
            .field("Emoji", emojis, true)
            .footer(CreateEmbedFooter::new(footer))
            .color(COLOR_OKAY);

    let urls: Option<String> = {
        let mut urls: Vec<String> = Vec::new();

        if let Some(icon_url) = guild.icon_url() {
            urls.push(url!("Icon", icon_url))
        };

        if let Some(banner_url) = guild.banner_url() {
            urls.push(url!("Banner", banner_url))
        };

        if let Some(splash_url) = guild.splash_url() {
            urls.push(url!("Splash", splash_url))
        };

        if !urls.is_empty(){
            Some(urls.join(", "))
        } else {
            None
        }
    };

    if let Some(urls) = urls {
        embed = embed.field("URLs", quote!(urls), false);
    }

    ctx.send(CreateReply::default().embed(embed)).await?;

    Ok(())
}