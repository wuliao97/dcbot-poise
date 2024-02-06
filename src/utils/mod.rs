pub mod constant;
pub mod collection;
pub mod db;
pub mod discord;
pub mod spotify_handler;
pub mod image;

use crate::data::Data;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;


#[allow(dead_code)]
pub fn cut_string<T: ToString>(material: T, number: usize) -> String {
    let name = material.to_string();
    let i = name.char_indices().nth(number).unwrap().0;
    format!("{}..", &name[..i])
}