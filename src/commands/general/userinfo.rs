use poise::serenity_prelude as serenity;
use poise::serenity_prelude::Mentionable;
use serenity::model::prelude::OnlineStatus;
use crate::{quote, url};


use crate::utils::{Context, Error};
use crate::utils::constant::*;


/// Display info of selected a User
#[poise::command(slash_command, guild_only)]
pub async fn user(
    ctx: Context<'_>,
    #[description = "Selected user"]
    #[description_localized("ja", "ユーザー")]
    user: Option<serenity::User>
) -> Result<(), Error> {
    let user_id = user.as_ref().unwrap_or_else(|| ctx.author()).id;
    let user = ctx.cache().user(user_id).unwrap();
    let member = ctx.guild().unwrap().member(&ctx.http(), user.id).await?;

    let name = quote!(&user.name);
    let nick = {
        let tmp = member.nick.clone().unwrap_or_else(|| member.display_name().to_string());
        quote!(tmp)
    };
    let is_bot = {
        let bot = if user.bot { "Yes" } else { "No" };
        quote!(bot)
    };
    let (role, role_count) = {
        let roles = member.roles.clone();
        let formatted_roles = roles.iter()
            .cloned()
            .map(|r| r.mention().to_string())
            .collect::<Vec<String>>()
            .join(" ");

        (quote!(formatted_roles), roles.clone().len())
    };
    let created = {
        let date = user.clone().created_at().timestamp();
        quote!(format!("<t:{}:R>", date))
    };
    let joined = {
        let date = member.clone().joined_at.unwrap().timestamp();
        quote!(format!("<t:{}:R>", date))
    };
    let status_color = {
        if let Some(presence) = ctx.guild().unwrap().presences.clone().get(&user_id.clone()) {
            match presence.status {
                OnlineStatus::DoNotDisturb => { 0xE74C3C }
                OnlineStatus::Idle => { 0xE67E22 }
                OnlineStatus::Online => { 0x2ECC71 }
                _ => { COLOR_GRAY }
            }
        } else {
            COLOR_GRAY
        }
    };
    let avatar_url = member.avatar_url().unwrap_or_else(|| user.default_avatar_url());
    let banner = member.user.banner_url();

    ctx.send(|c| {
        c.embed(|e| {
            e.field("Name", name, true)
                .field("Display Name", nick, true)
                .field("Bot?", is_bot, true)

                .field(format!("Role ({})", role_count), role, false)

                .field("Created Date", created, true)
                .field("Joined Date", joined, true)

                .footer(|f| f.text(format!("ID: {}", &user.id)))
                .color(status_color)
                .thumbnail(avatar_url);

            if let Some(banner_url) = banner {
                e.image(banner_url);
            }

            e

        })
    }).await.unwrap();

    Ok(())
}


/// Display Avatar of selected User
#[poise::command(slash_command, guild_only)]
pub async fn avatar(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>
) -> Result<(), Error> {
    let user = user.as_ref().unwrap_or_else(|| ctx.author());

    let description = {
        let mentioned_user = &user.mention();
        format!("{}'s Avatar", mentioned_user)
    };

    let user_avatar = user.avatar_url().unwrap_or_else(|| user.default_avatar_url());
    let member_avatar = ctx.cache().guild(&ctx.guild_id().unwrap()).unwrap()
        .member(&ctx.http(), user.id)
        .await?
        .avatar_url().unwrap_or_else(|| user.default_avatar_url());

    let is_same_url = is_same(&user_avatar, &member_avatar);

    let url = {
        let mut urls = Vec::new();

        if is_same_url {
            urls.push(url!("User Avatar", user_avatar));
        } else {
            urls.push(url!("User Avatar", user_avatar));
            urls.push(url!("Server Avatar", member_avatar));
        }

        urls.join(", ")
    };

    ctx.send(|c| {
        c.embed(|e| {
            e.description(description)
                .color(COLOR_OKAY)
                .field("URLs", quote!(url), true)
                .image(member_avatar);
            if !is_same_url {
                e.thumbnail(user_avatar)
            } else {
                e.color(COLOR_GRAY)
            }
        })
    }).await.unwrap();

    Ok(())
}


// Why'd get the banner of user...and WTF REST API idk even tho
/// Display Banner of selected User if user have
#[poise::command(slash_command, guild_only)]
pub async fn banner(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>
) -> Result<(), Error> {
    let user_id = user.as_ref().unwrap_or_else(|| ctx.author()).id;
    let member = ctx.guild().unwrap().member(&ctx.http(), user_id).await?;
    let banner = &member.user.banner_url();

    let url = {
        let tmp = banner.clone().unwrap();
        quote!(url!("Banner", tmp))
    };

    ctx.send(|c| {
        c.embed(|e| {
            if banner.is_some() {
                e.description(format!("**{}'s Banner**", member.mention()))
                    .image(banner.clone().unwrap())
                    .field("URLs", url, true)
                    .color(COLOR_OKAY)
            } else {
                e.description(format!("Oops, Couldn't get Banner url from {}.. It might haven't a Banner...? :wave:", member.mention()))
                    .color(COLOR_FAIL)
            }

        })
    }).await.unwrap();

    Ok(())
}



#[inline]
fn is_same(url_1: &String, member_a: &String) -> bool {
    if url_1 == member_a {
        return true;
    };
    false
}