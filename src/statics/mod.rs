
use lazy_static;
use regex::Regex;


lazy_static! {
  pub static ref VERSION_RE: Regex = Regex::new(r"^\d+\.\d+\.\d+$").unwrap();
  pub static ref ABOVE_VERSION_RE: Regex = Regex::new(r"^\^\d+\.\d+\.\d+$").unwrap();
}
