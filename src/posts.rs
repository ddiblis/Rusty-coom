use futures::stream::futures_unordered::FuturesUnordered;
use futures::StreamExt;

use crate::helpers::{get_media_links, get_pbar, download_text};
use crate::media::get_media;
use std::fs;

pub async fn get_posts(
  posts_links: Vec<String>,
  artist_name: &str,
  page_index: usize,
  base_url: &str,
  client: &reqwest::Client,
) -> Result<(), Box<dyn std::error::Error>> {
  let mut handles_posts = FuturesUnordered::new();
  let post_bar = get_pbar(
    posts_links.len() as u64,
    &format!("Posts for {}, Page {:0>3}", artist_name, page_index + 1,),
  )?;
  for (post_index, li) in posts_links.iter().enumerate() {
    let location = format!(
      "coomer/{}/Page {:0>3}/Post {:0>3}",
      artist_name,
      page_index + 1,
      post_index + 1
    );
    fs::create_dir_all(&location).unwrap();

    let handle = async move {
      let post = get_media_links(li, base_url, &client).await.unwrap();

      if post.photos.len() > 1 {
        get_media(
          post.photos,
          artist_name,
          page_index,
          post_index,
          &location,
          &client,
        )
        .await
        .unwrap();
      }
      if post.videos.len() > 1 {
        get_media(
          post.videos,
          artist_name,
          page_index,
          post_index,
          &location,
          &client,
        )
        .await
        .unwrap();
      }
      download_text(&post.text, &location);
    };
    handles_posts.push(handle);
  }
  while let Some(_item) = handles_posts.next().await {
    post_bar.inc(1);
  }
  post_bar.finish();
  Ok(())
}
