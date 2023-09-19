use poise::serenity_prelude as serenity;
use poise::serenity_prelude::Mentionable;
use crate::quote;


use crate::utils::{Context, Error};
use crate::utils::constant::*;
use crate::utils::discord::{
    describe::spotify_err_msg_one,
    activity::{SpotifyActivity, InfoType}
};


// (e.g.Line:14)Why'd this gonna be default description of Slash command!?!?!
// I got Shocked fr

// Various Spotify Command
#[poise::command(
slash_command,
subcommands("track", "cover", "listening", ),
subcommand_required,
name_localized("ja", "スポティファイ"),
description_localized("ja", "スポティファイの様々なコマンド"),
)]
pub async fn spotify(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}


/// Display the Track url that the User is Listening to
#[poise::command(
slash_command,
name_localized("ja", "トラック"),
description_localized("ja", "ユーザーが聞いてるトラックのURLを表示"),
)]
pub async fn track(
    ctx: Context<'_>,
    #[description = "Selected user"]
    #[description_localized("ja", "表示したいユーザー")]
    user: Option<serenity::User>
) -> Result<(), Error> {
    let user = user.as_ref().unwrap_or_else(|| ctx.author());
    let mut activity = SpotifyActivity::new(ctx, user.id);

    if !activity.listening().await {
        ctx.send(|c|
            c.embed(|e| {
                let err_msg = format!("{} {}", user.mention(), spotify_err_msg_one(ctx.locale()));
                e.description(err_msg)
                    .field("Try again", quote!("</spotify cover:1152405901936971879>"), true)
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

/// Display the Track url that the User is Listening to
#[poise::command(
slash_command,
name_localized("ja", "ジャケット"),
description_localized("ja", "ユーザーが聞いてるトラックのジャケットを表示"),
)]
pub async fn cover(
    ctx: Context<'_>,
    #[description = "Selected user"]
    #[description_localized("ja", "表示したいユーザー")]
    user: Option<serenity::User>
) -> Result<(), Error> {
    let user = user.as_ref().unwrap_or_else(|| ctx.author());
    let mut activity = SpotifyActivity::new(ctx, user.id);

    if !activity.listening().await {
        ctx.send(|c|
            c.embed(|e| {
                let err_msg = format!("{} {}", user.mention(), spotify_err_msg_one(ctx.locale()));
                e.description(err_msg)
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

// Display the Track info that the User Listening to
#[poise::command(
slash_command,
name_localized("ja", "リスニング"),
description_localized("ja", "ユーザーが聞いてるトラックの情報を表示"),
)]
pub async fn listening(
    ctx: Context<'_>,
    #[description = "Selected user"]
    #[description_localized("ja", "表示したいユーザー")]
    user: Option<serenity::User>
) -> Result<(), Error> {
    let user = user.as_ref().unwrap_or_else(|| ctx.author());
    let mut activity = SpotifyActivity::new(ctx, user.id);

    if !activity.listening().await {
        ctx.send(|c|
            c.embed(|e| {
                let err_msg = format!("{} {}", user.mention(), spotify_err_msg_one(ctx.locale()));
                e.description(err_msg)
                    .field("Try again", quote!("</spotify cover:1152405901936971879>"), true)
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

