use std::fs;
use std::io::{self, Read};
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
      let source = process(file)?;
      sources.push(source);
    }

    source_dirs.push(SourceCodeDirectory {
      language_name: language,
      source_files: sources,
    });
  }

  Ok(source_dirs)
}

enum IndentCounterSM {
  Start {
    source_code: String,
  },
  Processing {
    file_name: String,
    lines: Vec<Line>,
    processing_index: usize,
    indent_guess: &'static str,
  },
  Finished(Source),
}

fn process<P: AsRef<Path>>(file_path: P) -> Result<Source, io::Error> {
  use self::IndentCounterSM::*;

  let file_name = file_path
    .as_ref()
    .file_name()
    .unwrap()
    .to_str()
    .unwrap()
    .to_string();
  let mut source_code = String::new();
  fs::File::open(file_path)
    ?.read_to_string(&mut source_code)?;

  let mut state = Start{ source_code };
  loop {
    state = match state {
      Start{ source_code } => tokenize(file_name.clone(), source_code),
      p@Processing{ .. }   => proceed(p),
      Finished(source)     => return Ok(source),
    }
  }
}

fn proceed(mut state: IndentCounterSM) -> IndentCounterSM {
  if let IndentCounterSM::Processing{
    file_name,
    mut lines,
    processing_index,
    indent_guess,
  } = state {
    // Finished condition
    if processing_index >= lines.len() {
      IndentCounterSM::Finished(Source {
        file_name,
        lines_of_code: lines,
      })
    } else {
      let (new_line, new_indent_guess) = update_indent_guess(&lines[processing_index], indent_guess);
      lines[processing_index] = new_line;

      IndentCounterSM::Processing {
        file_name,
        lines,
        processing_index: processing_index + 1,
        indent_guess: new_indent_guess,
      }
    }
  } else {
    state
  }
}

fn update_indent_guess(line: &Line, indent_guess: &'static str) -> (Line, &'static str) {
  let mut line = Line {
    number: line.number,
    indent: line.indent,
    code: line.code.clone(),
  };

  if line.code.starts_with(" ") {
    let num_spaces = line.code
      .chars()
      .take_while(|&c| c == ' ')
      .collect::<Vec<char>>()
      .len();
    if num_spaces % 4 == 0 {
      line.indent = (num_spaces / 4) as u32;

      (line, "    ")
    } else {
      line.indent = (num_spaces / 2) as u32;

      (line, "  ")
    }
  } else {
    let num_tabs = line.code
      .chars()
      .take_while(|&c| c == '\t')
      .collect::<Vec<char>>()
      .len();
    line.indent = num_tabs as u32;

    (line, "\t")
  }
}

fn tokenize(file_name: String, source_code: String) -> IndentCounterSM {
  let mut lines = Vec::new();
  let mut line_no = 0;

  for line_str in source_code.split("\n") {
    lines.push(Line {
      number: line_no,
      indent: 0,
      code: line_str.to_string(),
    });
    line_no += 1;
  }

  IndentCounterSM::Processing {
    file_name,
    lines,
    processing_index: 0,
    indent_guess: "    ",
  }
}
