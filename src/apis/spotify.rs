use chrono::NaiveDateTime;
use rspotify::{Credentials, ClientCredsSpotify, ClientResult};
use rspotify::clients::BaseClient;
use rspotify::model::{idtypes, FullTrack, SearchResult, SearchType, Market};
use serenity::builder::CreateEmbed;
use crate::{quote, url_with_bold};
use crate::utils::constant::SPOTIFY_GREEN;


pub struct SpotifyAPI {
    client: ClientCredsSpotify,
}

impl SpotifyAPI {
    #[must_use]
    pub async fn new() -> Self {
        if let Err(why) = dotenv::dotenv() {
            log::error!("Unable find .env file: {:?}", why);
        }
        let creds = Credentials::from_env().unwrap();
        let client = ClientCredsSpotify::new(creds);
        client.request_token().await.unwrap();

        Self { client }
    }

    pub async fn search(&self, q: &str, search_type: SearchType, market: Option<Market>, limit: u32) -> ClientResult<SearchResult> {
        self.client.search(q, search_type, market, None, Some(limit), None)
            .await
    }

    pub async fn track(&self, track_id: &str) -> ClientResult<FullTrack> {
        let id = idtypes::TrackId::from_id(track_id).unwrap();
        self.client.track(id)
            .await
    }
}


#[allow(dead_code)]
pub struct ExtractInfo {
    material: SearchResult,
}

impl ExtractInfo {
    pub fn new(material: SearchResult) -> Self {
        Self { material }
    }


    pub fn sub_info(&self, index: usize) -> Vec<String> {
        let mut base: Vec<String> = vec![];

        match self.material.clone() {
            SearchResult::Albums(info) => {
                let item = info.items.get(index).unwrap().clone();
                base.push(item.images.get(0).unwrap().url.to_string());
                base.push(item.release_date.unwrap().clone());
            }
            SearchResult::Artists(info) => {
                let item = info.items.get(index).unwrap();
                base.push(item.images.get(0).unwrap().url.to_string());
                base.push(item.genres.join(", "));
                base.push(item.followers.total.to_string());
            }
            SearchResult::Tracks(info) => {
                let item = info.items.get(index).unwrap();
                base.push(item.album.images.get(0).unwrap().url.to_string());
                base.push(ExtractInfo::format_time(item.duration.num_seconds()));
            }
            _ => {}
        }
        base
    }


    pub fn vec_to_show(&self) -> Vec<String> {
        let mut base: Vec<String> = vec![];

        match self.material.clone() {
            SearchResult::Albums(info) => {
                for item in info.items {
                    let album_with_url = {
                        let name = &item.name;
                        let url = &item.external_urls["spotify"];
                        url_with_bold!(name, url)
                    };
                    let artist_name = item.artists
                        .iter()
                        .map(|artist| format!("**{}**", artist.name))
                        .collect::<Vec<String>>()
                        .join(", ");
                    base.push(format!("**{}** by {}", album_with_url, artist_name));
                }
            }
            SearchResult::Artists(info) => {
                for item in info.items {
                    let artist_with_url = {
                        let name = &item.name;
                        let url = &item.external_urls["spotify"];
                        url_with_bold!(name, url)
                    };
                    base.push(format!("**{}**", artist_with_url));
                }
            }
            SearchResult::Tracks(info) => {
                for item in info.items {
                    let title_with_url = {
                        let name = &item.name;
                        let url = &item.external_urls["spotify"];
                        url_with_bold!(name, url)
                    };
                    let artist = item.artists
                        .iter()
                        .map(|artist| format!("**{}**", artist.name))
                        .collect::<Vec<String>>()
                        .join(", ");
                    let album = item.album.name;
                    base.push(format!("**{}** by {} on **{}**", title_with_url, artist, album));
                }
            }
            _ => {}
        }

        base
    }


    pub fn to_show_with_embed(&self, index: usize) -> CreateEmbed {
        let mut embed = CreateEmbed::default()
            .title("Search Result")
            .color(SPOTIFY_GREEN)
            .clone();

        match self.material.clone() {
            SearchResult::Albums(info) => {
                let item = info.items.get(index).unwrap();
                let artist_with_url = &item.artists
                    .iter()
                    .map(|artist| url_with_bold!(&artist.name, &artist.external_urls["spotify"]))
                    .collect::<Vec<String>>()
                    .join(", ");
                dbg!(&item.album_group);

                embed.title(item.name.to_string())
                    .url(item.external_urls["spotify"].to_string())
                    .field("by", quote!(artist_with_url), false)
                    .thumbnail(item.images.get(0).unwrap().url.to_string())
                    .footer(|f| f.text(format!("Released: {}", item.release_date.clone().unwrap())));
            }
            SearchResult::Artists(info) => {
                let item = info.items.get(index).unwrap();
                let genres = item.genres
                    .iter()
                    .map(|genre| format!("**{}**", genre))
                    .collect::<Vec<String>>()
                    .join(", ");

                embed.title(&item.name)
                    .url(&item.external_urls["spotify"])
                    .description(quote!(genres))
                    .thumbnail(item.images.get(0).unwrap().url.to_string())
                    .footer(|f| f.text(format!("Follower: {}", &item.followers.total)));
            }
            SearchResult::Tracks(info) => {
                let item = info.items.get(index).unwrap();

                let title = url_with_bold!(&item.name, &item.external_urls["spotify"]);
                let artist = item.artists.iter()
                    .map(|artist| url_with_bold!(artist.name, &artist.external_urls["spotify"]))
                    .collect::<Vec<String>>()
                    .join(", ");
                let album = url_with_bold!(item.album.name.clone(), item.album.external_urls["spotify"].clone());

                embed.thumbnail(&item.album.images.get(0).unwrap().url.to_string())
                    .field("title", quote!(title), false)
                    .field("by", quote!(artist), false)
                    .field("on", quote!(album), false)
                    .footer(|f| f.text(format!("Time: {}", ExtractInfo::format_time(item.duration.num_seconds()))))
                ;
            }
            _ => {}
        }
        embed
    }



    // SearchResult::Albums(info) => {}
    // SearchResult::Artists(info) => {}
    // SearchResult::Tracks(info) => {}
    // _ => {}


    pub fn names(&self) -> Vec<String> {
         match &self.material {
            SearchResult::Albums(info) => {
                info.items
                    .iter()
                    .map(|item| item.name.to_owned())
                    .collect::<Vec<String>>()
            }
            SearchResult::Artists(info) => {
                info.items
                    .iter()
                    .map(|item| item.name.to_owned())
                    .collect::<Vec<String>>()
            }
            SearchResult::Tracks(info) => {
                info.items
                    .iter()
                    .map(|item| item.name.to_owned())
                    .collect::<Vec<String>>()
            }
            _ => {vec![]}
        }
    }

    pub fn surface(&self) -> Vec<Vec<String>> {
        let mut base: Vec<Vec<String>> = vec![];

        match self.material.clone() {
            SearchResult::Albums(info) => {
                for item in info.items {
                    let mut set = vec![];
                    set.push(item.name);
                    base.push(set);
                }
            }
            SearchResult::Artists(info) => {
                for item in info.items {
                    let set = vec![item.name];
                    base.push(set);
                }
            }
            SearchResult::Tracks(info) => {
                for item in info.items {
                    let mut set = vec![];
                    set.push(item.name.to_owned());
                    set.push(item.album.name.to_owned());
                    set.push(item.artists.iter().map(|artist| artist.name.to_owned()).collect::<Vec<String>>().join(";"));
                    base.push(set);
                };

            }
            _ => {}
        }
        base
    }

    pub fn urls(&self) -> Vec<Vec<String>> {
        let mut base: Vec<Vec<String>> = vec![];

        match self.material.clone() {
            SearchResult::Albums(info) => {
                for item in info.items {
                    let set = vec![item.external_urls["spotify"].to_owned()];
                    base.push(set);
                }
                base
            }
            SearchResult::Artists(info) => {
                for item in info.items {
                    let set = vec![item.external_urls["spotify"].to_owned()];
                    base.push(set);
                }
                base
            }
            SearchResult::Tracks(info) => {
                for item in info.items {
                    let mut set = vec![];
                    set.push(item.external_urls["spotify"].to_owned());
                    set.push(item.album.external_urls["spotify"].to_owned());
                    set.push(item.artists.iter().map(|artist| artist.external_urls["spotify"].to_owned()).collect::<Vec<String>>().join(";"));
                    base.push(set);
                };
                base
            }
            _ => {base}
        }
    }

    pub fn url_from_track(material: FullTrack) -> Vec<String> {
        let mut urls = Vec::new();
        urls.push(material.external_urls.get("spotify").unwrap().clone());
        urls.push(material.album.external_urls.get("spotify").unwrap().clone());
        urls.push(material.artists
            .iter()
            .map(|a| a.external_urls.get("spotify").unwrap().clone())
            .collect::<Vec<String>>()
            .join(";")
        );
        urls
    }


    pub fn format_time(duration: i64) -> String {
        let time = NaiveDateTime::from_timestamp_opt(duration, 0).unwrap();
        let length = if duration < 60 {
            time.format("%S")
        } else if duration > 3600 {
            time.format("%H:%M:%S")
        } else {
            time.format("%M:%S")
        };
        format!("Time: {}", length).to_string()
    }
}
