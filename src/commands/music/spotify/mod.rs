use poise::serenity_prelude::{
    self as serenity,
    // CollectComponentInteraction
};
use poise::serenity_prelude::Mentionable;
use serenity::builder::{CreateActionRow, CreateEmbed};
use rspotify::model::{SearchType, Country, Market::*};
use serenity::futures::StreamExt;
use serenity::model::application::component::ComponentType;
use serenity::model::prelude::component::ButtonStyle;

use crate::apis::spotify::{SpotifyAPI, ExtractInfo};
use crate::{
    url, quote, quote_with_bold,
    utils::{
        Context, Error,
        // cut_string,
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
use crate::utils::discord::page::Paging;


// (e.g.Line:41)Why'd this gonna be default description of Slash command!?!?!
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
                    .color(SPOTIFY_GREEN)
            })
        }).await.unwrap()
    };

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
                e.description(format!("{} {}", user.mention(), SPTFY_MSG_CASE_ONE))
                    .thumbnail(activity.get_cover_url())
                    .field("title", quote!(title), false)
                    .field("by", quote!(artist), false)
                    .field("on", quote!(album), false)
                    .footer(|f| f.text(activity.format_time()))
                    .color(activity.get_color())
            })
        }).await.unwrap()
    };

    Ok(())
}


// I'd like to fix this spaghetti but I'm not enough done that my skills.

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
    let material = extract.surface();
    let urls = extract.urls();
    let formatted = forming_vec_to_show(material, urls);
    let distinction_vec = distinction_vec(&formatted, 10);
    let handler = SpotifyHandler::new(formatted.clone());
    let mut embeds = Page::from_vec(handler.decorate_embeds(distinction_vec.clone(), &user));



    let inter = ctx.send(|c| {
        c.embeds = vec![embeds.get_page(0).unwrap().clone()];
        c.components(|c| {
            c.create_action_row(|row| {
                row.create_select_menu(|select| {
                    select.custom_id("select_song")
                        .min_values(1)
                        .max_values(1)
                        .placeholder("Choose your want a Song!")
                        .options(|option| {
                            for (idx, name) in names.get(0).unwrap().iter().enumerate() {
                                option.create_option(|o| o.label(name).value(idx));
                            }
                            option
                        })
                })
            });
            if embeds.get_page(1).is_some() {
                c.create_action_row(|row| {
                    row.create_button(|btn| {
                        btn.custom_id("search_left")
                            .label("<<")
                            .style(ButtonStyle::Success)
                            .disabled(true)
                    }).create_button(|btn| {
                        btn.custom_id("search_left_one")
                            .label("<")
                            .style(ButtonStyle::Success)
                            .disabled(true)
                    }).create_button(|btn| {
                        btn.custom_id("search_right_one")
                            .label(">")
                            .style(ButtonStyle::Success)
                    }).create_button(|btn| {
                        btn.custom_id("search_right")
                            .label(">>")
                            .style(ButtonStyle::Success)
                    })
                });
            }
            c
        })
    }).await.unwrap();

    // let mut context = ;
    while let Some(interaction) = inter.message().await.unwrap()
        .await_component_interactions(ctx.serenity_context())
        .author_id(user.id)
        .timeout(std::time::Duration::from_secs(240))
        .build()
        .next()
        .await
    {
        interaction.defer(&ctx.http()).await.unwrap();
        match interaction.data.component_type {
            ComponentType::Button => {
                let embed = match interaction.data.custom_id.as_str() {
                    "search_left" => {
                        embeds.first().unwrap()
                    }
                    "search_left_one" => {
                        embeds.previous().unwrap()
                    }
                    "search_right_one" => {
                        embeds.next().unwrap()
                    }
                    "search_right" => {
                        embeds.last().unwrap()
                    }
                    _ => {CreateEmbed::default()}
                };
                let page = embeds.get_current();

                inter.edit(ctx.clone(), |resp| {
                    resp.embed(|e| { *e = embed; e });
                    resp.components(|comp| {
                        comp
                            .add_action_row(tmp(names.clone(), page))
                            .add_action_row(tmp_2((embeds.available_page())))
                    })
                }).await.unwrap()
            }
            ComponentType::SelectMenu => {
                match interaction.data.custom_id.as_str() {
                    "select_song" => {
                        let page = embeds.get_current();
                        let column = interaction.data.values.get(0).unwrap().parse::<usize>().unwrap();
                        let index = (page * 10) + column;
                        let info = extract.general_info(index);
                        let sub_info = extract.sub_info(index);

                        inter.edit(ctx, |create_reply| {
                            create_reply.embed(|embed| {
                                embed.author(|a| a.name("Search Result"))
                                    .color(SPOTIFY_GREEN)
                                    .field("title", quote_with_bold!(info[0]), false)
                                    .field("by", quote_with_bold!(info[1]), false)
                                    .field("on", quote_with_bold!(info[2]), false)
                                    .thumbnail(sub_info.get(1).unwrap())
                            })
                                .components(|c| c)
                        }).await.unwrap();
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


#[poise::command(prefix_command)]
pub async fn register(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await.unwrap();
    Ok(())
}


pub fn forming_vec_to_show(material: Vec<Vec<String>>, urls: Vec<Vec<String>>) -> Vec<String> {
    let mut vec = vec![];
    for (m, u) in material.iter().cloned().zip(urls) {
        let title = url!(m.get(0).unwrap(), u.get(0).unwrap());
        let artist = m.get(2).unwrap().split(";").collect::<Vec<&str>>().join(", ");
        let album = m.get(1).unwrap();

        vec.push(format!("**{}** by **{}** on **{}**", title, artist, album))
    }

    vec
}


fn tmp(names: Vec<Vec<String>>, page: usize) -> CreateActionRow {
    CreateActionRow::default()
        .create_select_menu(|select| {
            select.custom_id("select_song")
                .min_values(1)
                .max_values(1)
                .placeholder("Choose your want a Song!")
                .options(|option| {
                    for (idx, name) in names.get(page).unwrap().iter().enumerate() {
                        option.create_option(|o| o.label(name).value(idx));
                    }
                    option
                })
        }).clone()
}

fn tmp_2((left, right): (bool, bool)) -> CreateActionRow {
    CreateActionRow::default()
        .create_button(|btn| {
            btn.custom_id("search_left")
                .label("<<")
                .style(ButtonStyle::Success)
                .disabled(left)
        }).create_button(|btn| {
            btn.custom_id("search_left_one")
                .label("<")
                .style(ButtonStyle::Success)
                .disabled(left)
        }).create_button(|btn| {
            btn.custom_id("search_right_one")
                .label(">")
                .style(ButtonStyle::Success)
                .disabled(right)
        }).create_button(|btn| {
            btn.custom_id("search_right")
                .label(">>")
                .style(ButtonStyle::Success)
                .disabled(right)
        }).clone()
}