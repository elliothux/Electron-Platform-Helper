
use toml;
use std::io::prelude::*;
use std::{env, fs::File};
use std::path::{Path, PathBuf};
use model::Platform;



pub fn is_path_exist(path: &PathBuf) -> bool {
  Path::new(path).exists()
}

pub fn path_buf_to_string(path: PathBuf) -> String {
  path
    .to_str()
    .unwrap()
    .to_owned()
}

pub fn read_file_to_string(path: PathBuf) -> String {
  let mut f = File::open(&path)
    .expect(&format!("file \"{}\" not found", &path.to_str().unwrap()));
  let mut contents = String::new();
  f.read_to_string(&mut contents)
    .expect("something went wrong reading the file");
  contents
}

// platform
fn platform_to_str(platform: Platform) -> String {
  match platform {
    Platform::DARWIN => String::from("darwin"),
    Platform::WIN32 => String::from("win32"),
    Platform::WIN64 => String::from("win64"),
    Platform::LINUX32 => String::from("linux32"),
    Platform::LINUX64 => String::from("linux64"),
    Platform::UNKNOWN => String::from("unknown")
  }
}

pub fn get_current_platform() -> Platform {
  if cfg!(target_os = "macos") {
    return Platform::DARWIN;
  } else if cfg!(target_os = "windows") {
    if cfg!(target_pointer_width = "32") {
      return Platform::WIN32;
    } else if cfg!(target_pointer_width = "64") {
      return Platform::WIN64;
    }
  } else if cfg!(target_os = "linux") {
    if cfg!(target_pointer_width = "32") {
      return Platform::LINUX32;
    } else if cfg!(target_pointer_width = "64") {
      return Platform::LINUX64;
    }
  }
  Platform::UNKNOWN
}

pub fn get_current_platform_string() -> String {
  platform_to_str(
    get_current_platform()
  )
}

// config
#[derive(Deserialize)]
pub struct Config {
  pub target: String,
  pub runtime: String,
  pub installed: bool
}

pub fn get_config() -> Config {
  let current_path = env::current_exe().unwrap();
  let config_path = Path::new(&current_path)
    .with_file_name("ElectronPlatform.toml");
  let values: Config = toml::from_str(
    &read_file_to_string(config_path)
  ).unwrap();
  return values;
}

// Handle HTML
fn inline_style(s: &str) -> String {
  format!(r#"<style type="text/css">{}</style>"#, s)
}

fn inline_script(s: &str) -> String {
  format!(r#"<script type="text/javascript">{}</script>"#, s)
}

pub fn generate_html(styles: Vec<&str>, scripts: Vec<&str>) -> String {
  let inline_styles = styles.into_iter()
    .map(inline_style)
    .collect::<Vec<String>>()
    .join("\n");
  let inline_scripts = scripts.into_iter()
    .map(inline_script)
    .collect::<Vec<String>>()
    .join("\n");

  format!(r#"
    <!doctype html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport"
              content="width=device-width, user-scalable=no, initial-scale=1.0, maximum-scale=1.0, minimum-scale=1.0">
        <meta http-equiv="X-UA-Compatible" content="ie=edge">
        <title>Document</title>
        {styles}
    </head>
    <body>
        <div id="root"></div>
        {scripts}
    </body>
    </html>"#,
    styles = inline_styles,
    scripts = inline_scripts
  )
}
