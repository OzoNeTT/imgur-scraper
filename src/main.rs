extern crate reqwest;
extern crate image;

use std::io;
use bytes::Bytes;
use std::fs::File;
use std::fs::metadata;
use std::fs::remove_file;
use std::collections::HashMap;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use image::ImageFormat;
use std::path::Path;
use std::env;
use std::str::FromStr;

//TODO : Make encapsulation
//TODO : Get normal distribution

fn get_random_string(length: usize) -> String{
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()

}

fn check_if_list(length_buf: u64) -> bool {
    let list = vec![0, 503, 5082, 4939, 4339, 4940, 4941, 12003, 5556];
    list.contains(&length_buf)
}

async fn save_image(path: &String){
    let filename: String = get_random_string(7);

    let url: String = format!("{}{}{}", "https://i.imgur.com/", filename, ".jpg");

    //TODO: Check for response code and change length from 7 to 6!
    let image_bytes = reqwest::get(&url)
        .await.unwrap()
        .bytes()
        .await.unwrap();

    let image = image::load_from_memory(&image_bytes).unwrap();

    let breiunos = format!("{}{}{}", path, filename, ".jpg");

    let mut newpath = Path::new(&breiunos);

    image.save(newpath)
        .unwrap();

    let metadata = metadata(newpath).unwrap();

    if check_if_list(metadata.len()) {
        println!("[-] Invalid: {}", url);
        remove_file(newpath).unwrap();
    } else {
        println!("[+] Valid: {}", url);
    }

}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args()
        .collect();
    let mut path = &args[1];
    let mut default_path  ="./".to_string();
    if path == "" {

        path = &default_path;
    }
    let query = &args[2];
    let n = i32::from_str(query)
        .unwrap_or(1);
    println!("Usage: sisa <path> <query>");


    let mut i = 0;
    while i < n{
        save_image(path)
            .await;
        i += 1;
    }
}
