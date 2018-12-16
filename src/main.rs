#![feature(custom_attribute, proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate rocket_contrib;

use std::collections::HashMap;
use std::path::Path;

use rocket::response::NamedFile;
use rocket_contrib::templates::Template;


#[get("/css/<filename>")]
fn css(filename: String) -> Option<NamedFile> {
  NamedFile::open(Path::new("css/").join(filename)).ok()
}

#[get("/")]
fn index() -> Template {
  let context: HashMap<String, String> = HashMap::new();
  Template::render("index", context)
}

fn main() {
  rocket::ignite()
    .mount("/", routes![index, css])
    .attach(Template::fairing())
    .launch();
}
