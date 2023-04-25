use std::{
    path::Path,
    fs::{File, self},
    io::{prelude::*, BufReader},
};
use futures::{stream, StreamExt};
use reqwest::Client;
use tokio;
use image;
use bytes::Bytes;

struct Img {
    bytes: Bytes,
    url: String
}

#[tokio::main]
async fn main() {
    let client: Client = Client::new();

    let file: File = File::open("urls.txt").expect("no such file");
    let buf: BufReader<File> = BufReader::new(file);
    let urls: Vec<String> = buf.lines()
        .map(|l| l.expect("failed to parse line"))
        .collect();
    let num_urls: usize = urls.len();

    let x = stream::iter(urls)
        .map(|url| {
            let client = client.clone();
            let url_clone = url.clone();
            tokio::spawn(async move{
                let resp = client.get(url).send().await.expect("failed in reqwest::send");
                let bytes = resp.bytes().await.expect("failed in reqwest::bytes");
                Img {
                    bytes: bytes,
                    url: url_clone,
                }
            })
        }).buffer_unordered(num_urls);

    let mut count = 1;
    loop {
        if Path::new(format!("out/{}", count).as_str()).is_dir() {
            count = count + 1;
        } else {
            break;
        }
    }
    fs::create_dir(format!("out/{}", count).as_str()).expect("failed to create new directory");

    x.for_each(|i| async move {
        let img = i.expect("failed in tokio");
        let filename_right = img.url.split('/').last().unwrap();
        let filename = format!("out/{}/{}", count, filename_right);
        image::load_from_memory(&img.bytes).expect("failed to decode image").save(filename).expect("failed to encode image");
    }).await;
}
