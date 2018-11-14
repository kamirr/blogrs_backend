#![feature(proc_macro_hygiene, decl_macro)]
#![feature(label_break_value)]
#![feature(slice_concat_ext)]

#![allow(proc_macro_derive_resolution_fallback)]

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket;
extern crate rocket_contrib;
extern crate serde_json;
extern crate dotenv;
extern crate serde;
extern crate rand;

mod manage_posts;
mod connection;
mod posts_info;
mod html_post;
mod auth_key;
mod schema;
mod models;
mod login;

fn main() {
    use crate::connection::establish_sql_connection;
    use rocket_contrib::serve::StaticFiles;

    rocket::ignite()
        .manage(establish_sql_connection())
        .mount("/", StaticFiles::from("frontend/"))
        .mount("/posts", routes![posts_info::posts_info])
        .mount("/posts", routes![html_post::fetch_html_post])
        .mount("/login", routes![login::login])
        .launch();
}
