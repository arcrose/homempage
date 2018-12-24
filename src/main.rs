#![feature(custom_attribute, proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate rocket_contrib;

use std::collections::HashMap;
use std::path::Path;

use rocket::{
  http::ContentType,
  response::{NamedFile, Response},
};
use rocket_contrib::templates::Template;


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

#[get("/")]
fn index() -> Template {
  let context: HashMap<String, String> = HashMap::new();
  Template::render("index", context)
}

fn main() {
  rocket::ignite()
    .mount("/", routes![index, css, js])
    .attach(Template::fairing())
    .launch();
}
