use futures::stream::futures_unordered::FuturesUnordered;
use futures::StreamExt;

use tokio::sync::Semaphore;
use std::sync::Arc;

use crate::helpers::{download_media, get_pbar};

pub async fn get_media(
  links: Vec<String>,
  location: &str,
  client: &reqwest::Client,
) -> Result<(), Box<dyn std::error::Error>> {
  let semaphore = Arc::new(Semaphore::new(10));
  let mut handles = FuturesUnordered::new();
  let image_bar = get_pbar(links.len() as u64, &format!("Images for {}", &location))?;

  for (i, img) in links.iter().enumerate() {
    let extension = img.split(".").last().unwrap();
    handles.push(download_media(img, &location, i + 1, extension, &client, semaphore.clone()));
  }
  while let Some(_item) = handles.next().await {
    image_bar.inc(1);
  }
  Ok(())
}
