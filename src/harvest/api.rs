use super::credentials::HarvestCredentials;
use chrono::{DateTime, Local};
use reqwest::header;
use serde::{Deserialize, Serialize};

const USER_AGENT: &str = "harvest-daily";
const ACCOUNT_ID_HEADER: &str = "Harvest-Account-ID";
const BASE_URL: &str = "https://api.harvestapp.com/api/v2";

fn attach_credentials(
  builder: reqwest::RequestBuilder,
  credentials: &HarvestCredentials,
) -> reqwest::RequestBuilder {
  builder
    .header(header::USER_AGENT, USER_AGENT)
    .header(header::AUTHORIZATION, bearer_token(&credentials.token))
    .header(ACCOUNT_ID_HEADER, format!("{}", &credentials.account_id))
}

fn bearer_token(token: &str) -> String {
  format!("Bearer {token}", token = token)
}

fn route(path: &str) -> String {
  format!("{base}{path}", base = BASE_URL, path = path)
}

pub async fn current_user(credentials: &HarvestCredentials) -> Result<User, reqwest::Error> {
  let url = route("/users/me");
  let builder = reqwest::Client::new().get(&url);
  let builder = attach_credentials(builder, &credentials);
  builder.send().await?.json().await
}

pub async fn time_entries(
  query: &query::TimeEntriesQuery,
  credentials: &HarvestCredentials,
) -> Result<TimeEntries, reqwest::Error> {
  let url = route("/time_entries");
  let builder = reqwest::Client::new().get(&url).query(query);
  let builder = attach_credentials(builder, &credentials);
  builder.send().await?.json().await
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TimeEntry {
  pub id: u32,
  pub spent_date: String,
  pub hours: f64,
  pub rounded_hours: f64,
  pub created_at: DateTime<Local>,
  pub user: User,
  pub project: Project,
  pub task: Task,
  pub notes: String,
  pub timer_started_at: Option<DateTime<Local>>,
  pub started_time: Option<String>,
  pub ended_time: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TimeEntries {
  pub time_entries: Vec<TimeEntry>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct User {
  pub id: u32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Project {
  pub id: u32,
  pub name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Task {
  pub id: u32,
  pub name: String,
}

pub mod query {
  use chrono::NaiveDate;
  use serde::Serialize;
  #[derive(Serialize, Debug)]
  pub struct TimeEntriesQuery {
    pub user_id: u32,
    pub from: Option<NaiveDate>,
    pub to: Option<NaiveDate>,
  }
}
