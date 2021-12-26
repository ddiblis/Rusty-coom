use linya::{Bar, Progress};
use std::fs;

mod helpers;
use helpers::{download_img, gen_links, get_media_links, get_page_num};

#[tokio::main]
async fn main() {
    return get_site().await.unwrap();
}

async fn get_images(links: Vec<String>, artist_name: &str, x: usize, post_index: usize) {
    let location = format!(
        "coomer/{}/Page {:0>3}/Post {:0>3}",
        artist_name,
        x + 1,
        post_index + 1
    );
    fs::create_dir_all(&location).unwrap();
    let mut image_progress = Progress::new();
    let image_bar: Bar = image_progress.bar(links.len(), format!("Downloading {} images", artist_name));

    for (i, img) in links.iter().enumerate() {
        image_progress.set_and_draw(&image_bar, i + 1);
        if img.split(".").last().unwrap() == "jpg" {
            download_img(img, &location, i + 1).await.unwrap();
        }
        
    }
}

async fn get_posts(
    site_page: &str,
    base_url: &str,
    artist_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let len = get_page_num(&site_page).await?;
    let mut page_progress = Progress::new();
    let page_bar: Bar = page_progress.bar(len, format!("Downloading {} Pages", artist_name));

    for x in 0..len {
        let posts_links = gen_links(&site_page, "post-card__link", x, base_url).await?;
        let mut post_progress = Progress::new();
        let post_bar: Bar = post_progress.bar(
            posts_links.len(),
            format!("Downloading {} Posts", artist_name),
        );
        
        for (post_index, li) in posts_links.iter().enumerate() {
            let media_links = get_media_links(li, base_url).await?;
            post_progress.set_and_draw(&post_bar, post_index + 1);
            get_images(media_links, artist_name, x, post_index).await;
        }
        page_progress.set_and_draw(&page_bar, x + 1);
    }
    Ok(())
}

async fn get_site() -> Result<(), Box<dyn std::error::Error>> {
    let base_url = "https://coomer.party";
    let url = "https://coomer.party/artists";

    let site_page_len = get_page_num(&url).await?;
    for i in 0..site_page_len {
        let site_pages = gen_links(&url, "user-card__name", i, base_url).await?;
        for site_page in site_pages.iter() {
            let artist_name = site_page.split("/").last().unwrap();

            get_posts(site_page, base_url, artist_name).await.unwrap();
        }
    }
    Ok(())
}
