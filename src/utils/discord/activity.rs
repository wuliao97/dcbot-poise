use poise::serenity_prelude::{ self as serenity, Activity, Presence, UserId};
use crate::utils::Context;
use crate::apis::spotify::{ExtractInfo, SpotifyAPI};
use crate::url_with_bold;
use crate::utils::constant::*;
use serenity::model::user::OnlineStatus;


pub struct SpotifyActivity {
    presence: Option<Presence>,
    activity: Option<Activity>,
    url: Vec<String>,
}


impl SpotifyActivity {
    #[must_use]
    pub fn new(ctx: Context<'_>, user_id: UserId) -> Self {
        let presence = ctx.guild().unwrap().presences.get(&user_id).cloned();

        Self {
            presence,
            activity: None,
            url: Vec::new(),
        }
    }

    pub async fn listening(&mut self) -> bool {
        return if self.presence.is_none() {
            false
        } else if self.presence.clone().unwrap().activities.is_empty() {
            false
        } else {
            let activity = self.presence.clone().unwrap().activities
                .iter()
                .cloned()
                .filter(|a| a.name.eq("Spotify"))
                .next();

            return if activity.clone().is_some() {
                self.activity = activity;
                let track = SpotifyAPI::new().await.track(self.id().as_str()).await.unwrap();
                let urls = ExtractInfo::url_from_track(track);
                self.url = urls;

                true
            } else {
                false
            };
        };
    }

    #[inline]
    fn act(&self) -> Activity {
        self.activity.clone().unwrap()
    }

    pub fn get_info(&self, info_type: InfoType) -> (String, String, String) {
        return match info_type {
            InfoType::WithUrl => {
                (
                    self.title_with_url(),
                    self.artist_with_url(),
                    self.album_with_url(),
                )
            }
            InfoType::WithoutUrl => {
                (
                    self.title(),
                    self.artist().join(", "),
                    self.album(),
                )
            }
        };
    }


    #[inline]
    fn title(&self) -> String {
        self.activity.clone().unwrap().details.unwrap()
    }

    #[inline]
    fn artist(&self) -> Vec<String> {
        self.activity.clone().unwrap().state.unwrap()
            .split(";")
            .map(|a| a.to_string())
            .collect::<Vec<String>>()
    }

    #[inline]
    fn album(&self) -> String {
        self.activity.clone().unwrap().assets.unwrap().large_text.unwrap()
    }

    #[inline]
    pub fn id(&self) -> String {
        self.activity.clone().unwrap().sync_id.unwrap()
    }


    pub fn title_with_url(&self) -> String {
        let title = self.title();
        let url = self.get_track_url();
        url_with_bold!(title, url)
    }

    pub fn artist_with_url(&self) -> String {
        let artist_url = &self.url[2..];
        let artist = self.artist();

        if artist_url.len() != artist.len() {
            let _artist = artist.get(0).unwrap();
            let _url = artist_url.get(0).unwrap();
            return url_with_bold!(_artist, _url);
        }

        let mut base = Vec::new();
        for (_artist, _url) in artist.iter().zip(artist_url) {
            base.push(url_with_bold!(_artist, _url));
        }

        base.join(", ")
    }

    pub fn album_with_url(&self) -> String {
        let album = self.album();
        let url = self.url.get(1).unwrap();
        url_with_bold!(album, url)
    }

    pub fn get_track_url(&self) -> String {
        let url = self.id();
        format!("{}{}", SPOTIFY_TRACK_URL, url)
    }

    pub fn get_cover_url(&self) -> String {
        let material = self.act();
        let cover_literal = material.assets.unwrap().large_image.unwrap();
        let i = cover_literal.char_indices().nth(8).unwrap().0;
        let cover = &cover_literal[i..];

        format!("https://i.scdn.co/image/{}", cover.to_string())
    }

    pub fn start(&self) -> u64 {
        self.act().timestamps.unwrap().start.unwrap() / 1000
    }

    pub fn end(&self) -> u64 {
        self.act().timestamps.unwrap().end.unwrap() / 1000
    }

    pub fn duration(&self) -> u64 {
        self.end() - self.start()
    }

    pub fn format_time(&self) -> String {
        let duration = self.duration() as i64;
        ExtractInfo::format_time(duration)
    }

    pub fn get_color(&self) -> u32 {
        SPOTIFY_GREEN
    }
}


#[allow(dead_code)]
pub enum InfoType {
    WithUrl,
    WithoutUrl,
}


pub async fn global_presence(ctx: &serenity::Context, message: String) {
    ctx.set_presence(
        Some(Activity::playing(message)),
        OnlineStatus::Online
    ).await;
}