
use lazy_static;
use regex::Regex;
use model::Config;
use utils;


lazy_static! {
  pub static ref VERSION_RE: Regex = Regex::new(r"^\d+\.\d+\.\d+$").unwrap();
  pub static ref ABOVE_VERSION_RE: Regex = Regex::new(r"^\^\d+\.\d+\.\d+$").unwrap();
  pub static ref CONFIG: Config = utils::get_config();
}
