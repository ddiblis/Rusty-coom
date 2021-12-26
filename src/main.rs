use indicatif::ProgressIterator;
use std::fs;

mod helpers;
use helpers::{download_img, gen_links, get_media_links, get_page_num, get_pbar};

#[tokio::main]
async fn main() {
    // return get_site().await.unwrap();
    return get_artist("https://coomer.party/onlyfans/user/jessabelletrix")
        .await
        .unwrap();
}

async fn get_artist(artist_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let base_url = "https://coomer.party";
    let artist_name = artist_url.split("/").last().unwrap();
    Ok(get_posts(artist_url, base_url, artist_name).await?)
}

async fn get_images(
    links: Vec<String>,
    artist_name: &str,
    page_index: usize,
    post_index: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    let location = format!(
        "coomer/{}/Page {:0>3}/Post {:0>3}",
        artist_name,
        page_index + 1,
        post_index + 1
    );
    fs::create_dir_all(&location).unwrap();
    let image_bar = get_pbar(links.len().try_into().unwrap(), &format!(
        "Images for {}, Page {} post {} [{{elapsed_precise}}] {{bar:40.cyan/blue}} {{pos:1}}/{{len:5}}",
        artist_name,
        page_index + 1,
        post_index + 1
    ))?;
    for (i, img) in links.iter().enumerate().progress_with(image_bar) {
        if img.split(".").last().unwrap() == "jpg" {
            download_img(img, &location, i + 1).await.unwrap();
        }
    }
    Ok(())
}

async fn get_posts(
    site_page: &str,
    base_url: &str,
    artist_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let len = get_page_num(&site_page).await?;
    let page_bar = get_pbar(
        len.try_into().unwrap(),
        &format!(
            "Pages for {} [{{elapsed_precise}}] {{bar:40.cyan/blue}} {{pos:1}}/{{len:5}}",
            artist_name
        ),
    )?;

    for x in (0..len).progress_with(page_bar) {
        let posts_links = gen_links(&site_page, "post-card__link", x, base_url).await?;
        let post_bar = get_pbar(
            posts_links.len().try_into().unwrap(),
            &format!(
            "Posts for {}, Page {} [{{elapsed_precise}}] {{bar:40.cyan/blue}} {{pos:1}}/{{len:5}}",
            artist_name,
            x + 1,
        ),
        )?;

        for (post_index, li) in posts_links.iter().enumerate().progress_with(post_bar) {
            let media_links = get_media_links(li, base_url).await?;
            get_images(media_links, artist_name, x, post_index).await?;
        }
    }
    Ok(())
}

async fn get_site() -> Result<(), Box<dyn std::error::Error>> {
    let base_url = "https://coomer.party";
    let url = "https://coomer.party/artists";

    let site_page_len = get_page_num(&url).await?;
    let site_bar = get_pbar(
        site_page_len.try_into().unwrap(),
        "Site coom.party [{elapsed_precise}] {bar:40.cyan/blue} {pos:1}/{len:5}",
    )?;

    for i in (0..site_page_len).progress_with(site_bar) {
        let site_pages = gen_links(&url, "user-card__name", i, base_url).await?;
        let artists_bar = get_pbar(
            site_pages.len().try_into().unwrap(),
            "Artists [{elapsed_precise}] {bar:40.cyan/blue} {pos:1}/{len:5}",
        )?;

        for site_page in site_pages.iter().progress_with(artists_bar) {
            let artist_name = site_page.split("/").last().unwrap();

            get_posts(site_page, base_url, artist_name).await?;
        }
    }
    Ok(())
}
