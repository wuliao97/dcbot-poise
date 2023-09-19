use rspotify::{Credentials, ClientCredsSpotify, ClientResult};
use rspotify::clients::BaseClient;
use rspotify::model::{idtypes, FullTrack, SearchResult, SearchType};



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

    pub async fn search(&self, q: &str, search_type: SearchType) -> ClientResult<SearchResult> {
        self.client.search(q, search_type, None, None, Some(50), None)
            .await
    }

    pub async fn track(&self, track_id: &str) -> ClientResult<FullTrack> {
        let id = idtypes::TrackId::from_id(track_id).unwrap();
        self.client.track(id)
            .await
    }
}


pub struct ExtractInfo {
    material: FullTrack,
    url: Option<Vec<String>>,
}


impl ExtractInfo {
    pub fn new(material: FullTrack) -> Self {
        Self {
            material,
            url: None
        }
    }

    pub fn url_from_track(&self) -> Vec<String> {
        let mut urls = Vec::new();

        urls.push(self.material.external_urls.get("spotify").unwrap().clone());
        urls.push(self.material.artists
            .iter()
            .map(|a| a.external_urls.get("spotify").unwrap().clone())
            .collect::<Vec<String>>()
            .join(";")
        );
        urls.push(self.material.album.external_urls.get("spotify").unwrap().clone());

        urls
    }



}