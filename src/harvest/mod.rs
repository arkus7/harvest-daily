mod api;
mod credentials;

use api::query::TimeEntriesQuery;
use api::TimeEntries;
use credentials::HarvestCredentials;

use chrono::offset::TimeZone;
use chrono::Datelike;
use chrono::Local;

#[derive(Debug)]
pub struct HarvestClient {
  credentials: HarvestCredentials,
  user_id: Option<u32>,
}

impl HarvestClient {
  pub fn new() -> HarvestClient {
    HarvestClient {
      credentials: HarvestCredentials::from_env()
        .expect("Could not load harvest credentials from envs"),
      user_id: None,
    }
  }

  pub async fn prepare(&mut self) -> Result<(), reqwest::Error> {
    let user = api::current_user(&self.credentials).await?;
    self.user_id = Some(user.id);

    Ok(())
  }

  pub async fn time_entries(&self) -> Result<TimeEntries, reqwest::Error> {
    let today = Local::now().date();
    let query = TimeEntriesQuery {
      user_id: self.user_id.expect("HarvestClient was not prepared"),
      from: Some(
        Local
          // .ymd(today.year(), today.month(), today.day())
          .ymd(today.year(), 7, 29)
          .and_hms(0, 0, 0),
      ),
    };

    api::time_entries(&query, &self.credentials).await
  }
}
