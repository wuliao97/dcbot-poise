use poise::{CreateReply, serenity_prelude as serenity};
use poise::serenity_prelude::Mentionable;
use serenity::builder::{CreateActionRow, CreateEmbed, CreateEmbedFooter};
use rspotify::model::{SearchType, Country, Market::*};
use serenity::all::{ButtonStyle, ComponentInteractionDataKind, CreateButton, CreateSelectMenu, CreateSelectMenuKind, CreateSelectMenuOption, EditMessage};
use serenity::futures::StreamExt;

use crate::apis::spotify::{SpotifyAPI, ExtractInfo};
use crate::{
    quote,
    utils::{
        Context, Error,
        discord::{
            describe::{
                spotify_err_msg_one,
                SpotifySearchType,
                SpotifySearchLanguage,
            },
            activity::{
                SpotifyActivity,
                InfoType,
            },
            page::Page,
        },
        constant::*,
        spotify_handler::SpotifyHandler,
    }
};
use crate::utils::collection::distinction_vec;
use crate::utils::discord::embed::spotify_some_error;
use crate::utils::discord::page::Paging;


// (e.g.Line:40) Why'd this gonna be default description of Slash command!?!?!
// I got Shocked fr

/// Various Spotify Command
#[poise::command(
    slash_command,
    subcommands("track", "cover", "listening", "search"),
    subcommand_required,
    name_localized("ja", "スポティファイ"),
    description_localized("ja", "スポティファイの様々なコマンド"),
    guild_only
)]
pub async fn spotify(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}


/// Display the Track url that the User is Listening to
#[poise::command(
    slash_command,
    name_localized("ja", "トラック"),
    description_localized("ja", "ユーザーが聞いてるトラックのURLを表示"),
    guild_only
)]
pub async fn track(
    ctx: Context<'_>,
    #[description = "Selected user"]
    #[description_localized("ja", "表示したいユーザー")]
    user: Option<serenity::User>
) -> Result<(), Error> {
    let user = user.as_ref().unwrap_or_else(|| ctx.author());
    let mut activity = SpotifyActivity::new(ctx, user.id);
    let flag = activity.listening().await;

    let reply = if flag {
        let url = activity.get_track_url();
        CreateReply::default().content(url)
    } else {
        let err_msg = format!("{} {}", user.mention(), spotify_err_msg_one(ctx.locale()));
        CreateReply::default().embed(spotify_some_error(err_msg, Some("</spotify track:1111367127614640178>".to_string())))
            .ephemeral(!flag)
    };

    ctx.send(reply).await?;

    Ok(())
}

/// Display the Track url that the User is Listening to
#[poise::command(
    slash_command,
    name_localized("ja", "ジャケット"),
    description_localized("ja", "ユーザーが聞いてるトラックのジャケットを表示"),
    guild_only
)]
pub async fn cover(
    ctx: Context<'_>,
    #[description = "Selected user"]
    #[description_localized("ja", "表示したいユーザー")]
    user: Option<serenity::User>
) -> Result<(), Error> {
    let user = user.as_ref().unwrap_or_else(|| ctx.author());
    let mut activity = SpotifyActivity::new(ctx, user.id);
    let flag = activity.listening().await;
    let embed = if flag {
        let url = activity.get_cover_url();
        CreateEmbed::default()
            .description(format!("{} is Listening", user.mention()))
            .field("", activity.title_with_url(), false)
            .image(url)
            .color(SPOTIFY_GREEN)
            .clone()
    } else {
        let err_msg = format!("{} {}", user.mention(), spotify_err_msg_one(ctx.locale()));
        spotify_some_error(err_msg, Some("</spotify cover:1111367127614640178>".to_string()))
    };

    let reply = CreateReply::default().embed(embed)
        .ephemeral(!flag);

    ctx.send(reply).await.unwrap();

    Ok(())
}


/// Display the Track info that the User Listening to
#[poise::command(
    slash_command,
    name_localized("ja", "リスニング"),
    description_localized("ja", "ユーザーが聞いてるトラックの情報を表示"),
    guild_only
)]
pub async fn listening(
    ctx: Context<'_>,
    #[description = "Selected user"]
    #[description_localized("ja", "表示したいユーザー")]
    user: Option<serenity::User>
) -> Result<(), Error> {
    let user = user.as_ref().unwrap_or_else(|| ctx.author());
    let mut activity = SpotifyActivity::new(ctx, user.id);
    let flag = activity.listening().await;
    let embed = if flag {
        let (title, artist, album) = activity.get_info(InfoType::WithUrl);
        CreateEmbed::default()
            .description(format!("{} {}", user.mention(), SPTFY_MSG_CASE_ONE))
            .thumbnail(activity.get_cover_url())
            .field("title", quote!(title), false)
            .field("by", quote!(artist), false)
            .field("on", quote!(album), false)
            .footer(CreateEmbedFooter::new(activity.format_time()))
            .color(activity.get_color())
    } else {
        let err_msg = format!("{} {}", user.mention(), spotify_err_msg_one(ctx.locale()));
        spotify_some_error(err_msg, Some("</spotify listening:1111367127614640178>".to_string()))
    };
    let reply = CreateReply::default()
        .embed(embed)
        .ephemeral(!flag);
    ctx.send(reply).await.unwrap();

    Ok(())
}


// I'd like to fix this spaghetti but I'm not enough done that my skills. 02/10/2023

/// ※BETA VERSION Agent of search on Spotify
#[poise::command(
    slash_command,
    name_localized("ja", "検索"),
    description_localized("ja", "Spotifyで検索"),
)]
pub async fn search(
    ctx: Context<'_>,
    #[description = "Keyword"]
    #[description_localized("ja", "検索語句")]
        q: String,
    #[rename = "search-type"]
    #[description = "Search Type | default: Track"]
    #[description_localized("ja", "検索対象のタイプ | デフォルト: トラック")]
        search_type: Option<SpotifySearchType>,
    #[description = "default: 5"]
    #[max = 50u8]
    #[min = 5u8]
        limit: Option<u32>,
    #[description = "Language Option of Search"]
    #[description_localized("ja", "検索オプションの言語選択")]
    language: Option<SpotifySearchLanguage>
) -> Result<(), Error> {
    let uuid = ctx.id();
    let api = SpotifyAPI::new().await;
    let user = ctx.author();
    let limit = limit.unwrap_or_else(|| 5);
    let search_type = match search_type.as_ref().unwrap_or(&SpotifySearchType::A) {
        SpotifySearchType::A => SearchType::Track,
        SpotifySearchType::B => SearchType::Artist,
        SpotifySearchType::C => SearchType::Album,
    };
    let language = match language.as_ref().unwrap_or(&SpotifySearchLanguage::A) {
        SpotifySearchLanguage::A => Country(Country::UnitedStates),
        SpotifySearchLanguage::B => Country(Country::Japan),
    };

    let search_result = api.search(q.as_str(), search_type, Some(language), limit).await.unwrap();

    let extract = ExtractInfo::new(search_result);
    let names = distinction_vec(&extract.names(), 10);
    let formatted = extract.vec_to_show();
    let distinction_vec = distinction_vec(&formatted, 10);
    let handler = SpotifyHandler::new(formatted.clone());
    let mut embeds = Page::from_vec(handler.decorate_embeds(distinction_vec.clone(), &user));

    let components = vec![
        tmp(names.clone(), embeds.get_current()),
        tmp_2(embeds.available_page())
    ];

    let inter = ctx.send(CreateReply::default()
        .embed(embeds.get_page(0).unwrap().clone())
        .components(components)).await?;


    // while let Some(mut interaction) = serenity::ComponentInteractionCollector::new(ctx)
    //     .author_id(ctx.author().id)
    //     .channel_id(ctx.channel_id())
    //     .timeout(std::time::Duration::from_secs(240))
    //     .filter(move|mci| mci.data.custom_id == uuid.to_string())
    //     .await
    while let Some(interaction) = inter
        .message()
        .await?
        .await_component_interactions(ctx.serenity_context())
        .author_id(user.id)
        .timeout(std::time::Duration::from_secs(240))
        .await
    {
        interaction.defer(&ctx.http()).await.unwrap();
        match interaction.data.kind {
            ComponentInteractionDataKind::Button => {
                let embed = match interaction.data.custom_id.as_str() {
                    "search_left" => embeds.first().unwrap(),
                    "search_left_one" => embeds.previous().unwrap(),
                    "search_right_one" => embeds.next().unwrap(),
                    "search_right" => embeds.last().unwrap(),
                    _ => CreateEmbed::default().description("custom_id error.").clone(),
                };
                let page = embeds.get_current();
                let components = vec![
                    tmp(names.clone(), page),
                    tmp_2(embeds.available_page())
                ];
                inter.edit(ctx.clone(), CreateReply::default()
                    .embed(embed)
                    .components(components)
                ).await.unwrap()
            }
            ComponentInteractionDataKind::StringSelect { values } => {
                match interaction.data.custom_id.as_str() {
                    "select_song" => {
                        let page = embeds.get_current();
                        let column = values[0].parse::<usize>()?;
                        let index = (page * 10) + column;

                        inter.edit(ctx, CreateReply::default()
                            .embed(extract.to_show_with_embed(index))
                            .components(vec![])
                        ).await.unwrap();
                        break;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
    Ok(())
}



fn tmp(names: Vec<Vec<String>>, page: usize) -> CreateActionRow {
    let options: Vec<CreateSelectMenuOption> = names.get(page).unwrap().iter().enumerate()
        .map(|(idx, name)| CreateSelectMenuOption::new(name, idx.to_string()))
        .collect();

    CreateActionRow::SelectMenu(CreateSelectMenu::new(
        "select_song", CreateSelectMenuKind::String { options })
                .min_values(1)
                .max_values(1)
                .placeholder("Choose your want a Song!")
    )
}

fn tmp_2((left, right): (bool, bool)) -> CreateActionRow {
    let buttons = vec![
        CreateButton::new("search_left")
            .label("<<")
            .style(ButtonStyle::Success)
            .disabled(left),
        CreateButton::new("search_left_one")
            .label("<")
            .style(ButtonStyle::Success)
            .disabled(left),
        CreateButton::new("search_right_one")
            .label(">")
            .style(ButtonStyle::Success)
            .disabled(right),
        CreateButton::new("search_right")
            .label(">>")
            .style(ButtonStyle::Success)
            .disabled(right),
    ];
    CreateActionRow::Buttons(buttons)
}