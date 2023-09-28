use rspotify::{Credentials, ClientCredsSpotify, ClientResult};
use rspotify::clients::BaseClient;
use rspotify::model::{idtypes, FullTrack, SearchResult, SearchType, Market};
use crate::url;


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

    pub fn general_info(&self, index: usize) -> Vec<String> {
        let mut base: Vec<String> = vec![];
        match self.material.clone() {
            SearchResult::Albums(_info) => {}
            SearchResult::Artists(_info) => {}
            SearchResult::Tracks(info) => {
                let item = info.items.get(index).unwrap();
                let title = url!(item.name, &item.external_urls["spotify"]);
                base.push(title);
                let artist = item.artists.iter()
                        .map(|artist| url!(artist.name, &artist.external_urls["spotify"]))
                        .collect::<Vec<String>>()
                        .join(", ");
                base.push(artist);
                let album = url!(item.album.name, item.album.external_urls["spotify"]);
                base.push(album);
            }
            _ => {}
        }
        base
    }

    pub fn sub_info(&self, index: usize) -> Vec<String> {
        let mut base: Vec<String> = vec![];

        match self.material.clone() {
            SearchResult::Albums(_info) => {}
            SearchResult::Artists(_info) => {}
            SearchResult::Tracks(info) => {
                let item = info.items.get(index).unwrap();
                base.push(item.duration.num_seconds().to_string());
                base.push(item.album.images.get(0).unwrap().url.to_string())
            }
            _ => {}
        }

        base

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
                base.push(info.items.iter().map(|item| item.name.to_owned()).collect());
                base
            }
            SearchResult::Artists(info) => {
                base.push(info.items.iter().map(|item| item.name.to_owned()).collect());
                base
            }
            SearchResult::Tracks(info) => {
                for item in info.items {
                    let mut set = vec![];
                    set.push(item.name.to_owned());
                    set.push(item.album.name.to_owned());
                    set.push(item.artists.iter().map(|artist| artist.name.to_owned()).collect::<Vec<String>>().join(";"));
                    base.push(set);
                };
                base
            }
            _ => {base}
        }
    }

    pub fn urls(&self) -> Vec<Vec<String>> {
        let mut base: Vec<Vec<String>> = vec![];

        match self.material.clone() {
            SearchResult::Albums(_nfo) => {base}
            SearchResult::Artists(_info) => {base}
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


    pub fn extract_vec(&self, material: Vec<String>, first: usize, last: usize) -> Vec<String> {
        let tmp: &[String] = &material[first..=last];
        tmp.to_vec()
    }

    pub fn distinction_vec(&self, material: Vec<String>, max_value: usize) -> Vec<Vec<String>> {
        let mut vec: Vec<Vec<String>> = Vec::new();
        let limit = material.len();
        let max_count = (limit as f32 / max_value as f32).ceil() as usize;

        for idx in 0..max_count {
            let (first, last) = {
                let index = idx;
                let first = &index * max_value;
                let last = if first + max_value > material.len() && material.len() % max_value != 0 {
                    let tmp = limit.clone();
                    tmp - 1
                } else {
                    (index + 1) * max_value - 1
                };
                (first, last)
            };
            vec.push(self.extract_vec(material.to_vec(), first, last));
        }
        vec
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[tokio::test]
//     async fn it_works() {
//         let api = SpotifyAPI::new().await;
//
//         let result = api.search("king gnu", SearchType::Track, None, 10).await.unwrap();
//         let info = ExtractInfo::new(result);
//
//         dbg!(info.surface());
//     }
// }