use poise::serenity_prelude as serenity;
use serenity::CreateEmbed;

use crate::utils::constant::*;


// pub fn spotify_default<D: ToString>(user: Option<&serenity::User>, description: Option<D>) -> &mut CreateEmbed {
//     let mut embed = CreateEmbed::default()
//         .color(SPOTIFY_GREEN);
//
//
//     if let Some(description) = description {
//         embed.description(description.to_string());
//     }
//
//     if let Some(user) = user {
//         let url = user.avatar_url().unwrap_or(user.default_avatar_url());
//         let name = format!("{} is listening", user.name);
//         embed.author(|author| author.icon_url(url).name(name))
//     };
//
//     embed
// }


pub fn spotify_search_embed(description: &String, user: &serenity::User, (first, last): (usize, usize)) -> CreateEmbed {
    CreateEmbed::default()
        .author(|author| {
            let url = user.avatar_url().unwrap_or(user.default_avatar_url());
            let name = format!("{} is searching", user.name);
            author.icon_url(url).name(name)
        })
        .color(SPOTIFY_GREEN)
        .description(description)
        .footer(|footer| footer.text(format!("Page: {}/{}", first, last))).clone()
}