use bytes::Buf;
use http::{header::COOKIE, HeaderMap, HeaderValue};
use indicatif::{ProgressBar, ProgressStyle};
use select::document::Document;
use select::predicate::{Class, Name};
use std::io::Cursor;
use std::fs::File;
use std::io::prelude::*;

pub async fn gen_client() -> Result<reqwest::Client, Box<dyn std::error::Error>> {
    Ok(reqwest::Client::new())
}

async fn get_dom(
    url: &str,
    client: &reqwest::Client,
) -> Result<Document, Box<dyn std::error::Error>> {
    let mut headers = HeaderMap::new();
    headers.append(COOKIE, HeaderValue::from_str("__ddg2=6fryH34fRixR8HCV")?);
    // println!("{}", url);
    let resp = client.clone().get(url).headers(headers).send().await?;

    assert!(resp.status().is_success());

    let body = Cursor::new(resp.bytes().await?).reader();
    let document = Document::from_read(body)?;
    Ok(document)
}

pub async fn gen_links(
    url: &str,
    class_name: &str,
    page_num: usize,
    base_url: &str,
    client: &reqwest::Client
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let doc = get_dom(&format!("{}?o={:?}", &url, page_num * 25), &client).await?;
    let mut links = Vec::new();
    for node in doc.find(Class(class_name)) {
        let name = node.find(Class("fancy-link")).next();
        match name {
            Some(value) => links.push([base_url, value.attr("href").unwrap()].join("")),
            None => {}
        }
    }
    Ok(links)
}

pub async fn get_page_num(link: &str, client: &reqwest::Client) -> Result<usize, Box<dyn std::error::Error>> {
    let mut page_len = 0;
    let doc = get_dom(&link, &client).await.unwrap();

    for node in doc.find(Class("paginator")).next() {
        let small_node = node.find(Name("small")).next().unwrap().text();
        let split = small_node.split(" ").last().unwrap();
        let num: usize = split[0..split.len() - 1].parse()?;
        page_len = match num {
            n if n % 25 == 0 => num / 25,
            n if n % 25 != 0 => num / 25 + 1,
            _ => 0,
        }
    }
    Ok(page_len)
}

pub async fn get_media_links(
    url: &str,
    base_url: &str,
    client: &reqwest::Client
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut links = Vec::new();
    let dom = get_dom(url, &client).await?;

    let items = match dom.clone() {
        value if value.find(Class("post__attachment-link")).next() != None => {
            dom.find(Class("post__attachment-link"))
        }
        value if value.find(Class("post__thumbnail")).next() != None => {
            dom.find(Class("post__thumbnail"))
        }
        _ => dom.find(Class("test")),
        
    };
    for item in items {
        let print = match item {
            value if value.attr("href") != None => value.attr("href"),
            value if value.find(Name("img")).next().unwrap().attr("src") != None => {
                value.find(Name("img")).next().unwrap().attr("src")
            }
            _ => Some("No"),
        };
        // let link = print.unwrap().split("?").nth(0).unwrap();
        
        if print.unwrap() != "No" {
            links.push([&base_url, print.unwrap()].join(""));
        }
    }
    Ok(links)
}

pub async fn download_img(
    url: &str,
    location: &str,
    image_index: usize,
    extension: &str,
    client: &reqwest::Client,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut headers = HeaderMap::new();
    headers.append(COOKIE, HeaderValue::from_str("__ddg2=6fryH34fRixR8HCV")?);
    let resp = client.clone().get(url).headers(headers).send().await?;
    let data = resp.bytes().await?;
    let name = format!("{}/{:0>3}.{}", location, image_index, extension);

    let mut pos = 0;
    let mut buffer = File::create(name)?;

    while pos < data.len() {
        let bytes_written = buffer.write(&data[pos..])?;
        pos += bytes_written;
    }
    Ok(())
}

pub fn get_pbar(
    length: u64,
    template: &str,
) -> Result<indicatif::ProgressBar, Box<dyn std::error::Error>> {
    let bar = ProgressBar::new(length);
    bar.set_style(
        ProgressStyle::default_bar()
            .template(&format!(
                "{} [{{elapsed_precise}}] {{bar:40.cyan/blue}} {{pos:1}}/{{len:5}}",
                template
            ))
            .progress_chars("=>-"),
    );
    Ok(bar)
}
