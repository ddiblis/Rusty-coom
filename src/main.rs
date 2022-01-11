mod helpers;
mod media;
mod pages;
mod posts;
use clap;
use helpers::gen_client;
use pages::get_pages;

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
            if matches.is_present("artist_url") {
                return get_artist(matches.value_of("artist_url").unwrap(), &client, base_url)
                .await
                .unwrap();
            }
            else if matches.is_present("artist_name") {
                return get_artist(&format!("https://coomer.party/onlyfans/user/{}", matches.value_of("artist_name").unwrap()), &client, base_url)
                .await
                .unwrap();
            }
        }

        else if matches.is_present("kemono") {
            println!("In kemono");
            let base_url = "https://kemono.party";
            if matches.is_present("artist_url"){
                return get_artist(matches.value_of("artist_url").unwrap(), &client, base_url)
                .await
                .unwrap();
            }
            else if matches.is_present("artist_index") {
                return get_artist(&format!("https://coomer.party/onlyfans/user/{}", matches.value_of("artist_index").unwrap()), &client, base_url)
                .await
                .unwrap();
            }
        }

}

async fn get_artist(
    artist_url: &str,
    client: &reqwest::Client,
    base_url: &str
) -> Result<(), Box<dyn std::error::Error>> {
    let artist_name = artist_url.split("/").last().unwrap();
    Ok(get_pages(artist_url, base_url, artist_name, client).await?)
}