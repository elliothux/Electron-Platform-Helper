
use reqwest::get;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use model::{Version, ReleaseResponse, Platform};
use statics::{VERSION_RE, ABOVE_VERSION_RE};
use helper;
use utils;


pub fn download_runtime(v: &str) -> Option<Version> {
    let valid_version = get_valid_runtime_version(v);
    if let Some(version) = valid_version {
        let temp_path = helper::get_platform_path().join("./temp");
        println!("{}", temp_path.display());
        println!("{}", &get_runtime_url(version))
    }
    None
}

fn download_to(url: &str, path: &str) {
    let mut resp = get(url).expect("request failed");
    let mut out = File::create("rustup-init.sh").expect("failed to create file");
    io::copy(&mut resp, &mut out).expect("failed to copy content");
}

fn get_runtime_url(version: Version) -> String {
    let v = helper::version_to_string(version);
    let prefix = format!("https://npm.taobao.org/mirrors/electron/{}", v);

    let platform = utils::get_current_platform();
    let mut platform_string: String = match platform {
        Platform::UNKNOWN => { panic!("Unsupported platform"); }
        Platform::DARWIN => { String::from("darwin-x64") }
        Platform::WIN32 => { String::from("win32-ia32") }
        Platform::WIN64 => { String::from("win32-x64") }
        Platform::LINUX32 => { String::from("linux-ia32") }
        Platform::LINUX64 => { String::from("linux-x64") }
    };

    format!("{}/electron-v{}-{}.zip", prefix, v, platform_string)
}

fn get_valid_runtime_version(v: &str) -> Option<Version> {
    // Lock runtime version
    if VERSION_RE.is_match(v) {
        return Some(helper::parse_version_string(v));
    }

    // Above one version
    if ABOVE_VERSION_RE.is_match(v) {
        return get_latest_version();
    }

    None
}

pub fn get_latest_version() -> Option<Version> {
    let request = get("https://api.github.com/repos/electron/electron/tags");
    match request {
        Err(_) => { return None; }
        Ok(mut response) => {
            match response.json::<Vec<ReleaseResponse>>() {
                Err(_) => { return None; }
                Ok(result) => {
                    let versions = &result
                        .into_iter()
                        .filter_map(|i| {
                            let v = i.name.as_str()[1..].to_owned();
                            // Without Beta & Nightly
                            if v.contains("-") { return None; }
                            return Some(helper::parse_version_string(&v));
                        })
                        .collect::<Vec<Version>>();
                    let latest = versions[0];
                    Some(latest)
                }
            }
        }
    }
}

pub fn get_text(url: &str) -> String {
    get(url).unwrap().text().unwrap()
}
