extern crate reqwest;
extern crate select;
extern crate tokio;

mod helpers;
use helpers::{get_page_num, gen_links, get_media_links};


#[tokio::main]
async fn main() {
    return artists().await.unwrap();
}

// async fn get_posts(links: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
//     for link in links.iter() {
//         let mut artist_document = get_dom(&link).await.unwrap();
//         let artist_page_len = get_page_num(&artist_document).unwrap();

//         for x in 0..artist_page_len {
//             artist_document = get_dom(&format!("{}?o={:?}", &link, x*25)).await.unwrap();
//             let artist_links = gen_links(&artist_document, &link, "post-card__link");
//             println!("{:?}", artist_links);
//         }
//     }
//     Ok(())
// }

async fn artists() -> Result<(), Box<dyn std::error::Error>> {
    let base_url = "https://coomer.party";
    let url = "https://coomer.party/artists";

    let site_page_len = get_page_num(&url).await.unwrap();
    for i in 0..site_page_len {
        let site_links = gen_links(&url, "user-card__name", i, base_url)
            .await
            .unwrap();

        for link in site_links.iter() {
            let artist_page_len = get_page_num(&link).await.unwrap();

            for x in 0..artist_page_len {
                let posts_links = gen_links(&link, "post-card__link", x, base_url)
                    .await
                    .unwrap();

                for li in posts_links.iter() {
                    let media_links = get_media_links(li, base_url).await.unwrap();

                    println!("{:?}", media_links);
                }
            }
        }
    }
    Ok(())
}
