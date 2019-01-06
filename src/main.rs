#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use rocket::response::NamedFile;
use rocket::State;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
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

    let file = BufReader::new(File::open(Path::new("static/phrases.txt")).unwrap());
    file.lines().for_each(|l| {
        let line = l.unwrap();
        let kvp: Vec<_> = line.split(",").collect();
        if kvp.len() < 2 { return; }
        println!("Loading {} = {}", kvp[0], kvp[1]);
        db.insert(kvp[0].to_string(), kvp[1].to_string());
    });
    rocket::ignite()
        .manage(Arc::new(db))
        .mount("/", routes![russian, favicon, index])
        .launch();
}
