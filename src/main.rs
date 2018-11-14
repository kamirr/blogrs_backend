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
mod post;

fn main() {
    use crate::connection::establish_sql_connection;
    use rocket_contrib::serve::StaticFiles;

    rocket::ignite()
        .manage(establish_sql_connection())
        .mount("/", StaticFiles::from("frontend/"))
        .mount("/", routes![post::html_post])
        .launch();
}
