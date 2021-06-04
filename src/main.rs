extern crate reqwest;
extern crate attohttpc;
extern crate image;

use std::io;
use bytes::Bytes;
use std::fs::File;
use std::collections::HashMap;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use image::ImageFormat;
use std::path::Path;
use std::env;
use std::str::FromStr;

fn get_random_string(length: usize) -> String{
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()

}

//TODO: Validate image

async fn save_image(path: &String){
    let filename: String = get_random_string(7);

    let url: String = format!("{}{}{}", "https://i.imgur.com/", filename, ".jpg");

    let image_bytes = reqwest::get(&url)
        .await.unwrap()
        .bytes()
        .await.unwrap();

    let image = image::load_from_memory(&image_bytes)
        .unwrap();

    let breiunos = format!("{}{}{}", path, filename, ".jpg");
    let mut newpath = Path::new(&breiunos);
    image.save(newpath)
        .unwrap();

}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args()
        .collect();
    let mut path = &args[1];
    let mut o  ="./".to_string();
    if path == "" {

        path = &o;
    }
    let query = &args[2];
    let n = i32::from_str(query)
        .unwrap_or(0);
    println!("Usage: sisa <path> <query>");


    let mut i = 0;
    while i < n{
        save_image(path)
            .await;
        i += 1;
    }
}
