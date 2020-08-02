mod api;
mod credentials;

use api::query::TimeEntriesQuery;
use api::{attach_credentials, route, TimeEntries, User};
use credentials::HarvestCredentials;

#[derive(Debug)]
pub struct HarvestClient {
  credentials: HarvestCredentials,
  user_id: Option<u32>,
}

impl HarvestClient {
  pub fn new() -> HarvestClient {
    HarvestClient {
      credentials: HarvestCredentials::from_env()
        .expect("Could not load harvest configuration from envs"),
      user_id: None,
    }
  }

  pub async fn prepare(&mut self) -> Result<(), reqwest::Error> {
    let url = route("/users/me");
    let builder = reqwest::Client::new().get(&url);
    let builder = attach_credentials(builder, &self.credentials);
    let user: User = builder.send().await?.json().await?;

    self.user_id = Some(user.id);

    Ok(())
  }

  pub async fn time_entries(&self) -> Result<TimeEntries, reqwest::Error> {
    let url = route("/time_entries");
    let builder = reqwest::Client::new()
      .get(&url)
      // .query(&[("user_id", self.user_id.unwrap())]);
      .query(&TimeEntriesQuery {
        user_id: self.user_id.unwrap(),
      });
    let builder = attach_credentials(builder, &self.credentials);
    println!("{:?}", builder);
    builder.send().await?.json().await
  }
}
