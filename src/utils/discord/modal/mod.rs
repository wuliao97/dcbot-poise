pub mod models {
    #[derive(poise::Modal, Debug)]
    pub struct StarRailRegister {
        pub ltuid: String,
        pub ltoken: String,
        pub starrail_id: String,
    }
}