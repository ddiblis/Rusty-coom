mod helpers;
mod images;
mod pages;
mod posts;
use helpers::gen_client;
use pages::get_pages;

#[tokio::main]
async fn main() {
    // return get_site().await.unwrap();
    let client = gen_client().await.unwrap();
    return get_artist("https://coomer.party/onlyfans/user/belledelphine", &client)
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
