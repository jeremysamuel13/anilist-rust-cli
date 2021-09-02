mod anilist_client;

use crate::anilist_client::*;
use reqwest::Error;
use std::io::{self, Write};
use viuer::{print, Config};

const DIVIDER: &str =
    "\n----------------------------------------------------------------------------------------\n";

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = AnilistClient::new();
    println!(
        "\n
             _______________________
            |                       |
            |  Welcome to AniList!  |
            |_______________________|

            \n
    "
    );

    loop {
        let inp = input("> Enter the id: ");
        match &inp.trim().to_lowercase()[..] {
            "exit" => {
                println!("👋 See you later!");
                println!("{}", DIVIDER);
                std::process::exit(1);
            }

            _ => {
                let parse = inp.trim().parse::<u32>();

                match parse {
                    Err(_) => {
                        println!("🚫 Invalid input");
                        println!("{}", DIVIDER);
                        continue;
                    }
                    Ok(val) => {
                        let res = client.get_entry(val).await;
                        if let Ok(o) = res {
                            o.print_entry().await;
                        } else {
                            println!("🚫 Can't connect to anilist")
                        }
                    }
                }
            }
        }

        println!("{}", DIVIDER);
    }
}

fn input(message: &str) -> String {
    print!("{}", message);
    io::stdout().flush().expect("Error reading from stdin");
    let mut ret = String::new();
    io::stdin()
        .read_line(&mut ret)
        .and_then(|_| {
            println!();
            Ok(())
        })
        .expect("Failed to read from stdin");

    ret
}

impl AnilistEntry {
    pub async fn print_entry(&self) {
        let title;
        let id;
        let format;
        let genres: Vec<String>;

        if let Some(x) = &self.data.media {
            let conf = Config {
                transparent: true,
                absolute_offset: false,
                ..Default::default()
            };

            let print_res = print(&self.image.image, &conf);

            if let Err(_) = print_res {
                println!("🚫 Sorry! There was an error printing the image!")
            }

            let titles = x.title.as_ref().unwrap();
            let na = "None".to_string();

            title = titles
                .english
                .as_ref()
                .unwrap_or(
                    titles
                        .romaji
                        .as_ref()
                        .unwrap_or(titles.native.as_ref().unwrap_or(&na)),
                )
                .to_string();

            //GETTING INFO
            id = x.id.unwrap_or_default();
            format = x.format.clone().unwrap_or_default();
            genres = x.genres.clone().unwrap_or_default();

            println!(
                "\nID: {}\nName: {}\nFormat: {}\nGenres: {:?}",
                id, title, format, genres
            );
        } else {
            println!("🚫 Can't fetch query")
        }
    }
}
