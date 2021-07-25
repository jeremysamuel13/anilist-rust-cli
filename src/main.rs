use std::{
    io::{self, Write},
};

// This example uses 3 crates serde_json, reqwest, tokio
use reqwest::{Client, Error};
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
        let client = Client::new();
        // Define query and variables
        let json = json!({"query": QUERY, "variables": {"id": id}});
        // Make HTTP post request
        let resp = client
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

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = AnilistClient::new();
    println!("Connected to AniList!");

    loop {
        let inp = input("Enter the id: ");
        match &inp.trim().to_lowercase()[..] {
            "exit" => {
                std::process::exit(1);
            }

            _ => {
                let parse = inp.trim().parse::<u32>();

                match parse {
                    Err(_) => {
                        println!("Invalid input");
                        continue;
                    }
                    Ok(val) => {
                        let res = client.get_entry(val).await;
                        println!("{:?}", &res);
                    }
                }
            }
        }
    }
}

fn input(message: &str) -> String {
    print!("{}", message);
    io::stdout().flush();
    let mut ret = String::new();
    io::stdin()
        .read_line(&mut ret)
        .expect("Failed to read from stdin");
    ret
}
