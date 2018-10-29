
use std::{fs, env};
use std::path::{Path, PathBuf};
use std::process::Command;
use os_info;


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
        {scripts}
    </body>
    </html>"#,
    styles = inline_styles,
    scripts = inline_scripts
  )
}


fn is_path_exist(path: &str) -> bool {
  Path::new(path).exists()
}

fn get_platform() -> String {
  match os_info::get().os_type() {
    os_info::Type::Macos => String::from("darwin"),
    os_info::Type::Windows => String::from("win32"),
    _ => String::from("linux")
  }
}

pub enum Platform {
  DARWIN,
  WIN32,
  WIN64,
  LINUX32,
  LINUX64
}

fn get_platform_str(platform: Platform) -> String {
  match platform {
    Platform::DARWIN => String::from("darwin"),
    Platform::WIN32 => String::from("win32"),
    Platform::WIN64 => String::from("win64"),
    Platform::LINUX32 => String::from("linux32"),
    Platform::LINUX64 => String::from("linux64")
  }
}

pub fn is_runtime_exist(platform: Platform, version: &str) -> bool {
  let platform_path = fs::canonicalize(
    &PathBuf::from(
      "~/.electron-platform"
    )
  )
    .unwrap()
    .to_str()
    .unwrap();
  if !is_path_exist(platform_path) {
    return false;
  }

  let runtime_path = Path::new(platform_path)
    .join(Path::new(
      format!("{}/{}",
          get_platform_str(platform),
          version
      )
    ))
    .to_str()
    .unwrap();
  is_path_exist(runtime_path)
}

pub fn open_app_bin() {
  let current_path = env::current_dir().unwrap();
  if cfg!(target_os = "windows") {
    // TODO: Windows
    Command::new("cmd")
      .args(&["/C", "echo hello"])
      .output()
      .expect("failed to execute process")
  } else {
    let bin_path = Path::new(current_path)
      .join("./electron")
      .to_str()
      .unwrap();
    Command::new("sh")
      .arg("-c")
      .arg(bin_path)
      .output()
      .expect("failed to execute process")
  };
}
