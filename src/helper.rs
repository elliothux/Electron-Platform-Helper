
use lazy_static;
use regex::Regex;
use std::{env, fs};
use std::path::{Path, PathBuf};
use std::process::Command;
use model::{Platform, Version};
use utils::{
  path_buf_to_string,
  is_path_exist,
  get_current_platform_string
};



lazy_static! {
  static ref VERSION_RE: Regex = Regex::new(r"^\d+\.\d+\.\d+$").unwrap();
  static ref ABOVE_VERSION_RE: Regex = Regex::new(r"^\^\d+\.\d+\.\d+$").unwrap();
}


fn get_runtimes_path() -> PathBuf {
  let home_path = env::home_dir().unwrap();
  let platform_path = Path::new(&home_path)
    .join(Path::new(".electron-platform"));
  Path::new(&platform_path)
    .join(Path::new(
      &format!("runtime/{}",
         &get_current_platform_string(),
      )
    ))
}

fn get_version_from_path(path: &PathBuf) -> Option<Version> {
  // TODO
  None
}

// return true if a > b
fn compare_version(a: Version, b: Version) -> bool {
  if a.0 > b.0 { return true; }
  else if a.0 == b.0 {
    if a.1 > b.1 { return true; }
    else if a.1 == b.1 {
      return a.2 > b.2;
    }
    return false;
  }
  return false;
}

fn get_latest_version() -> (Version, PathBuf) {

}

fn parse_version_string(v: &str) -> Version {
  let t = v.split(".").collect().map(|i| i as u8);
  (t[0], t[1], t[2])
}
pub fn get_valid_runtime_path(v: &str) -> Option<PathBuf> {
  let version_string = String::from_str(&v).trim();
  let runtimes_path = get_runtimes_path();

  // Lock runtime version
  if VERSION_RE.is_match(&version_string) {
    let version = parse_version_string(&version_string);
    if is_runtime_exist(version) {
      return Some(gen_path_from_version(version))
    }
    return None;
  }

  let (latest_version, latest_path) = get_latest_version();
  //
  if version_string == "*" {
    return Some(latest_path);
  }
  // TODO

  let paths = fs::read_dir(&runtimes_path).unwrap();

  let mut valid_runtime_path: Option<PathBuf> = None;
  let mut valid_runtime_version: Option<(u8, u8, u8)> = None;

  for p in paths {
    let path = p.unwrap().path();
    println!("Name: {}", &path.display());

    if let Some(curr_version) = get_version_from_path(&path) {
      if version_string == "*" {
        if let Some(v) = valid_runtime_version {

        }
        valid_runtime_version = get_version_from_path(&path);
        valid_runtime_path = Some(path);
      } else {

      }
    }
  }

  valid_runtime_path
}

fn gen_path_from_version(version: Version) -> PathBuf {
  let platform_path = get_runtimes_path();
  let platform = get_current_platform_string();
  Path::new(&platform_path)
    .join(Path::new(
      &format!("runtime/{}/{}.{}.{}",
               platform, version.0, version.1, version.2
      )
    ))
}
pub fn is_runtime_exist(version: Version) -> bool {
  let platform_path = get_runtimes_path();
  if !is_path_exist(&platform_path) {
    return false;
  }

  let runtime_path = gen_path_from_version(version);
  is_path_exist(&runtime_path)
}

pub fn open_app_bin() {
  let current_path = env::current_exe().unwrap();
  if cfg!(target_os = "windows") {
    // TODO: Windows
    Command::new("cmd")
      .args(&["/C", "echo hello"])
      .spawn()
      .expect("failed to execute process")
  } else {
    let bin_path = path_buf_to_string(
      Path::new(&current_path)
        .with_file_name("App")
    );
    Command::new(&bin_path)
      .spawn()
      .expect("failed to execute process")
  };
}
