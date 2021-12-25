extern crate image;
extern crate reqwest;
extern crate select;
extern crate tokio;

use http::{header::COOKIE, HeaderMap, HeaderValue};
use image::io::Reader as ImageReader;
use image::ImageFormat;
use std::io::Cursor;
use std::fs;

mod helpers;
use helpers::{download_img, gen_links, get_media_links, get_page_num};

#[tokio::main]
async fn main() {
    // return download_video().await.unwrap();
    // return download_img().await.unwrap();
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

// async fn download_video() -> Result<(), Box<dyn std::error::Error>> {
//     let video_url = "https://coomer.party/data/7e/83/7e831c7933356ba2cf8daa2491dd30cbe29803378f9f4613d9da2890441b4241.mp4?f=0gna42ojmzs2wp90ftg05_source.mp4";
//     let mut headers = HeaderMap::new();
//     headers.append(
//         COOKIE,
//         HeaderValue::from_str("__ddg2=6fryH34fRixR8HCV")?,
//     );

//     let client = reqwest::Client::new();
//     let resp = client.get(video_url).headers(headers).send().await?;
//     let video = resp.bytes().await?;
//     let reader = ImageReader::new(Cursor::new(video))
//     .with_guessed_format()?;
//     // println!("{:?}", video);
//     assert_eq!(reader.format(), Some(ImageFormat::Jpeg));
//     let vid = reader.decode()?;
//     Ok(())
// }

async fn artists() -> Result<(), Box<dyn std::error::Error>> {
    let base_url = "https://coomer.party";
    let url = "https://coomer.party/artists";

    let site_page_len = get_page_num(&url).await?;
    for i in 0..site_page_len {
        let site_links = gen_links(&url, "user-card__name", i, base_url).await?;

        for link in site_links.iter() {
            let artist_page_len = get_page_num(&link).await?;
            let artist_name = link.split("/").last().unwrap();

            for x in 0..artist_page_len {
                let posts_links = gen_links(&link, "post-card__link", x, base_url).await?;

                for (post_index,li) in posts_links.iter().enumerate() {
                    let media_links = get_media_links(li, base_url).await?;

                    for (img_index, img) in media_links.iter().enumerate() {
                        fs::create_dir_all(format!("coomer/{}/Post {}", artist_name, post_index+1))?;
                        if img.split(".").last().unwrap() == "jpg" {
                            download_img(img, artist_name, post_index+1, img_index).await;
                        }
                    }
                }
            }
        }
    }
    Ok(())
}
