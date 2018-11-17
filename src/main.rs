#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate web_view;
extern crate toml;
extern crate regex;
#[macro_use]
extern crate lazy_static;
extern crate reqwest;
extern crate unzip;
extern crate timer;
extern crate chrono;

pub mod model;
pub mod utils;
pub mod helper;
pub mod installer;
pub mod downloader;
pub mod statics;
pub mod rpc;

use std::{env, path::Path};
use regex::Regex;
use utils::{path_buf_to_string};


fn main() {
  let config = &statics::CONFIG;
  if config.installed {
    helper::open_app_bin();
    return;
  }

  match helper::get_valid_runtime_path(&config.runtime) {
    None => {
      installer::open_install_helper();
    },
    Some((version, runtime_path)) => {
      helper::link_runtime(&runtime_path);
      // TODO: mark installed
      helper::open_app_bin();
    }
  }
}
