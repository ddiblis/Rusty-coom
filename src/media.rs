use indicatif::ProgressIterator;
use crate::helpers::{download_media, get_pbar};

pub async fn get_media(
  links: Vec<String>,
  artist_name: &str,
  page_index: usize,
  post_index: usize,
  location: &str,
  client: &reqwest::Client
) -> Result<(), Box<dyn std::error::Error>> {
  let image_bar = get_pbar(
      links.len() as u64,
      &format!(
          "Images for {}, Page {:0>3} post {:0>3}",
          artist_name,
          page_index + 1,
          post_index + 1
      ),
  )?;

  
  for (i, img) in links.iter().enumerate().progress_with(image_bar) {
    let extension = img.split(".").last().unwrap();

          download_media(img, &location, i + 1, extension, &client).await.unwrap();
  }
  Ok(())
}
