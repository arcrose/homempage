use std::fs;
use std::io;
use std::path::Path;


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

pub type AnalysisResult = Result<Vec<SourceCodeDirectory>, io::Error>;

pub fn analyze<P: AsRef<Path>>(directory: P) -> AnalysisResult {
  let dir_contents = fs::read_dir(directory.as_ref())?;
  let directories = dir_contents
    .filter(Result::is_ok)
    .map(Result::unwrap)
    .map(|file| file.path())
    .filter(|path| path.is_dir());

  let mut source_dirs = Vec::new();
  for dir in directories {
    let language = dir
      .to_str()
      .unwrap()
      .split("/")
      .last()
      .unwrap()
      .to_string();
    let mut sources = Vec::new();

    let dir_contents = fs::read_dir(dir)?;
    let files = dir_contents
      .filter(Result::is_ok)
      .map(Result::unwrap)
      .map(|file| file.path())
      .filter(|path| path.is_file());

    for file in files {
      let source = process(&language, file)?;
      sources.push(source);
    }

    source_dirs.push(SourceCodeDirectory {
      language_name: language,
      source_files: sources,
    });
  }

  Ok(source_dirs)
}

fn process<S, P>(language_name: S, file_path: P) -> Result<Source, io::Error> {
  Ok(Source {
    file_name: "test".to_string(),
    lines_of_code: vec![],
  })
}
