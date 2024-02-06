use std::fs::File;
use std::io;
use std::io::BufReader;

use poise::CreateReply;
use serenity::all::{CreateEmbed, CreateEmbedFooter};
use crate::apis::sellrank::SellRankingClient;
use crate::utils::{Context, Error};


#[poise::command(
slash_command,
subcommands("sell_ranking", "calc_exp"),
subcommand_required,
rename = "dokkan"
)]
pub async fn dokkanbattle(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[poise::command(
slash_command,
rename = "sell-ranking-iphone"
)]
pub async fn sell_ranking(ctx: Context<'_>) -> Result<(), Error> {
    let client = SellRankingClient::new();
    let apps = client.get_ranking().await.unwrap();
    let mut desc = String::new();
    let mut footer_text: Option<String> = None;

    for (idx, app) in apps.feed.app.iter().enumerate() {
        let idx = idx + 1;

        let desc_ = if app.name.label.contains("ドッカン") {
            footer_text = Some(format!("現在: {} 位", idx));
            format!("**{}** -  **{}**\n", idx, &app.name.label)
        } else {
            format!("**{}** - {}\n", idx, &app.name.label)
        };
        desc.push_str(&desc_);
    }

    let text = footer_text.unwrap_or(String::from("現在: 圏外"));
    let embed = CreateEmbed::default()
        .title("iOS sells ranking")
        .description(desc)
        .footer(CreateEmbedFooter::new(text));

    ctx.send(CreateReply::default().embed(embed))
        .await?;

    Ok(())
}


// #[poise::command(
//     slash_command,
//     rename = "calc-exp"
// )]
// pub async fn calc_exp(
//     ctx: Context<'_>,
//
//     #[max = 999]
//     #[min = 1]
//     rank: i32,
//
//     #[min = 1]
//     rank_now_value: i32,
// ) -> Result<(), Error> {
//     let mut embed = CreateEmbed::default();
//
//     const RANK_PATH: &str = "resources/dokkan/natural_rank.json";
//     const EXP_PATH: &str = "resources/dokkan/exp.json";
//
//
//     let rank_buf = BufReader::new(File::open(RANK_PATH)?);
//     let exp_buf = BufReader::new(File::open(EXP_PATH)?);
//
//     use super::model::*;
//     // let rank_list = serde_json::from_reader::<BufReader<File>, Devide>(rank_buf);
//     let exp_list = serde_json::from_reader::<BufReader<File>, Story>(exp_buf);
//
//     // dbg!(rank_list);
//     dbg!(exp_list);
//
//     Ok(())
//
// }