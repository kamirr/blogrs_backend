#![feature(proc_macro_hygiene, decl_macro)]
#![feature(label_break_value)]
#![feature(slice_concat_ext)]
#![allow(proc_macro_derive_resolution_fallback)]

#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket;
extern crate rocket_contrib;
extern crate dotenv;
extern crate rand;
extern crate sha2;

mod authentication;
mod manage_posts;
mod connection;
mod schema;
mod models;

fn parse_entry_templ(entry: &String, id: u64) -> Option<String> {
    Some(entry.replace(
        "{ID}",
        &format!("{}", id)
    ))
}

#[get("/post/<id>")]
fn post_templ(id: u64) -> Option<String> {
    let entry = "frontend/templates/entry.html";
    match std::fs::read_to_string(entry) {
        Ok(text) => parse_entry_templ(&text, id),
        _ => None
    }
}

fn main() {
    use rocket_contrib::serve::StaticFiles;

    rocket::ignite()
        .mount("/", StaticFiles::from("frontend/"))
        .mount("/", routes![post_templ])
        .launch();
}
