use image::DynamicImage;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Debug)]
pub struct AnilistEntry {
    #[serde(rename = "data")]
    pub data: Data,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    #[serde(rename = "Media")]
    pub media: Option<Media>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Media {
    #[serde(rename = "id")]
    pub id: Option<u32>,

    #[serde(rename = "title")]
    pub title: Option<Title>,

    #[serde(rename = "format")]
    pub format: Option<String>,

    #[serde(rename = "genres")]
    pub genres: Option<Vec<String>>,

    #[serde(rename = "coverImage")]
    pub cover_image: Option<CoverImage>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Title {
    #[serde(rename = "romaji")]
    pub romaji: Option<String>,

    #[serde(rename = "english")]
    pub english: Option<String>,

    #[serde(rename = "native")]
    pub native: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CoverImage {
    #[serde(rename = "medium")]
    pub img: Option<String>,
}

const QUERY: &str = "
query ($id: Int){
    Media (id: $id) {
      id
      title {
        romaji
        english
        native
      }
      format
      genres
      coverImage {
        medium
      }
    }
}
";

pub struct AnilistClient {
    client: Client,
}

impl AnilistClient {
    pub fn new() -> AnilistClient {
        Self {
            client: Client::new(),
        }
    }

    pub async fn get_entry(&self, id: u32) -> AnilistEntry {
        // Define query and variables
        let json = json!({"query": QUERY, "variables": {"id": id}});
        // Make HTTP post request
        let resp = self
            .client
            .post("https://graphql.anilist.co/")
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .body(json.to_string())
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        // Get json
        let result: AnilistEntry = serde_json::from_str(&resp).unwrap();

        result
    }
}

impl AnilistEntry {
    pub async fn get_image(url: String) -> Option<DynamicImage> {
        let img_bytes = reqwest::get(url).await.ok()?.bytes().await.ok()?;

        let image = image::load_from_memory(&img_bytes).ok()?;

        Some(image)
    }
}
