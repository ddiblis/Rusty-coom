use crate::helpers::{download_text, get_media_links, get_pbar};
use crate::media::get_media;
use std::fs;

use indicatif::ProgressIterator;

pub async fn get_posts(
  posts_links: Vec<String>,
  artist_name: &str,
  page_index: usize,
  base_url: &str,
  client: &reqwest::Client,
) -> Result<(), Box<dyn std::error::Error>> {
  let post_bar = get_pbar(
    posts_links.len() as u64,
    &format!("Posts for {}, Page {:0>3}", artist_name, page_index + 1,),
  )?;
  for (post_index, li) in posts_links.iter().enumerate().progress_with(post_bar) {
    let location = format!(
      "{}/Page {:0>3}/Post {:0>3}",
      artist_name,
      page_index + 1,
      post_index + 1
    );
    fs::create_dir_all(format!("coomer/{}", location)).unwrap();

    let post = get_media_links(li, base_url, &client).await.unwrap();

    if post.photos.len() > 0 {
      get_media(post.photos, &location, &client).await.unwrap();
    }
    if post.videos.len() > 0 {
      get_media(post.videos, &location, &client).await.unwrap();
    }
    download_text(&post.text, &location).await;
  }
  Ok(())
}
