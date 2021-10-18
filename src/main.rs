mod anilist_client;

use crate::anilist_client::*;
use clap::{App, Arg};
use reqwest::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = AnilistClient::new();
    let matches = App::new("anilist")
        .version("1.0")
        .author("Jeremy Samuel")
        .about("A CLI client for anilist")
        .arg(
            Arg::with_name("GET")
                .short("g")
                .long("get")
                .value_name("CODE")
                .help("fetches an anilist entry based on a given code")
                .takes_value(true),
        )
        .get_matches();

    let code = matches.value_of("GET").unwrap().parse().unwrap();

    match client.get_entry(code).await {
        Ok(val) => val.print_entry().await,
        _ => println!("Error"),
    }

    Ok(())
}
