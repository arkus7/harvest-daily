use crate::harvest::api::{Project, TimeEntry};
use chrono::NaiveDate;
use handlebars::Handlebars;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

struct TemplateData {
  dates: Vec<EntriesByDate>,
}

struct EntriesByDate {
  date: NaiveDate,
  projects: Vec<EntriesByProject>,
}

struct EntriesByProject {
  project: Project,
  entries: Vec<TimeEntry>,
}

pub fn render_file_template(
  entries: &Vec<TimeEntry>,
  template_path: &PathBuf,
) -> Result<String, handlebars::TemplateRenderError> {
  let template = read_template(&template_path).map_err(|e| {
    handlebars::TemplateRenderError::IOError(
      e,
      format!(
        "could not load template at path: {path}",
        path = template_path.to_string_lossy()
      ),
    )
  })?;
  render(entries, template.as_str())
}

fn render(
  entries: &Vec<TimeEntry>,
  template: &str,
) -> Result<String, handlebars::TemplateRenderError> {
  let handlebars = Handlebars::new();

  handlebars.render_template(template, &entries)
}

fn read_template(path: &PathBuf) -> Result<String, std::io::Error> {
  let mut source_template = File::open(&path)?;
  let mut template: String = String::from("");
  source_template.read_to_string(&mut template).unwrap();
  Ok(template)
}
