use std::collections::HashMap;
use reqwest::{Client, ClientBuilder};
use serde::Deserialize;

pub struct SellRankingClient {
    url: super::iphone::Url,
    client: Client,
}
impl SellRankingClient {
    pub fn new() -> Self {
        let client = Client::builder().build().unwrap();
        Self {
            url: super::iphone::Url::default(),
            client,
        }
    }

    pub async fn get_ranking(&self) -> anyhow::Result<SellRankingIphone> {
        let url = self.url.to_string() ;
        let result = self.client.get(&url).send().await.unwrap();
        Ok(result.json::<SellRankingIphone>().await.unwrap())
    }

}


#[derive(Debug, Deserialize)]
pub struct SellRankingIphone {
    pub feed: Feed,
}

#[derive(Debug, Deserialize)]
pub struct Feed {
    pub author: Author,
    #[serde(rename = "entry")]
    pub app: Vec<App>,
}

#[derive(Debug, Deserialize)]
pub struct Author {
    pub name: Label,
    pub uri: Label,
}

#[derive(Debug, Deserialize, Default)]
pub struct Label {
    pub label: String,
}


#[derive(Debug, Deserialize)]
pub struct App {
    #[serde(rename = "im:name")]
    pub name: Label,
    #[serde(rename = "im:image")]
    pub images: Vec<Image>,
    #[serde(skip)]
    pub summary: Summary,
    #[serde(rename = "im:price")]
    pub price: Price,
    #[serde(rename = "im:contentType")]
    pub content_type: ContentType,
    pub rights: Right,
    pub title: Title,
    #[serde(skip)]
    pub link: Vec<Link>,
    #[serde(skip)]
    pub id: Id,
    #[serde(rename = "im:artist")]
    #[serde(skip)]
    pub artist: Artist,
    #[serde(rename = "category")]
    #[serde(skip)]
    pub category: Category,
    #[serde(rename = "im:releaseDate")]
    pub release_date: ReleaseDate
}

#[derive(Debug, Deserialize)]
pub struct Image {
    pub label: String,
    pub attributes: ImageAttributes,
}

#[derive(Debug, Deserialize)]
pub struct ImageAttributes {
    pub height: String,
}

#[derive(Debug, Deserialize, Default)]
pub struct Summary {
    pub label: Label,
}

#[derive(Debug, Deserialize)]
pub struct Price {
    pub label: String,
    pub attributes: Attributes
}

#[derive(Debug, Deserialize)]
pub struct Attributes {
    pub amount: String,
    pub currency: String,
}

#[derive(Debug, Deserialize)]
pub struct ContentType {
    pub attributes: ContentAttributes
}

#[derive(Debug, Deserialize)]
pub struct ContentAttributes {
    pub term: String,
    pub label: String,
}

#[derive(Debug, Deserialize)]
pub struct Right {
    pub label: String,
}

#[derive(Debug, Deserialize)]
pub struct Title {
    pub label: String,
}

#[derive(Debug, Deserialize, Default)]
pub struct Link;

#[derive(Debug, Deserialize, Default)]
pub struct Id;

#[derive(Debug, Deserialize, Default)]
pub struct Artist {
    pub label: String,
    pub attributes: ArtistAttributes
}

#[derive(Debug, Deserialize, Default)]
pub struct ArtistAttributes {
    pub href: String
}

#[derive(Debug, Deserialize, Default)]
pub struct Category;

#[derive(Debug, Deserialize)]
pub struct ReleaseDate {
    pub label: String,
    pub attributes: ReleaseDateAttributes,
}

#[derive(Debug, Deserialize)]
pub struct ReleaseDateAttributes {
    pub label: String
}

// #[derive(Debug, Deserialize)]
// pub struct SellRankingIphone {
//
// }

