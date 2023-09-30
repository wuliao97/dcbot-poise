use poise::serenity_prelude as serenity;
use serenity::builder::CreateEmbed;
use crate::utils::discord::embed::spotify_search_embed;


#[derive(Debug)]
pub struct SpotifyHandler {
    material: Vec<String>,
}

impl SpotifyHandler {
    #[must_use]
    pub fn new(material: Vec<String>) -> Self {
        Self {
            material,
        }
    }


    pub fn decorate_embed(&self, material: &Vec<String>, user: &serenity::User, (first, last): (usize, usize)) -> CreateEmbed {
        let description = material.join("\n");
        spotify_search_embed(&description, &user, (first, last))
    }

    pub fn decorate_embeds(&self, material: Vec<Vec<String>>, user: &serenity::User) -> Vec<CreateEmbed> {
        let mut embeds: Vec<CreateEmbed> = vec![];
        let max_count = material.len();

        for idx in 0..max_count {
            let tmp = &material.get(idx).unwrap().to_owned();
            embeds.push(self.decorate_embed(&tmp, &user, (idx + 1, max_count)).clone())
        }
        embeds
    }

}
