use crate::config::Config;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct HarvestCredentials {
  #[serde(rename = "harvest_account_id")]
  pub account_id: u32,
  #[serde(rename = "harvest_token")]
  pub token: String,
}

impl HarvestCredentials {
  pub fn from_cfg(config: &Config) -> HarvestCredentials {
    let token = config
      .harvest_token
      .clone()
      .expect("Harvest: Missing token");
    let account_id = config
      .harvest_account_id
      .expect("Harvest: Missing Account ID");
    HarvestCredentials { account_id, token }
  }
}
