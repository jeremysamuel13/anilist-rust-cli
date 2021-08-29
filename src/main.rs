use image::DynamicImage;
use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{
    fmt::Display,
    io::{self, Write},
};
use viuer::{print, Config};

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
    pub coverImage: Option<CoverImage>,
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

    async fn get_entry(&self, id: u32) -> AnilistEntry {
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
    async fn get_image(url: String) -> Option<DynamicImage> {
        let img_bytes = reqwest::get(url).await.ok()?.bytes().await.ok()?;

        let image = image::load_from_memory(&img_bytes).ok()?;

        Some(image)
    }

    fn option_to_string<T>(opt: Option<T>) -> String
    where
        T: Display,
    {
        if let Some(x) = opt {
            format!("{}", x)
        } else {
            String::from("None")
        }
    }

    async fn print_entry(&self) {
        if let Some(x) = &self.data.media {
            if let Some(y) = &x.coverImage {
                if let Some(z) = &y.img {
                    let url = z.to_string();
                    if let Some(img) = AnilistEntry::get_image(url).await {
                        let conf = Config {
                            transparent: true,
                            absolute_offset: false,
                            ..Default::default()
                        };

                        print(&img, &conf);

                        println!(
                            "\nID: {}\nName: {}",
                            AnilistEntry::option_to_string(x.id),
                            AnilistEntry::option_to_string(
                                x.title.as_ref().unwrap().english.clone()
                            )
                        )
                    }
                }
            }
        } else {
            println!("🚫 Can't print query 🚫")
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = AnilistClient::new();
    println!(
        "\n
             _______________________
            |                       |
            | Connected to AniList! |
            |_______________________|

            \n
    "
    );

    loop {
        let inp = input("🔵 Enter the id: ");
        match &inp.trim().to_lowercase()[..] {
            "exit" => {
                println!("See you later! 👋");
                std::process::exit(1);
            }

            _ => {
                let parse = inp.trim().parse::<u32>();

                match parse {
                    Err(_) => {
                        println!("🚫 Invalid input 🚫");
                        continue;
                    }
                    Ok(val) => {
                        let res = client.get_entry(val).await;
                        res.print_entry().await;
                    }
                }
            }
        }

        println!("\n----------------------------------------------------------------------------------------\n");
    }
}

fn input(message: &str) -> String {
    print!("{}", message);
    io::stdout().flush();
    let mut ret = String::new();
    io::stdin()
        .read_line(&mut ret)
        .and_then(|e| {
            println!("");
            Ok(())
        })
        .expect("Failed to read from stdin");

    ret
}
