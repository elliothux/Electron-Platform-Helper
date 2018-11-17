
pub enum Platform {
  DARWIN,
  WIN32,
  WIN64,
  LINUX32,
  LINUX64,
  UNKNOWN
}

pub type Version = (u8, u8, u8);

#[derive(Deserialize, Debug)]
pub struct ReleaseResponse {
  pub name: String
}

#[derive(Deserialize)]
pub struct Config {
  pub target: String,
  pub runtime: String,
  pub installed: bool
}