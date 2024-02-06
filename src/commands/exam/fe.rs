use rand::Rng;
use poise::CreateReply;
use rand::distributions::Distribution;
use rand::distributions::Uniform;
use serenity::all::{CreateEmbed, CreateEmbedFooter};
use crate::quote;

use crate::utils::{Context, Error};
use crate::utils::db::{
    get_pool,
    fe::{
        model::FullQuestion,
        insert_or_update,
        get_total_num,
        get_all,
        get_one_by_key
    }
};
use crate::utils::db::fe::model;
use crate::utils::discord::button::EZButton;

#[poise::command(
    slash_command,
    subcommand_required,
    subcommands("help", "exam", "add")
)]
pub async fn fe(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}


#[poise::command(
    slash_command
)]
pub async fn help(ctx: Context<'_>) -> Result<(), Error> {
    let command_id = " </fe exam:1194849598360006656>";
    let url = "https://www.google.com/search?q=基本情報技術者試験%E3%80%80とは";
    let embed = CreateEmbed::new()
        .title("feコマンドについて")
        .description(format!(">>> 基本情報技術者試験（FE）は、経済産業省が認定する国家資格で、高度IT人材となるために必要な知識や技能、実践的な活用能力を認定する試験です。\n([もっと詳しく]({}))\n\n\
            情報処理推進機構（IPA）が実施・運営するこの試験は、システムエンジニアやプログラマーの基礎スキルがあることを証明できます。IT関連の業種に従事している方、またはこれからIT業界で働きたいと考えている方を対象としています。\n\n\
            基本情報技術者試験では、情報処理の分野の他にも、マネジメント分野や経営・会計分野が出題されるため、IT業界で活躍するために必要な知識も身につけられます。\
            令和4年度（2022年度）には年間10万人以上が受験しており、IT業界への登竜門としても人気があります。\n\n\
            当コマンドはFEの過去問を四択で出し、それらを使用者が回答するといった機能です。",
            url))
        .field("Try It!", quote!(command_id), true)
        .footer(CreateEmbedFooter::new("※現在プレビュー版 | v.0.0"));

    ctx.send(CreateReply::default().embed(embed))
        .await?;

    Ok(())
}


// 以下駄文

/// FE過去問: /fe help
#[poise::command(
    slash_command
)]
pub async fn exam(
    ctx: Context<'_>,
    #[description = "問題文指定番号"]
    #[min = 0]
    number: Option<i64>
) -> Result<(), Error> {
    let pool = get_pool().await?;

    let number = number.unwrap_or({
        let data_size = get_total_num(&pool).await.unwrap();
        let mut between = Uniform::from(0..data_size);
        between.sample(&mut rand::thread_rng()) as i64
    });

    let data = get_one_by_key(&pool, &number).await?;
    let material = data.extract_choices();
    let vec: Vec<(String, String, bool)> = material.iter()
        .enumerate()
        .map(|(idx, d)| {
            let idx = (idx + 1) as i64;
            (format!("{}.", idx), quote!(d), false)
        }).collect();

    let emb = CreateEmbed::new()
        .description(data.title)
        .fields(vec);

    let mut ez_btn = EZButton::new();
    let custom_ids: Vec<String> =  (0..material.len()).into_iter()
        .map(|m| {
            let id = format!("ez_btn_{}", m);
            ez_btn.add_btn(&id, &(m + 1).to_string(), None);
            id.to_string()
        }).collect();

    let interaction = ctx.send(
        CreateReply::default()
            .embed(emb)
            .components(ez_btn.build())
    ).await?;

    if let Some(interaction) = interaction
        .message()
        .await?
        .await_component_interactions(ctx.serenity_context())
        .author_id(ctx.author().id)
        .timeout(std::time::Duration::from_secs(180))
        .await
    {
        let c_id = interaction.data.custom_id;
        if let Some(id) = custom_ids.iter().filter(|id| id.eq(&&c_id)).next() {

        }
    }


    Ok(())
}

#[poise::command(
    prefix_command,
    hide_in_help,
    owners_only,
)]
pub async fn add(
    ctx: Context<'_>,
    title: String,
    choice_fst: String,
    choice_snd: String,
    choice_thd: String,
    choice_fth: String,
    right_answer: String,
    explain: String
) -> Result<(), Error> {
    let pool = get_pool().await?;

    let data = get_all(&pool).await?;
    let id = data.len() as i64;

    let model = FullQuestion {
        id,
        title,
        choice_fst: Some(choice_fst),
        choice_snd: Some(choice_snd),
        choice_thd: Some(choice_thd),
        choice_fth: Some(choice_fth),
        right_answer,
        explain: Some(explain),
    };

    let mut emb = CreateEmbed::new()
        .title("Success!");

    if data.iter().filter(|d| d.title.eq(&model.title)).next().is_none() {
        match insert_or_update(&pool, model).await {
            Ok(result) => {
                let re_data = get_one_by_key(&pool, &id)
                    .await?;

                let vec = vec![
                    ("last insert rowid", quote!(result.last_insert_rowid()), true),
                    ("rows affected", quote!(result.rows_affected()), true),
                ];

                emb = emb.fields(vec)
                    .description(
                        format!(
                            "id = {}\n\ntitle = {}\n\nchoices = [\n\t{:?},\n\t{:?},\n\t{:?},\n\t{:?}\n]\n\nright_answer = {}\n\nexplain = {:?}",
                            re_data.id, re_data.title,
                            re_data.choice_fst, re_data.choice_snd, re_data.choice_thd, re_data.choice_fth,
                            re_data.right_answer, re_data.explain
                    ));
            }
            Err(err) => emb = emb.description(quote!(err.to_string())),
        }
    }

    ctx.send(CreateReply::default()
        .embed(emb))
        .await?;

    Ok(())
}