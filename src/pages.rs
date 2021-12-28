use crate::helpers::{gen_links, get_page_num, get_pbar};
use crate::posts::get_posts;
use futures::stream::futures_unordered::FuturesUnordered;
use futures::StreamExt;

pub async fn get_pages(
    site_page: &str,
    base_url: &str,
    artist_name: &str,
    client: &reqwest::Client,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut handles_pages = FuturesUnordered::new();
    let len = get_page_num(&site_page, &client).await?;
    let page_bar = get_pbar(
        len as u64,
        &format!("Pages for {}", artist_name),
    )?;
    for x in 0..len {
        let handle = async move {
            let post_client = &client.clone();
            let page_client = &client.clone();
            let posts_links = gen_links(&site_page, "post-card__link", x, base_url, post_client)
                .await
                .unwrap();
                
            get_posts(posts_links, artist_name, x, base_url, page_client)
                .await
                .unwrap();
        };
        handles_pages.push(handle);
    }
    while let Some(_item) = handles_pages.next().await {
        page_bar.inc(1);
    }
    page_bar.finish();
    Ok(())
}
