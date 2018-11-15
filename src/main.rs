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

mod manage_posts_table;
mod set_login_param;
mod manage_posts;
mod connection;
mod auth_key;
mod webpost;
mod schema;
mod models;
mod logout;
mod login;
mod post;
mod meta;

fn main() {
    use crate::connection::establish_sql_connection;
    use rocket_contrib::serve::StaticFiles;

    let api_routes = routes![
        login::login,
        logout::logout,
        meta::meta,
        post::post,
        manage_posts::new,
        manage_posts::update,
        manage_posts::delete,
        set_login_param::set_login,
        set_login_param::set_password
    ];

    let static_route = StaticFiles::from("frontend/");

    rocket::ignite()
        .manage(establish_sql_connection())
        .mount("/api", api_routes)
        .mount("/", static_route)
        .launch();
}
