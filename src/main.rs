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
extern crate sha2;

mod manage_posts;
mod connection;
mod posts_info;
mod html_post;
mod auth_key;
mod schema;
mod models;
mod logout;
mod login;

fn main() {
    use crate::connection::establish_sql_connection;
    use rocket_contrib::serve::StaticFiles;

    let posts_routes = routes![
        posts_info::posts_info,
        html_post::fetch_html_post
    ];

    let api_routes = routes![
        login::login,
        logout::logout
    ];

    let static_route = StaticFiles::from("frontend/");

    rocket::ignite()
        .manage(establish_sql_connection())
        .mount("/posts", posts_routes)
        .mount("/api", api_routes)
        .mount("/", static_route)
        .launch();
}
