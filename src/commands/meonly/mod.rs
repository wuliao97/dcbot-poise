use poise::CreateReply;
use serenity::all::{CreateEmbed, CreateEmbedFooter};

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
    let cache = ctx.cache();
    let guilds = cache.guilds();

    let guild_sets = guilds
        .iter()
        .map(|guild| format!("{} - {}", guild.to_string(), guild.name(&ctx.cache()).unwrap()))
        .collect::<Vec<String>>()
        .join("\n");

    ctx.send(CreateReply::default()
        .embed(CreateEmbed::default().description(guild_sets)
        .footer(CreateEmbedFooter::new(format!("Total Servers: {}", guilds.len()))))
    )
        .await.unwrap();


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
    // let guild = ctx.cache().guild(GuildId(NonZeroU64::try_from(id).unwrap())).unwrap_or(ctx.guild().unwrap());
    // let cache = ctx.cache();
    //
    // let owner = quote!(&guild.owner_id.mention());
    // let roles = quote!(format!("**{}**", &guild.roles.len()));
    // let creation = {
    //     let date = &ctx.guild_id().unwrap().created_at().timestamp();
    //     quote!(format!("<t:{}:R>", date))
    // };
    // let members = {
    //     let members = &guild.members.values().cloned().collect::<Vec<Member>>();
    //     let bots = members.iter().cloned().filter(|m| m.user.bot).count();
    //     let users = (members.len() - 1) - bots;
    //     quote!(format!("**{}** User | **{}** Bot", users, bots))
    // };
    // let channels = {
    //     let channels: Vec<_> = guild.channels.values().cloned().filter(|c| c.clone().guild(cache).is_some()).collect();
    //     let texts = channels.iter().cloned().filter(|c| c.clone().kind.name().eq("text")).count();
    //     let voices = channels.iter().cloned().filter(|c| c.clone().kind.name().eq("voice")).count();
    //     quote!(format!("**{}** Text | **{}** Voice", texts, voices))
    // };
    // let emojis = {
    //     let emojis = &guild.emojis.len();
    //     let stickers = &guild.stickers.len();
    //     quote!(format!("**{}** Emoji | **{}** Sticker", emojis, stickers))
    // };
    // let urls: Option<String> = {
    //     let mut urls: Vec<String> = Vec::new();
    //
    //     if let Some(icon_url) = guild.icon_url().clone() {
    //         urls.push(url!("Icon", icon_url))
    //     };
    //
    //     if let Some(banner_url) = guild.banner_url().clone() {
    //         urls.push(url!("Banner", banner_url))
    //     };
    //
    //     if let Some(splash_url) = guild.splash_url().clone() {
    //         urls.push(url!("Splash", splash_url))
    //     };
    //
    //     if !urls.is_empty(){
    //         Some(urls.join(", "))
    //     } else {
    //         None
    //     }
    // };
    //
    // let mut embed = CreateEmbed::default()
    //     .description(format!("**{}**'s info", &guild.name))
    //     .field("Owner", owner, true)
    //     .field("Role", roles, true)
    //     .field("Creation", creation, true)
    //     .field("Member", members, true)
    //     .field("Channel", channels, true)
    //     .field("Emoji", emojis, true);
    //
    // if let Some(urls) = urls {
    //     embed = embed.field("URLs", quote!(urls), false);
    // }
    //
    // ctx.send(CreateReply::default().embed(embed.clone())).await.unwrap();

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
    // let guild = ctx.cache().guild(GuildId(id_u64!(id))).unwrap_or(ctx.guild().unwrap());
    //
    // let material = guild.members.values().cloned().collect::<Vec<Member>>();
    // let member_list = material
    //     .iter()
    //     .map(|member| format!("{} - {}", member.user.id, member.display_name()))
    //     .collect::<Vec<String>>()
    //     .join("\n");
    //
    // ctx.send(CreateReply::default().embed(CreateEmbed::default().description(member_list))).await.unwrap();

    Ok(())
}