use chrono::{Local, NaiveDate};
use serde::{Deserialize, Serialize};

use std::path::{Path, PathBuf};
use structopt::StructOpt;

#[derive(Debug, Serialize, Deserialize, StructOpt)]
#[structopt(name = "harvest-daily")]
pub struct Config {
  #[structopt(long = "template", parse(from_os_str))]
  #[serde(rename = "template")]
  /// Path to template which is being used to render entries from Harvest.
  /// Template should be Handlebars template with `.hbs` extension.
  pub template_path: Option<PathBuf>,
  #[structopt(short, long)]
  /// Start date for querying time entries in Harvest.
  /// Date in YYYY-MM-DD format.
  /// Defaults to current date.
  pub start_date: Option<NaiveDate>,
  #[structopt(short, long)]
  /// End date for querying time entries in Harvest.
  /// Date in YYYY-MM-DD format.
  /// Defaults to current date.
  pub end_date: Option<NaiveDate>,
  #[structopt(short, long = "config", parse(from_os_str))]
  #[serde(rename = "config")]
  /// Path to configuration file in TOML format.
  /// Check `src/config.rs` for more info about configuration.
  pub config_path: Option<PathBuf>,
  #[structopt(short = "a", long = "account", env = "HARVEST_ACCOUNT_ID")]
  #[serde(rename = "account")]
  /// Numeric account id from Harvest.
  /// Check https://id.getharvest.com/developers for more info.
  pub harvest_account_id: Option<u32>,
  #[structopt(
    short = "t",
    long = "token",
    env = "HARVEST_TOKEN",
    hide_env_values = true
  )]
  #[serde(rename = "token")]
  /// Personal access token for Harvest API.
  /// Check https://id.getharvest.com/developers for more info.
  pub harvest_token: Option<String>,
  #[structopt(short = "u", long = "user", env = "HARVEST_USER_ID")]
  #[serde(rename = "user")]
  /// Numeric user ID from which you want to query entries.
  /// Leave empty to use ID gathered from access token
  pub harvest_user_id: Option<u32>,
}

impl Default for Config {
  fn default() -> Self {
    Config {
      template_path: None,
      start_date: Some(Local::today().naive_local()),
      end_date: Some(Local::today().naive_local()),
      config_path: None,
      harvest_account_id: None,
      harvest_token: None,
      harvest_user_id: None,
    }
  }
}

impl Config {
  pub fn new() -> Result<Config, confy::ConfyError> {
    let args_config = Config::from_args();
    if let Some(path) = &args_config.config_path {
      let file_config = Config::from_toml(path).expect("Failed to read config from file");
      return Ok(Config::merge(vec![args_config, file_config]));
    }
    return Ok(args_config);
  }

  fn from_toml(path: impl AsRef<Path>) -> Result<Config, confy::ConfyError> {
    confy::load_path(path)
  }

  fn merge(configs: Vec<Config>) -> Config {
    let mut config = Config::default();
    configs.iter().for_each(|c| {
      if let Some(_) = c.template_path {
        config.template_path = c.template_path.clone();
      }
      if let Some(_) = c.start_date {
        config.start_date = c.start_date.clone();
      }

      if let Some(_) = c.end_date {
        config.end_date = c.end_date.clone();
      }

      if let Some(_) = c.config_path {
        config.config_path = c.config_path.clone();
      }

      if let Some(_) = c.harvest_account_id {
        config.harvest_account_id = c.harvest_account_id.clone();
      }

      if let Some(_) = c.harvest_token {
        config.harvest_token = c.harvest_token.clone();
      }

      if let Some(_) = c.harvest_user_id {
        config.harvest_user_id = c.harvest_user_id.clone();
      }
    });
    return config;
  }

  // fn validate(&self) -> Result<(), ConfigError> {
  //   if let Some(template_path) = &self.template_path {
  //     if let Err(err) = absolute_path(&template_path) {}
  //   }
  //   Ok(())
  // }
}

// fn absolute_path(src: &PathBuf) -> Result<PathBuf, io::Error> {
//   fs::canonicalize(src).map(|path| path)
// }
