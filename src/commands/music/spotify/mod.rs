use poise::serenity_prelude as serenity;
use poise::serenity_prelude::Mentionable;
use crate::quote;


use crate::utils::{Context, Error};
use crate::utils::constant::*;
use crate::utils::discord::activity::{SpotifyActivity, InfoType};


#[poise::command(
    slash_command, subcommands("track", "cover", "listening", ), subcommand_required
)]
pub async fn spotify(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}


#[poise::command(slash_command)]
pub async fn track(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>
) -> Result<(), Error> {
    let user = user.as_ref().unwrap_or_else(|| ctx.author());
    let mut activity = SpotifyActivity::new(ctx, user.id);

    if !activity.listening().await {
        ctx.send(|c|
            c.embed(|e| {
                e.description(format!("{} {}", user.mention(), SPTFY_ERR_MSG_CASE_1))
                    .field("Try again", quote!("</spotify track:1152405901936971879>"), true)
            })
                .ephemeral(true)
        ).await.unwrap()
    } else {
        ctx.send(|c| {
            let url = activity.get_track_url();
            c.content(url)
        }).await.unwrap()
    };


    Ok(())
}


#[poise::command(slash_command)]
pub async fn cover(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>
) -> Result<(), Error> {
    let user = user.as_ref().unwrap_or_else(|| ctx.author());
    let mut activity = SpotifyActivity::new(ctx, user.id);

    if !activity.listening().await {
        ctx.send(|c|
            c.embed(|e| {
                e.description(format!("{} {}", user.mention(), SPTFY_ERR_MSG_CASE_1))
                    .field("Try again", quote!("</spotify cover:1152405901936971879>"), true)
            })
                .ephemeral(true)
        ).await.unwrap()
    } else {
        ctx.send(|c| {
            c.embed(|e| {
                let url = activity.get_cover_url();
                e.description(format!("{} is Listening", user.mention()))
                    .field("", activity.title_with_url(), false)
                    .image(url)
            })
        }).await.unwrap()
    };

    Ok(())
}


#[poise::command(slash_command)]
pub async fn listening(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>
) -> Result<(), Error> {
    let user = user.as_ref().unwrap_or_else(|| ctx.author());
    let mut activity = SpotifyActivity::new(ctx, user.id);

    if !activity.listening().await {
        ctx.send(|c|
            c.embed(|e| {
                e.description(
                    format!("{} {}", user.mention(), SPTFY_ERR_MSG_CASE_1)
                )
                    .field("Try again", quote!("</spotify listening:1152405901936971879>"), true)
            })
                .ephemeral(true)
        ).await.unwrap()
    } else {
        ctx.send(|c| {
            c.embed(|e| {
                let (title, artist, album) = activity.get_info(InfoType::WithUrl);
                // let fields = artist;

                e.description(format!("{} {}", user.mention(), SPTFY_MSG_CASE_ONE))
                    .thumbnail(activity.get_cover_url())
                    .field("title", quote!(title), false)
                    .field("by", quote!(artist), false)
                    .field("on", quote!(album), false)
                    // .fields(fields)
                    .footer(|f| f.text(activity.format_time()))
                    .color(activity.get_color())
            })
        }).await.unwrap()
    };


    Ok(())
}

