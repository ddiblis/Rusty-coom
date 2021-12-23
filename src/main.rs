extern crate reqwest;
extern crate select;
extern crate tokio;

use bytes::Buf;
use std::io::{Cursor};

use select::document::Document;
use select::predicate::{Class};
use http::{HeaderMap, HeaderValue, header::{COOKIE}};

async fn get_dom(url: &str) -> Result<Document, Box<dyn std::error::Error>> {
    let mut headers = HeaderMap::new();
    headers.append(COOKIE, HeaderValue::from_str("__ddg2=6fryH34fRixR8HCV").unwrap());
    // headers.append(COOKIE, HeaderValue::from_str("__ddgid=HSL2mhPtYotOZi06").unwrap());
    // headers.append(COOKIE, HeaderValue::from_str("__ddg1=pilvS6hPG8lPPBGTjHXw").unwrap());
    // headers.append(COOKIE, HeaderValue::from_str("__ddgmark=FFQgw8rEC24qvsx1").unwrap());
    
    let client = reqwest::Client::new();
    let resp = client.get(url)
    .headers(headers)
    .send()
    .await?;

    assert!(resp.status().is_success());

    let body = Cursor::new(resp.bytes().await?).reader();
    let document = Document::from_read(body).unwrap();
    Ok(document)
}

fn gen_links(doc: &Document, url: &str, class_name: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut links = Vec::new();
    for node in doc.find(Class(class_name)) {
        let name = node.find(Class("fancy-link")).next();
        match name {
            Some(value) => {links.push([&url[0..url.len()-12], value.attr("href").unwrap()].join(""))}
            None => {}
        }
    }
    Ok(links)
}

#[tokio::main]
async fn main() {

    return artists("https://coomer.party/artists").await.unwrap();
}


async fn artists(url: &str) -> Result<(), Box<dyn std::error::Error>>{ 
    let artist_document = get_dom(&url).await.unwrap();
    let artist_links = gen_links(&artist_document, &url, "user-card__name").unwrap();

    println!("{:?}", artist_links);
    // for link in artist_links.iter() {
    //     let posts_document = get_dom(&link).await.unwrap();
    //     let posts_links = gen_links(&posts_document, &url, "post-card__link");
        
    //     println!("{:?}", posts_links);
    // }
    Ok(())
}