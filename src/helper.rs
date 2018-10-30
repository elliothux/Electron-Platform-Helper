
use std::{fs, env};
use std::path::{Path, PathBuf};
use std::process::Command;
use os_info;
use model::{Platform};
use utils::{path_buf_to_string, is_path_exist};


fn get_platform() -> String {
  match os_info::get().os_type() {
    os_info::Type::Macos => String::from("darwin"),
    os_info::Type::Windows => String::from("win32"),
    _ => String::from("linux")
  }
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
  let home_path = path_buf_to_string(env::home_dir().unwrap());
  let platform_path = path_buf_to_string(
    Path::new(&home_path)
      .join(Path::new(".electron-platform"))
  );
  if !is_path_exist(&platform_path) {
    return false;
  }

  let runtime_path = Path::new(&platform_path)
    .join(Path::new(
      &format!("runtime/{}/{}",
               get_platform_str(platform),
               version
      )
    ))
    .to_str()
    .unwrap()
    .to_owned();
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
    println!("current: {}", &path_buf_to_string(current_path.clone()));
    println!("bin: {}", &bin_path);
    Command::new(&bin_path)
      .spawn()
      .expect("failed to execute process")
  };
}
