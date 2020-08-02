use envy;
use serde::Deserialize;
#[derive(Deserialize, Debug)]
pub struct HarvestCredentials {
  #[serde(rename = "harvest_account_id")]
  pub account_id: String,
  #[serde(rename = "harvest_token")]
  pub token: String,
}

impl HarvestCredentials {
  pub fn from_env() -> Result<HarvestCredentials, envy::Error> {
    dotenv::dotenv().expect("Cannot load .env file");
    envy::from_env::<HarvestCredentials>()
  }
}
