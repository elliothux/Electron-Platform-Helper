use zip;
use reqwest::get;
use std::{io, fs};
use std::io::prelude::*;
use std::fs::File;
use std::path::{Path, PathBuf};
use model::{Version, ReleaseResponse, Platform};
use statics::{VERSION_RE, ABOVE_VERSION_RE};
use helper;
use utils;


pub fn download_runtime(v: &str) -> Option<Version> {
  match get_valid_runtime_version(v)  {
    Err(_) => None,
    Ok(version) => {
      let temp_path = helper::get_platform_path().join("./temp");
      let download_url = get_runtime_url(version);
      let filename = format!("{}.zip", helper::version_to_string(version));
      println!("{}", temp_path.display());
      println!("{}", &download_url);
      if let Ok(file_path) = download_file(&download_url, &filename, &temp_path) {}
      None
    }
  }
}

pub fn unzip_file(file_path: PathBuf, to: PathBuf) -> Result<(), String> {
  let mut file = File::open(file_path).unwrap();
  let mut archive_result = zip::ZipArchive::new(file);
  match archive_result {
    Ok(mut archive) => {
      for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = to.join(file.sanitized_name());

        {
          let comment = file.comment();
          if !comment.is_empty() {
            println!("File {} comment: {}", i, comment);
          }
        }

        if (&*file.name()).ends_with('/') {
          println!("File {} extracted to \"{}\"", i, outpath.as_path().display());
          fs::create_dir_all(&outpath).unwrap();
        } else {
          println!("File {} extracted to \"{}\" ({} bytes)", i, outpath.as_path().display(), file.size());
          if let Some(p) = outpath.parent() {
            if !p.exists() {
              fs::create_dir_all(&p).unwrap();
            }
          }
          let mut outfile = fs::File::create(&outpath).unwrap();
          io::copy(&mut file, &mut outfile).unwrap();
        }

        // Get and Set permissions
        #[cfg(unix)]
          {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
              fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
            }
          }
      }
      Ok(())
    }
    _ => Err("Unzip failed".to_owned())
  }
}

fn download_file(url: &str, filename: &str, path: &PathBuf) -> Result<PathBuf, String> {
  let file_path = path.join(filename);
  match get(url) {
    Ok(mut resp) => {
      let mut out = File::create(&file_path)
        .expect("failed to create file");
      match io::copy(&mut resp, &mut out) {
        Ok(_) => Ok(file_path),
        Err(_) => Err("Download failed".to_owned())
      }
    }
    Err(_) => Err("Request failed".to_owned())
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

fn get_valid_runtime_version(v: &str) -> Result<Version, String> {
  if VERSION_RE.is_match(v) {
    // Lock runtime version
    Ok(helper::parse_version_string(v))
  } else if ABOVE_VERSION_RE.is_match(v) {
    // Above one version
    get_latest_version()
  } else {
    Err("Invalid config 'version'".to_owned())
  }
}

pub fn get_latest_version() -> Result<Version, String> {
  let request = get("https://api.github.com/repos/electron/electron/tags");
  match request {
    Err(_) => Err("Request failed".to_owned()),
    Ok(mut response) => {
      match response.json::<Vec<ReleaseResponse>>() {
        Err(_) => Err("Parse json error".to_owned()),
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
