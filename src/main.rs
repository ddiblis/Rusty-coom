mod helpers;
mod media;
mod pages;
mod posts;



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
        // .arg(
        //     Arg::new("many artists")
        //     .short('m')
        //     .long("many")
        //     .value_name("many")
        //     .help("input many links or artist names separated by spaces to download")
        //     .takes_value(true)
        // )
        // .arg(
        //     Arg::new("site")
        //     .short('s')
        //     .long("site")
        //     .value_name("site_name")
        //     .help("choose between coomer.party and kemono.party")
        //     .takes_value(true)
        // )
        // .arg(
        //     Arg::new("all_sites")
        //     .short('a')
        //     .long("all_sites")
        //     .value_name("all_sites")
        //     .help("Flag for downloading artist posts from all websites coded in")
        //     .takes_value(true)
        // )
//     println!("{}", matches.value_of("artist_url").unwrap_or("No match"));
// }

// #[tokio::main]
// async fn main() {
//     // return get_site().await.unwrap();
//     let client = gen_client().await.unwrap();
//     return get_artist("https://coomer.party/onlyfans/user/hidorirose", &client)
//         .await
//         .unwrap();
// }

// async fn get_artist(
//     artist_url: &str,
//     client: &reqwest::Client,
// ) -> Result<(), Box<dyn std::error::Error>> {
//     let base_url = "https://coomer.party";
//     let artist_name = artist_url.split("/").last().unwrap();
//     Ok(get_pages(artist_url, base_url, artist_name, client).await?)
// }

// use clap;

fn main() {
    println!("NO")
    // let matches = clap::clap_app!(myapp =>
    //     (version: "1.0")
    //     (author: "Kevin K. <kbknapp@gmail.com>")
    //     (about: "Does awesome things")
    //     (@arg CONFIG: -c --config +takes_value "Sets a custom config file")
    //     (@arg INPUT: +required "Sets the input file to use")
    //     (@arg verbose: -v --verbose "Print test information verbosely")
    //     (@subcommand test =>
    //         (about: "controls testing features")
    //         (version: "1.3")
    //         (author: "Someone E. <someone_else@other.com>")
    //         (@arg debug: -d ... "Sets the level of debugging information")
    //     )
    // ).get_matches();

    // Same as previous examples...
}