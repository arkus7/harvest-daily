pub mod api;
mod credentials;

use super::config::Config;
use api::query::TimeEntriesQuery;
use api::TimeEntries;
use credentials::HarvestCredentials;

use chrono::offset::TimeZone;
use chrono::Datelike;
use chrono::{Local, NaiveDate};

#[derive(Debug)]
pub struct HarvestClient {
  credentials: HarvestCredentials,
  user_id: Option<u32>,
  query: QueryOptions,
}

#[derive(Debug)]
struct QueryOptions {
  from: Option<NaiveDate>,
  to: Option<NaiveDate>,
}

impl HarvestClient {
  pub fn new(config: &Config) -> HarvestClient {
    HarvestClient {
      credentials: HarvestCredentials::from_cfg(config),
      user_id: config.harvest_user_id,
      query: QueryOptions {
        from: config.start_date,
        to: config.end_date,
      },
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
      user_id: self
        .user_id
        .expect("No user ID, provide user ID or use `prepare` method"),
      from: self.query.from,
      to: self.query.to,
    };

    api::time_entries(&query, &self.credentials).await
  }
}
