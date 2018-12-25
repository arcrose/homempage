use std::io;
use std::path::Path;

use serde_derive::{Deserialize, Serialize};


#[derive(Debug, Serialize)]
pub struct Line {
  pub number: u32,
  pub indent: u32,
  pub code: String,
}

#[derive(Debug, Serialize)]
pub struct Source {
  #[serde(rename = "fileName")]
  pub file_name: String,
  #[serde(rename = "linesOfCode")]
  pub lines_of_code: Vec<Line>,
}

#[derive(Debug, Serialize)]
pub struct SourceCodeDirectory {
  #[serde(rename = "languageName")]
  pub language_name: String,
  #[serde(rename = "sourceFiles")]
  pub source_files: Vec<Source>,
}

pub fn analyze<P: AsRef<Path>>(directory: P) -> Result<Vec<SourceCodeDirectory>, io::Error> {
  Ok(vec![])
}
