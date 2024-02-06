use tokio::sync::Mutex;


pub struct Data {
    pub pool: Mutex<sqlx::SqlitePool>,
    pub mihoyo: Mutex<miHoYo_API::client::Client>
}

