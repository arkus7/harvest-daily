use std::error::Error;

mod config;
mod harvest;
mod template;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  load_env();
  let cfg = config::Config::new().unwrap();
  let mut harvest = harvest::HarvestClient::new(&cfg);
  if cfg.harvest_user_id == None {
    harvest.prepare().await?;
  }
  let time_entries = harvest.time_entries().await?;
  println!("{:?}", time_entries.group_by_date());
  let entries = &time_entries.time_entries;
  let result = template::render_file_template(entries, &cfg.template_path.unwrap()).unwrap();
  println!("{}", result);

  Ok(())
}

fn load_env() {
  match dotenv::dotenv() {
    Ok(path) => println!(
      "Loaded environmental variables from path: {}",
      path.to_string_lossy()
    ),
    Err(_) => print!("Ignoring loading .env file, as it was not found..."),
  }
}
