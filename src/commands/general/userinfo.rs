use poise::{CreateReply, serenity_prelude as serenity};
use poise::serenity_prelude::Mentionable;
use serenity::all::CreateEmbedFooter;
use serenity::builder::CreateEmbed;
use serenity::model::guild::Member;
use serenity::model::prelude::OnlineStatus;
use crate::{quote, url};


use crate::utils::{Context, Error};
use crate::utils::constant::*;


/// Display info of selected a User
#[poise::command(slash_command, guild_only)]
pub async fn user(
    ctx: Context<'_>,
    #[description = "Select user"]
    #[description_localized("ja", "ユーザー")]
    user: Option<serenity::User>
) -> Result<(), Error> {
    let user = user.as_ref().unwrap_or_else(|| ctx.author());
    dbg!(user);
    // let cache = ctx.cache();
    // let guild = ctx.guild().unwrap();
    // let user = cache.user(user_id).unwrap();
    // let member = guild.member(&ctx.http(), user.id.get()).await?.clone();
    //
    // let name = quote!(user.name);
    // let nick = {
    //     let tmp = member.nick.clone().unwrap_or_else(|| member.display_name().to_string());
    //     quote!(tmp)
    // };
    // let is_bot = {
    //     let bot = if user.bot { "Yes" } else { "No" };
    //     quote!(bot)
    // };
    // let (role, role_count) = {
    //     let roles = member.roles.clone();
    //     let formatted_roles = roles.iter()
    //         .cloned()
    //         .map(|r| r.mention().to_string())
    //         .collect::<Vec<String>>()
    //         .join(" ");
    //     (quote!(formatted_roles), roles.clone().len())
    // };
    // let created = {
    //     let date = user.clone().created_at().timestamp();
    //     quote!(format!("<t:{}:R>", date))
    // };
    // let joined = {
    //     let date = member.joined_at.unwrap().timestamp();
    //     quote!(format!("<t:{}:R>", date))
    // };
    // let status_color = {
    //     if let Some(presence) = guild.presences.clone().get(&user_id.clone()) {
    //         match presence.status {
    //             OnlineStatus::DoNotDisturb => { 0xE74C3C }
    //             OnlineStatus::Idle => { 0xE67E22 }
    //             OnlineStatus::Online => { 0x2ECC71 }
    //             _ => { COLOR_GRAY }
    //         }
    //     } else {
    //         COLOR_GRAY
    //     }
    // };
    // let avatar_url = member.avatar_url().unwrap_or_else(|| user.default_avatar_url());
    // let banner = member.user.banner_url();
    //
    // let mut embed = CreateEmbed::default()
    //         .field("Name", name, true)
    //         .field("Display Name", nick, true)
    //         .field("Bot?", is_bot, true)
    //         .field(format!("Role ({})", role_count), role, false)
    //         .field("Created Date", created, true)
    //         .field("Joined Date", joined, true)
    //         .footer(CreateEmbedFooter::new(format!("ID: {}", &user.id)))
    //         .color(status_color)
    //         .thumbnail(avatar_url);
    //
    // if let Some(banner_url) = banner {
    //     embed = embed.image(banner_url);
    // }
    //
    // ctx.send(CreateReply::default().embed(embed.clone())).await.unwrap();

    Ok(())
}


/// Display Avatar of selected User
#[poise::command(slash_command, guild_only)]
pub async fn avatar(
    ctx: Context<'_>,
    #[description = "Select user"] user: Option<serenity::User>
) -> Result<(), Error> {
    let user = user.as_ref().unwrap_or_else(|| ctx.author());
    let description = format!("{}'s Avatar", &user.mention());
    let avatar = user.avatar_url();
    let user_avatar = avatar.unwrap();
    let url = url!("User Avatar", &user_avatar);

    let mut embed = CreateEmbed::default()
        .description(description)
        .color(COLOR_OKAY)
        .field("URLs", quote!(url), true)
        .image(user_avatar);

    ctx.send(CreateReply::default().embed(embed.clone())).await.unwrap();

    Ok(())
}


// Why'd get the banner of user...and WTF REST API idk even tho
/// Display Banner of selected User if user have
#[poise::command(slash_command, guild_only)]
pub async fn banner(
    ctx: Context<'_>,
    #[description = "Select user"] user: Option<serenity::User>
) -> Result<(), Error> {
    let user = user.as_ref().unwrap_or_else(|| ctx.author());
    let banner = user.banner_url();
    let user_mentioned = user.mention();

    let mut embed = CreateEmbed::default();
    embed = match banner {
        None => embed.description(
            format!(
                "Oops, Couldn't get Banner url from {}.. It might haven't a Banner...? :wave:",
                user_mentioned))
            .color(COLOR_FAIL),
        Some(banner_url) => embed.description(format!("**{}'s Banner**", user_mentioned))
            .image(&banner_url)
            .field("URLs", quote!(banner_url), true)
            .color(COLOR_OKAY)
    };

    ctx.send(CreateReply::default().embed(embed.clone())).await?;

    Ok(())
}
