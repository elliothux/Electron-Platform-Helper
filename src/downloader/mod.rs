
use reqwest::get;
use std::io;
use std::fs::File;
use std::path::Path;
use model::{Version};


pub fn download_runtime(v: Version) {

}

fn download_to(url: &str, path: &str) {
    let mut resp = get(url).expect("request failed");
    let mut out = File::create("rustup-init.sh").expect("failed to create file");
    io::copy(&mut resp, &mut out).expect("failed to copy content");
}
