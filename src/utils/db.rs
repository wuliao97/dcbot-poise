use anyhow::Result;


pub async fn get_pool() -> Result<sqlx::SqlitePool> {
    if let Err(why) = dotenv::dotenv() {
        eprintln!("{}", why);
    };
    let url = std::env::var("DATABASE_URL").unwrap();
    Ok(sqlx::SqlitePool::connect(url.as_str()).await.unwrap())
}

pub mod hsr {
    pub mod model {
        #[derive(Debug)]
        pub struct UserInfo {
            pub id: i64,
            pub ltuid: Option<String>,
            pub ltoken: Option<String>,
            pub starrail_id: Option<i64>,
        }

        pub struct RegiUserInfo {
            pub id: i64,
            pub ltuid: Option<String>,
            pub ltoken: Option<String>,
            pub starrail_id: Option<i64>,
        }
        impl RegiUserInfo {
            pub fn new(id: i64, ltuid: Option<String>, ltoken: Option<String>, uid: Option<i64>) -> Self {
                Self { id, ltuid, ltoken, starrail_id: uid }
            }
        }

        pub struct ParticleId {
            pub id: i64,
        }

        #[derive(Debug)]
        pub struct ParticleStarRailId {
            pub id: i64,
            pub starrail_id: Option<i64>,
        }

        pub struct Cookies {
            pub ltuid: Option<String>,
            pub ltoken: Option<String>,
        }
    }

    // pub enum StarRailQueryPattern {
    //     All,
    //     Contains,
    // }

    /// [`sqlx::query_as!`] which can't execute if SQL statements is not Literal
    // impl StarRailQueryPattern {
    //     fn q_to_str(&self) -> String {
    //         let result = match self {
    //             StarRailQueryPattern::All => "select * from users",
    //             StarRailQueryPattern::Contains => "select id form users",
    //         };
    //         result.to_string()
    //     }
    // }

    // pub async fn get_pool() -> anyhow::Result<sqlx::SqlitePool> {
    //     if let Err(why) = dotenv::dotenv() {
    //         eprintln!("{}", why);
    //     };
    //     let url = std::env::var("DATABASE_URL").unwrap();
    //     Ok(sqlx::SqlitePool::connect(url.as_str()).await.unwrap())
    // }

    pub async fn get_all(pool: &sqlx::sqlite::SqlitePool) -> anyhow::Result<Vec<model::UserInfo>> {
        Ok(sqlx::query_as!(model::UserInfo, "SELECT * FROM users")
            .fetch_all(pool)
            .await
            .unwrap())
    }

    pub async fn exists(pool: &sqlx::SqlitePool, id: i64) -> bool {
        match sqlx::query!("SELECT id FROM users where id = ?", id).fetch_one(pool).await {
            Ok(val) => true,
            Err(_) => false
        }
    }

    pub async fn get_particle(pool: &sqlx::SqlitePool, primary_key: i64) -> Option<model::ParticleStarRailId> {
        match sqlx::query_as!(model::ParticleStarRailId, "SELECT id, starrail_id FROM users WHERE id = ?", primary_key)
            .fetch_one(pool).await
        {
            Ok(val) => Some(val),
            Err(_) => None
        }

    }

    pub async fn insert_or_update(pool: &sqlx::SqlitePool, material: model::RegiUserInfo) -> anyhow::Result<sqlx::sqlite::SqliteQueryResult> {
        let id = material.id;
        Ok(
            sqlx::query!(
            "INSERT INTO users values (?, ?, ?, ?) ON CONFLICT (id) DO UPDATE SET ltuid = ?, ltoken = ?, starrail_id = ?",
            id, material.ltuid, material.ltoken, material.starrail_id, material.ltuid, material.ltoken, material.starrail_id
            )
                .execute(pool)
                .await
                .unwrap()
        )
    }

    // pub async fn delete_data(pool: &sqlx::SqlitePool, primary_key: i64) -> Result<sqlx::sqlite::SqliteQueryResult> {
    //     Ok(
    //         sqlx::query!("DELETE FROM users WHERE id = ?", primary_key)
    //         .execute(pool)
    //         .await
    //         .unwrap()
    //     )
    // }


    pub async fn get_partial(pool: &sqlx::SqlitePool) -> anyhow::Result<Vec<model::ParticleId>> {
        Ok(
            sqlx::query_as!(model::ParticleId, "select id from users")
                .fetch_all(pool)
                .await
                .unwrap()
        )
    }

    pub async fn get_values(pool: &sqlx::SqlitePool, primary_key: i64) -> (Option<String>, Option<String>) {
        let result = sqlx::query_as!(model::Cookies, "select ltuid, ltoken from users where id = ?", primary_key)
            .fetch_optional(pool)
            .await
            .unwrap();

        match result {
            Some(cookies) => {
                (cookies.ltuid, cookies.ltoken)
            },
            None => (None, None),
        }
    }
}

pub mod fe {
    pub mod model {
        #[derive(Debug)]
        pub struct FullQuestion {
            pub id: i64,
            pub title: String,
            pub choice_fst: Option<String>,
            pub choice_snd: Option<String>,
            pub choice_thd: Option<String>,
            pub choice_fth: Option<String>,
            pub right_answer: String,
            pub explain: Option<String>,
        }
        impl FullQuestion {
            pub fn extract_choices(&self) -> Vec<String> {
                vec![
                    self.choice_fst.clone(), self.choice_snd.clone(), self.choice_thd.clone(), self.choice_fth.clone()
                ].into_iter()
                    .filter(|i| i.is_some())
                    .map(|i| i.unwrap())
                    .collect()
            }
        }

        #[derive(Debug)]
        pub struct QuestionIdx {
            pub id: i64
        }
    }

    pub async fn insert_or_update(pool: &sqlx::SqlitePool, model: model::FullQuestion) -> anyhow::Result<sqlx::sqlite::SqliteQueryResult> {
        Ok(
            sqlx::query!("INSERT INTO fe_exam values (?, ?, ?, ?, ?, ?, ?, ?) \
                          ON CONFLICT (id) DO UPDATE SET \
                          title = ?, choice_fst = ?, choice_snd = ?, choice_thd = ?, choice_fth = ?, right_answer = ?, explain = ?;",
                model.id, model.title, model.choice_fst, model.choice_snd, model.choice_thd, model.choice_fth, model.right_answer, model.explain,
                model.title, model.choice_fst, model.choice_snd, model.choice_thd, model.choice_fth, model.right_answer, model.explain
            )
                .execute(pool)
                .await
                .unwrap()
        )
    }

    pub async fn get_one_by_key(pool: &sqlx::SqlitePool, key: &i64) -> anyhow::Result<model::FullQuestion> {
        Ok(
            sqlx::query_as!(model::FullQuestion, "SELECT * FROM fe_exam WHERE id = ?;", key)
                .fetch_one(pool)
                .await
                .unwrap()
        )
    }


    pub async fn get_all(pool: &sqlx::SqlitePool) -> anyhow::Result<Vec<model::FullQuestion>> {
        Ok(
            sqlx::query_as!(model::FullQuestion, "SELECT * FROM fe_exam;")
                .fetch_all(pool)
                .await?
        )
    }

    pub async fn get_total_num(pool: &sqlx::SqlitePool) -> anyhow::Result<usize> {
         let data = sqlx::query_as!(model::QuestionIdx, "SELECT id FROM fe_exam;")
             .fetch_all(pool)
             .await
             .unwrap();
        Ok(data.len())
    }
}
