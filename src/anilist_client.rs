use image::DynamicImage;
use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug)]
pub struct AnilistImage {
    pub image: DynamicImage,
}

impl AnilistImage {
    pub fn new(image: DynamicImage) -> Self {
        Self { image }
    }
}

impl Default for AnilistImage {
    fn default() -> Self {
        Self {
            image: DynamicImage::new_bgr8(0, 0),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AnilistEntry {
    #[serde(rename = "data")]
    pub data: Data,

    #[serde(skip)]
    pub image: AnilistImage,
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

    pub async fn get_entry(&self, id: u32) -> Result<AnilistEntry, Error> {
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
            .await?
            .text()
            .await?;

        // Get json
        let mut result: AnilistEntry = serde_json::from_str(&resp).unwrap();

        // Fetching and storing cover image
        if let Some(x) = &result.data.media {
            if let Some(y) = &x.cover_image {
                if let Some(z) = &y.img {
                    let url = z.to_string();
                    if let Ok(img) = AnilistEntry::get_image(url).await {
                        result.image = AnilistImage::new(img);
                    } else {
                        result.image = AnilistImage::default();
                    }
                }
            }
        }

        Ok(result)
    }
}

impl AnilistEntry {
    async fn get_image(url: String) -> Result<DynamicImage, Box<dyn std::error::Error>> {
        let img_bytes = reqwest::get(url).await?.bytes().await?;

        let image = image::load_from_memory(&img_bytes)?;

        Ok(image)
    }
}
