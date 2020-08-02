use super::credentials::HarvestCredentials;
use reqwest::header;
use serde::Deserialize;

const USER_AGENT: &str = "harvest-daily";
const ACCOUNT_ID_HEADER: &str = "Harvest-Account-ID";
const BASE_URL: &str = "https://api.harvestapp.com/api/v2";

pub fn attach_credentials(
  builder: reqwest::RequestBuilder,
  credentials: &HarvestCredentials,
) -> reqwest::RequestBuilder {
  builder
    .header(header::USER_AGENT, USER_AGENT)
    .header(header::AUTHORIZATION, bearer_token(&credentials.token))
    .header(ACCOUNT_ID_HEADER, &credentials.account_id)
}

fn bearer_token(token: &str) -> String {
  format!("Bearer {token}", token = token)
}

pub fn route(path: &str) -> String {
  format!("{base}{path}", base = BASE_URL, path = path)
}

#[derive(Deserialize, Debug)]
pub struct TimeEntry {
  pub id: u32,
  pub spent_date: String,
  pub hours: f64,
  pub rounded_hours: f64,
  pub created_at: String,
  pub user: User,
  pub project: Project,
}

#[derive(Deserialize, Debug)]
pub struct TimeEntries {
  pub time_entries: Vec<TimeEntry>,
}

#[derive(Deserialize, Debug)]
pub struct User {
  pub id: u32,
}

#[derive(Deserialize, Debug)]
pub struct Project {
  pub id: u32,
  pub name: String,
}

pub mod query {
  use serde::{Deserialize, Serialize};
  #[derive(Deserialize, Serialize, Debug)]
  pub struct TimeEntriesQuery {
    pub user_id: u32,
  }
}
