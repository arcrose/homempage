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

pub Analysis(AnalysisState);

pub type AnalysisResult = Result<Vec<SourceCodeDirectory>, io::Error>;

enum AnalysisState {
  Start(Path),
  KnowWhereToSearch(Vec<Path>),
  Finish(Vec<SourceCodeDirectory>),
}

pub fn analyze<P: AsRef<Path>>(directory: P) -> Analysis {
  Analysis(AnalysisState::Start(p.as_ref().clone()))
}

pub fn run(a: Analysis(state)) -> AnalysisResult {
  use AnalysisState::*;

  match state {
    Start(path)              => search_for_snippets(path)
    KnowWhereToSearch(paths) => analyze_sources(paths),
    Finish(results)          => Ok(results),
  }
}

fn search_for_snippets(base: Path) -> AnalysisResult {
  Ok(vec![])
}

fn analyze_sources(file_paths: Vec<Path>) -> AnalysisResult {
  Ok(vec![])
}
