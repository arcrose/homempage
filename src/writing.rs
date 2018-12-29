use std::fs;
use std::io::{self, Read};
use std::path::Path;


#[derive(Debug, Serialize)]
pub struct Line {
  pub text: String,
}

#[derive(Debug, Serialize)]
pub struct Sample {
  pub lines: Vec<Line>,
}

pub type CollectResult = Result<Vec<Sample>, io::Error>;

pub fn collect<P: AsRef<Path>>(directory: P) -> CollectResult {
  let dir_contents = fs::read_dir(directory.as_ref())?;
  let sources = dir_contents
    .filter(Result::is_ok)
    .map(Result::unwrap)
    .map(|file| file.path())
    .filter(|path| path.is_file());

  let mut samples = Vec::new();
  for source in sources {
    let mut content = String::new();
    fs::File::open(source)?.read_to_string(&mut content)?;

    let lines = content
      .split("\n")
      .map(|s| Line {
        text: s.trim().to_string(),
      })
      .collect::<Vec<Line>>();
    samples.push(Sample{ lines });
  }

  Ok(samples)
}
