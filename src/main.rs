mod helpers;
mod media;
mod pages;
mod posts;
use helpers::gen_client;
use pages::get_pages;

// use clap::{App, Arg, SubCommand};

// fn main() {
//     let matches = App::new("My Super Program")
//         .version("1.0")
//         .author("Kevin K. <kbknapp@gmail.com>")
//         .about("Does awesome things")
//         .arg(
//             Arg::new("artist_url")
//                 .short('u')
//                 .long("artist_url")
//                 .value_name("url")
//                 .help("URL of artist to parse")
//                 .takes_value(true),
//         )
//         .arg(
//             Arg::new("artist_name")
//             .short('n')
//             .long("artist_name")
//             .value_name("name")
//             .help("Name of artist to parse")
//             .takes_value(true)
//         )
//         .get_matches();
//     println!("{}", matches.value_of("artist_url").unwrap_or("No match"));
// }

#[tokio::main]
async fn main() {
    // return get_site().await.unwrap();
    let client = gen_client().await.unwrap();
    return get_artist("https://coomer.party/onlyfans/user/kiwisunset", &client)
        .await
        .unwrap();
}

async fn get_artist(
    artist_url: &str,
    client: &reqwest::Client,
) -> Result<(), Box<dyn std::error::Error>> {
    let base_url = "https://coomer.party";
    let artist_name = artist_url.split("/").last().unwrap();
    Ok(get_pages(artist_url, base_url, artist_name, client).await?)
}
