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
    match dotenv::dotenv() {
      Ok(path) => println!(
        "Loaded environmental variables from path: {}",
        path.to_string_lossy()
      ),
      Err(_) => print!("Ignoring loading .env file, as it was not found..."),
    }
    envy::from_env::<HarvestCredentials>()
  }
}
