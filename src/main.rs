use http::{header::COOKIE, HeaderMap, HeaderValue};
use image::io::Reader as ImageReader;
use image::ImageFormat;
use std::io::Cursor;
use std::fs;

mod helpers;
use helpers::{download_img, gen_links, get_media_links, get_page_num};

#[tokio::main]
async fn main() {
    return artists().await.unwrap();
}


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

                for (post_index, li) in posts_links.iter().enumerate() {
                    let media_links = get_media_links(li, base_url).await?;

                    for (img_index, img) in media_links.iter().enumerate() {
                        fs::create_dir_all(format!(
                            "coomer/{}/Page {:0>3}/Post {:0>3}",
                            artist_name,
                            x + 1,
                            post_index + 1
                        ))?;
                        if img.split(".").last().unwrap() == "jpg" {
                            download_img(img, artist_name, x + 1, post_index + 1, img_index + 1)
                                .await;
                        }
                    }
                }
            }
        }
    }
    Ok(())
}
