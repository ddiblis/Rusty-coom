use futures::stream::futures_unordered::FuturesUnordered;
use futures::StreamExt;

use std::sync::Arc;
use tokio::sync::Semaphore;

use crate::helpers::{download_media, get_pbar};

pub async fn get_media(
  links: Vec<String>,
  location: &str,
  client: &reqwest::Client,
) -> Result<(), Box<dyn std::error::Error>> {
  if links.len() > 0 {
    let semaphore = Arc::new(Semaphore::new(10));
    let mut handles = FuturesUnordered::new();
    let media_bar = get_pbar(links.len() as u64, &format!("Media for {}", &location))?;
  
    for (i, media) in links.iter().enumerate() {
      let extension = media.split(".").last().unwrap();
      handles.push(download_media(
        media,
        &location,
        i + 1,
        extension,
        client.clone(),
        semaphore.clone(),
      ));
    }
    while let Some(_item) = handles.next().await {
      media_bar.inc(1);
    }
  }
  Ok(())
}
