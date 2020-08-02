use chrono::{Local, NaiveDate};
use serde::{Deserialize, Serialize};

use std::{
  fmt,
  path::{Path, PathBuf},
};
use structopt::StructOpt;

#[derive(Debug)]
enum ConfigError {
  TemplateNotFound { path: PathBuf },
  InvalidDate { variable: String, value: String },
}

impl fmt::Display for ConfigError {
  fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
    match self {
      ConfigError::TemplateNotFound { path } => write!(
        fmt,
        "Template not found, path was: {path}",
        path = std::fs::canonicalize(path).unwrap().to_str().unwrap()
      ),
      ConfigError::InvalidDate { variable, value } => write!(
        fmt,
        "Could not parse date, config variable: {var}, value: {value}",
        var = variable,
        value = value
      ),
    }
  }
}

#[derive(Debug, Serialize, Deserialize, StructOpt)]
#[structopt(name = "harvest-daily")]
pub struct Config {
  #[structopt(short, long = "template", parse(from_os_str))]
  pub template_path: Option<PathBuf>,
  #[structopt(short, long = "start")]
  pub start_date: Option<NaiveDate>,
  #[structopt(short, long = "end")]
  pub end_date: Option<NaiveDate>,
}

impl Default for Config {
  fn default() -> Self {
    Config {
      template_path: None,
      start_date: Some(Local::today().naive_local()),
      end_date: Some(Local::today().naive_local()),
    }
  }
}

impl Config {
  pub fn new() -> Result<Config, confy::ConfyError> {
    let config = Config::from_toml("settings.toml").unwrap();
    let args = Config::from_args();

    Ok(Config::merge(vec![config, args]))
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
