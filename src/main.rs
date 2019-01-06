#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use rocket::response::NamedFile;
use rocket::State;
use std::path::Path;
use std::collections::HashMap;
use std::sync::Arc;

#[get("/favicon.ico")]
fn favicon() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/favicon.ico")).ok()
}

#[get("/")]
fn index() -> &'static str {
    "нет"
}

#[get("/<phrase>")]
fn russian(phrase: String, db: State<Arc<Phrases>>) -> String {
    match db.get(&phrase) {
        Some(russian) => russian.clone(),
        None => "Phrase unknown".to_string(),
    }
}

type Phrases = HashMap<String, String>;

fn main() {
    let mut db = Phrases::new();
    db.insert(
        "good morning".to_string(),
        "доброе утро".to_string()
    );
    rocket::ignite()
        .manage(Arc::new(db))
        .mount("/", routes![russian, favicon, index])
        .launch();
}
