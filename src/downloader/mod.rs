
use zip::read::read_zipfile_from_stream;
use reqwest::get;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::path::{Path, PathBuf};
use model::{Version, ReleaseResponse, Platform};
use statics::{VERSION_RE, ABOVE_VERSION_RE};
use helper;
use utils;


pub fn download_runtime(v: &str) -> Option<Version> {
    let valid_version = get_valid_runtime_version(v);
    if let Some(version) = valid_version {
        let temp_path = helper::get_platform_path().join("./temp");
        let download_url = get_runtime_url(version);
        let filename = format!("{}.zip", helper::version_to_string(version));
        println!("{}", temp_path.display());
        println!("{}", &download_url);
        if let Ok(file_path) = download_file(&download_url, &filename, &temp_path) {

        }
        None
    }
    None
}

fn unzip_file(file_path: PathBuf, to: PathBuf) -> Result<(), io::Error> {
    let mut file = File::open(file_path).unwrap();
    let zip = read_zipfile_from_stream(f);
    match zip {
        Ok(Some(zip_file)) => {
            Ok(())
        },
        _ => Err(io::Error::from("Unzip failed"))
    }
}

fn download_file(url: &str, filename: &str, path: &PathBuf) -> Result<PathBuf, io::Error> {
    let file_path = path.join(filename);
    match get(url) {
        Ok(mut resp) => {
            let mut out = File::create(file_path)
                .expect("failed to create file");
            match io::copy(&mut resp, &mut out) {
                Ok(_) => Ok(file_path),
                Err(_) => Err(io::Error::from("Download failed"))
            }
        },
        Err(_) => Err(io::Error::from("Request failed"))
    }
}

fn get_runtime_url(version: Version) -> String {
    let v = helper::version_to_string(version);
    let prefix = format!("https://npm.taobao.org/mirrors/electron/{}", v);

    let platform = utils::get_current_platform();
    let mut platform_string: String = match platform {
        Platform::UNKNOWN => panic!("Unsupported platform"),
        Platform::DARWIN => String::from("darwin-x64"),
        Platform::WIN32 => String::from("win32-ia32"),
        Platform::WIN64 => String::from("win32-x64"),
        Platform::LINUX32 => String::from("linux-ia32"),
        Platform::LINUX64 => String::from("linux-x64")
    };

    format!("{}/electron-v{}-{}.zip", prefix, v, platform_string)
}

fn get_valid_runtime_version(v: &str) -> Result<Version, io::Error> {
    // Lock runtime version
    if VERSION_RE.is_match(v) {
        Ok(helper::parse_version_string(v))
    }

    // Above one version
    else if ABOVE_VERSION_RE.is_match(v) {
        get_latest_version()
    }

    else {
        Err(io::Error::from("Invalid config 'version'"))
    }
}

pub fn get_latest_version() -> Result<Version, io::Error> {
    let request = get("https://api.github.com/repos/electron/electron/tags");
    match request {
        Err(_) => Err(io::Error::from("Request failed")),
        Ok(mut response) => {
            match response.json::<Vec<ReleaseResponse>>() {
                Err(_) => Err(io::Error::from("Parse json error")),
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
                    Ok(latest)
                }
            }
        }
    }
}

pub fn get_text(url: &str) -> String {
    get(url).unwrap().text().unwrap()
}
