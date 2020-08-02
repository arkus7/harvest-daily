use std::error::Error;

mod harvest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  let mut harvest = harvest::HarvestClient::new();
  harvest.prepare().await?;
  let entries = harvest.time_entries().await?.time_entries;
  println!("{:?}", entries);

  entries.iter().for_each(|e| {
    println!(
      "{when},{duration},{what},{project}",
      when = e.spent_date,
      duration = e.hours,
      what = format!("{} - {}", e.task.name, e.notes),
      project = e.project.name
    )
  });

  println!("{}", entries.iter().map(|e| e.hours).sum::<f64>());

  Ok(())
}
