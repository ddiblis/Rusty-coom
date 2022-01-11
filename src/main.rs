mod helpers;
mod media;
mod pages;
mod posts;
use clap;
use helpers::gen_client;

#[tokio::main]
async fn main() {
        let client = gen_client().await.unwrap();
        let matches = clap::clap_app!(myapp =>
            (version: "1.0")
            (author: "Wafiq A. <wafiqaladwan@gmail.com>")
            (about: "Parses and downloads all artist content from a select few websites")
            (@group site +required =>  
                (@arg coomer: -c --coomer "Sets parse to coomer.party")
                (@arg kemono: -k --kemono "Sets parser to kemono.party")
            )
            (@group artist +required => 
                (@arg artist_url: -u --artist_url +takes_value "URL of artist to parse")
                (@arg artist_name: -n --artist_name +takes_value "Name of artist to parse for coomer")
                (@arg artist_index: -i --artist_index +takes_value "Number of artist to parse for kemono")
            )
            (@arg many_artists: -m --many_artists "A true or false flag for downloading many artists")
        ).get_matches();

        if matches.is_present("coomer") {
            println!("In coomer");
            let base_url = "https://coomer.party";
            println!("{:?}", matches.value_of("artist"));
        }
        else if matches.is_present("kemono") {
            println!("In kemono");
            let base_url = "https://kemono.party";
            println!("{:?}", matches.value_of("artist"))
        }

}

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