
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
  // TODO: win32
  let p = String::from(path.to_str().unwrap());
  let v: Vec<&str> = p.rsplit('/').collect();

  if v.len().eq(&0) {
    return None;
  }

  let version = v[0];
  if VERSION_RE.is_match(version) {
    return Some(parse_version_string(version))
  }

  None
}

// return true if a >= b
fn compare_version(a: &Version, b: &Version) -> bool {
  if a.0 > b.0 { return true; }
  else if a.0 == b.0 {
    if a.1 > b.1 { return true; }
    else if a.1 == b.1 {
      return a.2 >= b.2;
    }
    return false;
  }
  return false;
}

fn get_latest_version() -> Option<(Version, PathBuf)> {
  let runtimes_path = get_runtimes_path();
  let mut latest_version: Option<Version> = None;
  let mut latest_path: Option<PathBuf> = None;

  let paths = fs::read_dir(runtimes_path).unwrap();
  for p in paths {
    let path = p.unwrap().path();
    if path.is_file() { continue; }

    if let Some(version) = get_version_from_path(&path) {
      if latest_version.eq(&None) || compare_version(&version, &latest_version.unwrap()) {
        latest_version = Some(version);
        latest_path = Some(path);
      }
    }
  }

  if let Some(version) = latest_version {
    return Some((version, latest_path.unwrap()))
  }
  None
}

fn parse_version_string(v: &str) -> Version {
  let t: Vec<u8> = v.split(".")
      .collect::<Vec<&str>>()
      .into_iter()
      .map(|i: &str| String::from(i).parse::<u8>().unwrap())
      .collect();
  (t[0], t[1], t[2])
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

pub fn get_valid_runtime_path(v: &str) -> Option<(Version, PathBuf)> {
  let version_string = String::from(v.trim());
  let latest = get_latest_version();
  if let Some((latest_version, latest_path)) = &latest {
    // Any version
    if version_string.eq("*") {
      return latest.clone();
    }

    // Lock runtime version
    if VERSION_RE.is_match(&version_string) {
      let version = parse_version_string(&version_string);
      if is_runtime_exist(version) {
        return Some((version, gen_path_from_version(version)))
      }
      return None;
    }

    // Above one version
    if ABOVE_VERSION_RE.is_match(&version_string) {
      let version = parse_version_string(
        &String::from(version_string)
            .replace("^", "")
      );
      if compare_version(&latest_version, &version) {
        return latest.clone();
      }
    }
  }
  return None;
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

pub fn link_runtime(runtime_path: &PathBuf) {
  let current_path = env::current_exe().unwrap();
  let to = &current_path
      .parent().unwrap()
      .parent().unwrap()
      .join("Frameworks")
      .to_path_buf();

  let paths = fs::read_dir(runtime_path).unwrap();
  for p in paths {
    let path = p.unwrap().path();
    if path.to_str().unwrap().contains(".DS_Store") {
      continue;
    }
    let from = &path;
    link_file(from, to);
  }
}

fn link_file(from: &PathBuf, to: &PathBuf) {
  if cfg!(target_os = "windows") {
    // TODO: Windows
    Command::new("cmd")
        .args(&["/C", "echo hello"])
        .output()
        .expect("failed to execute process");
  } else {
    Command::new("ln")
        .args(&[
          "-s",
          "-f",
          from.to_str().unwrap(),
          to.to_str().unwrap()
        ])
        .spawn()
        .unwrap()
        .wait()
        .expect("failed to execute process");
    println!("{}-{}", from.display(), to.display());
  };
}
