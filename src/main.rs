use std::error::Error;

mod harvest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  let mut harvest = harvest::HarvestClient::new();
  harvest.prepare().await?;
  println!("{:?}", harvest.time_entries().await);

  Ok(())
}
