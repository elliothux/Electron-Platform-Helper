#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate web_view;
extern crate toml;
extern crate regex;
#[macro_use]
extern crate lazy_static;

pub mod model;
pub mod utils;
pub mod helper;
pub mod installer;

use std::{env, path::Path};
use regex::Regex;
use utils::{path_buf_to_string};


fn main() {
  let config = utils::get_config();
  if config.installed {
    helper::open_app_bin();
    return;
  }

  match helper::get_valid_runtime_path(&config.runtime) {
    None => {
      // TODO: install runtime
      return;
      installer::open_install_helper();
    },
    Some((version, runtime_path)) => {
      // TODO: link runtime to app
      println!("{:?} - {}", &version, runtime_path.display());
      helper::link_runtime(&runtime_path);
      return;
      helper::open_app_bin();
    }
  }
}
