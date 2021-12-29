use crate::helpers::{gen_links, get_page_num, get_pbar};
use crate::posts::get_posts;

use indicatif::ProgressIterator;

pub async fn get_pages(
    site_page: &str,
    base_url: &str,
    artist_name: &str,
    client: &reqwest::Client,
) -> Result<(), Box<dyn std::error::Error>> {
    let len = get_page_num(&site_page, &client).await?;
    let page_bar = get_pbar(len as u64, &format!("Pages for {}", artist_name))?;
    for x in (0..len).progress_with(page_bar) {
        let post_client = &client.clone();
        let page_client = &client.clone();
        let posts_links = gen_links(&site_page, "post-card__link", x, base_url, post_client)
            .await
            .unwrap();
        get_posts(posts_links, artist_name, x, base_url, page_client)
            .await
            .unwrap();
    }
    Ok(())
}
