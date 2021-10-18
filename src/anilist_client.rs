use image::DynamicImage;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use viuer::{print, Config};

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
    pub id: i32,

    #[serde(rename = "title")]
    pub title: Title,

    #[serde(rename = "format")]
    pub format: String,

    #[serde(rename = "genres")]
    pub genres: Vec<String>,

    #[serde(rename = "coverImage")]
    pub cover_image: CoverImage,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Title {
    #[serde(rename = "romaji")]
    pub romaji: String,

    #[serde(rename = "english")]
    pub english: String,

    #[serde(rename = "native")]
    pub native: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CoverImage {
    #[serde(rename = "medium")]
    pub img: String,
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

    pub async fn get_entry(&self, id: i32) -> Result<AnilistEntry, Box<dyn std::error::Error>> {
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
        let result: Result<AnilistEntry, _> = serde_json::from_str(&resp);

        match result {
            Ok(mut result) => {
                if let Some(x) = &result.data.media {
                    // Fetching and storing cover image
                    if let Ok(img) = AnilistEntry::get_image(x.cover_image.img.to_string()).await {
                        result.image = AnilistImage::new(img);
                    } else {
                        result.image = AnilistImage::default();
                    }
                }
                return Ok(result);
            }
            Err(e) => return Err(Box::new(e)),
        }
    }
}

impl AnilistEntry {
    async fn get_image(url: String) -> Result<DynamicImage, Box<dyn std::error::Error>> {
        let img_bytes = reqwest::get(url).await?.bytes().await?;

        let image = image::load_from_memory(&img_bytes)?;

        Ok(image)
    }

    pub async fn print_entry(&self) {
        if let Some(x) = &self.data.media {
            let conf = Config {
                transparent: true,
                absolute_offset: false,
                ..Default::default()
            };

            if print(&self.image.image, &conf).is_err() {
                println!("🚫 Sorry! There was an error printing the image!")
            }

            println!(
                "\nID: {}\nName: {}\nFormat: {}\nGenres: {:?}",
                &x.id, &x.title.english, &x.format, &x.genres
            );
        } else {
            println!("🚫 Can't fetch query")
        }
    }
}
