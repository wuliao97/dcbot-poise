use poise::serenity_prelude as serenity;
use poise::CreateReply;
use poise::serenity_prelude::Mentionable;
use crate::{quote, url};


use crate::utils::{Context, Error};
use crate::utils::constant::*;


#[poise::command(slash_command)]
pub async fn user(
    ctx: Context<'_>,
    #[description = "Selected user"]
    #[description_localized("ja", "ユーザー")]
    user: Option<serenity::User>
) -> Result<(), Error> {
    let user_id = user.as_ref().unwrap_or_else(|| ctx.author());
    let user = ctx.cache().user(user_id).unwrap();

    dbg!(user);

    ctx.send(|c| {
        c.embed(|e| {
            e.description("test")
        })
    }).await.unwrap();

    Ok(())
}


#[poise::command(slash_command)]
pub async fn avatar(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>
) -> Result<(), Error> {
    let user_id = user.as_ref().unwrap_or_else(|| ctx.author());
    let user = ctx.cache().user(user_id).unwrap();

    let description = {
        let mentioned_user = &user.mention();
        format!("{}'s Avatar", mentioned_user)
    };

    let user_avatar = user.avatar_url().unwrap();

    let member_avatar = ctx.cache().guild(&ctx.guild_id().unwrap()).unwrap()
        .member(&ctx.http(), user_id)
        .await.unwrap()
        .avatar_url().unwrap();

    let url = {
        let mut urls = Vec::new();

        if user_avatar == member_avatar {
            urls.push(url!("User Avatar", user_avatar));
        } else {
            urls.push(url!("User Avatar", user_avatar));
            urls.push(url!("Server Avatar", member_avatar));
        }

        urls.join(", ")
    };


    // ctx.send(|c| {
    //     c.
    // }).await.unwrap();

    dbg!(user_avatar);
    dbg!(member_avatar);
    dbg!(url);



    Ok(())
}


#[poise::command(slash_command)]
pub async fn banner(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>
) -> Result<(), Error> {
    let user_id = user.as_ref().unwrap_or_else(|| ctx.author());
    let user = ctx.cache().user(user_id).unwrap();

    Ok(())
}
