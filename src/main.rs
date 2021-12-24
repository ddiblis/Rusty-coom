extern crate reqwest;
extern crate select;
extern crate tokio;

use bytes::Buf;
use std::io::{Cursor};

use select::document::Document;
use select::predicate::{Class, Name};
use http::{HeaderMap, HeaderValue, header::{COOKIE}};

async fn get_dom(url: &str) -> Result<Document, Box<dyn std::error::Error>> {
    let mut headers = HeaderMap::new();
    headers.append(COOKIE, HeaderValue::from_str("__ddg2=6fryH34fRixR8HCV").unwrap());

    println!("{}", url);
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

async fn gen_links(url: &str, class_name: &str, page_num: usize, base_url: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let doc = get_dom(&format!("{}?o={:?}", &url, page_num*25)).await.unwrap();
    let mut links = Vec::new();
    for node in doc.find(Class(class_name)) {
        let name = node.find(Class("fancy-link")).next();
        match name {
            Some(value) => {links.push([base_url, value.attr("href").unwrap()].join(""))}
            None => {}
        }
    }
    Ok(links)
}

async fn get_page_num(link: &str) -> Result<usize, Box<dyn std::error::Error>>{
    let mut page_len = 0;
    let doc = get_dom(&link).await.unwrap();

    for node in doc.find(Class("paginator")).next() {
        let small_node = node.find(Name("small")).next().unwrap().text();
        let split = small_node.split(" ").last().unwrap();
        let num: usize = split[0..split.len()-1].parse().unwrap();
        page_len = match num {
            n if n % 25 == 0 => { num / 25 }
            n if n % 25 != 0 => { num / 25 + 1 }
            _ => 0
        }
    }
    Ok(page_len)
}

async fn get_media_links(url: &str, base_url: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut links = Vec::new();
    let dom = get_dom(url).await.unwrap();

    let items = match dom.clone() {
        value if value.find(Class("post__attachment-link")).next() != None => { dom.find(Class("post__attachment-link")) }
        value if value.find(Class("post__thumbnail")).next() != None => { dom.find(Class("post__thumbnail")) } 
        _ => {dom.find(Class("post__content"))}
    };

    for item in items {
        let print = match item {
            value if value.attr("href") != None => {value.attr("href")}
            value => {value.find(Name("img")).next().unwrap().attr("src")}
        };
        links.push([&base_url, print.unwrap()].join(""));
    }
    
    Ok(links)
}

#[tokio::main]
async fn main() {

    return artists().await.unwrap();
}


// async fn get_posts(links: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
//     for link in links.iter() {
//         let mut artist_document = get_dom(&link).await.unwrap();
//         let artist_page_len = get_page_num(&artist_document).unwrap();

//         for x in 0..artist_page_len {
//             artist_document = get_dom(&format!("{}?o={:?}", &link, x*25)).await.unwrap();
//             let artist_links = gen_links(&artist_document, &link, "post-card__link");
//             println!("{:?}", artist_links);    
//         }
//     }
//     Ok(())
// }

async fn artists() -> Result<(), Box<dyn std::error::Error>> {
    let base_url = "https://coomer.party";
    let url = "https://coomer.party/artists";

    let site_page_len = get_page_num(&url).await.unwrap();
    
    for i in 0..site_page_len {
        let site_links = gen_links(&url, "user-card__name", i, base_url).await.unwrap();

        for link in site_links.iter() {
            let artist_page_len = get_page_num(&link).await.unwrap();

            for x in 0..artist_page_len {
                let posts_links = gen_links(&link, "post-card__link", x, base_url).await.unwrap();

                for li in posts_links.iter() { 
                    let media_links = get_media_links(li, base_url).await.unwrap();
                    println!("{:?}", media_links);
                }
            }
        }
    }    
    Ok(())
}