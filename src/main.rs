#![feature(custom_attribute, proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;

mod code;
mod writing;

use std::fs;
use std::io::Write;
use std::path::Path;

use rocket::{
  http::ContentType,
  response::{
    content::Html,
    NamedFile,
    Redirect,
    Response,
  },
};


#[get("/css/<filename>")]
fn css<'r>(filename: String) -> Response<'r> {
  let file_path = Path::new("css/").join(filename);

  if let Ok(file) = NamedFile::open(file_path) {
    Response::build()
      .header(ContentType::CSS)
      .sized_body(file)
      .finalize()
  } else {
    let err_msg = "No such file";
    Response::build()
      .header(ContentType::Plain)
      .sized_body(::std::io::Cursor::new(err_msg))
      .finalize()
  }
}

#[get("/js/<filename>")]
fn js<'r>(filename: String) -> Response<'r> {
  let file_path = Path::new("js/").join(filename);

  if let Ok(file) = NamedFile::open(file_path) {
    Response::build()
      .header(ContentType::JavaScript)
      .sized_body(file)
      .finalize()
  } else {
    let err_msg = "No such file";
    Response::build()
      .header(ContentType::Plain)
      .sized_body(::std::io::Cursor::new(err_msg))
      .finalize()
  }
}

#[get("/resume")]
fn resume() -> Html<NamedFile> {
  Html(NamedFile::open("html/resume.html").unwrap())
}

#[get("/")]
fn index() -> Html<NamedFile> {
  Html(NamedFile::open("html/index.html").unwrap())
}

#[catch(404)]
fn not_found() -> Redirect {
  Redirect::to(uri!(index))
}

fn main() {
  let source_dirs = code::analyze("./snippets")
    .map(|dirs| serde_json::to_vec(&dirs).unwrap())
    .expect("Code analysis failed");
  let mut code_snippets_js = fs::OpenOptions::new()
    .write(true)
    .create(true)
    .open("./js/code_snippets.js")
    .expect("Could not open js/code_snippets.js");
  code_snippets_js.write(b"const CODE_SNIPPETS = ").unwrap();
  code_snippets_js.write_all(&source_dirs).unwrap();

  let writing_samples = writing::collect("./writing")
    .map(|samples| serde_json::to_vec(&samples).unwrap())
    .expect("Failed to load writing samples");
  let mut writing_samples_js = fs::OpenOptions::new()
    .write(true)
    .create(true)
    .open("./js/writing_samples.js")
    .expect("Could not open js/writing_samples.js");
  writing_samples_js.write(b"const WRITING_SAMPLES = ").unwrap();
  writing_samples_js.write_all(&writing_samples).unwrap();

  rocket::ignite()
    .register(catchers![not_found])
    .mount("/", routes![index, resume, css, js])
    .launch();
}
