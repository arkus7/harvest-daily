use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
  pub template_path: Option<PathBuf>,
  pub date_from: Option<String>,
  pub date_to: Option<String>,
}

impl Default for Config {
  fn default() -> Self {
    Config {
      template_path: None,
      date_from: None,
      date_to: None,
    }
  }
}

impl Config {
  pub fn from_toml(path: impl AsRef<Path>) -> Result<Config, confy::ConfyError> {
    confy::load_path(path)
  }
}
