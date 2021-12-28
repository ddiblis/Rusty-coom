use futures::stream::futures_unordered::FuturesUnordered;
use futures::StreamExt;

use crate::helpers::{get_pbar, get_media_links};
use crate::images::get_images;



pub async fn get_posts(
  posts_links: Vec<String>,
  artist_name: &str,
  page_index: usize,
  base_url: &str,
  client: &reqwest::Client
) -> Result<(), Box<dyn std::error::Error>> {
  let mut handles_posts = FuturesUnordered::new();
  let post_bar = get_pbar(
      posts_links.len() as u64,
      &format!("Posts for {}, Page {:0>3}", artist_name, page_index + 1,),
  )?;

  for (post_index, li) in posts_links.iter().enumerate() {
      let handle = async move {
          let media_links = get_media_links(li, base_url, &client).await.unwrap();
          get_images(media_links, artist_name, page_index, post_index, &client)
              .await
              .unwrap();
      };
      handles_posts.push(handle);
  }
  while let Some(_item) = handles_posts.next().await {
      post_bar.inc(1);
  }
  post_bar.finish();
  Ok(())
}