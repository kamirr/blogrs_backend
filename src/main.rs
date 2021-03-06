#![feature(proc_macro_hygiene, decl_macro)]
#![feature(label_break_value)]
#![feature(slice_concat_ext)]

#![allow(proc_macro_derive_resolution_fallback)]

#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket;
extern crate pulldown_cmark;
extern crate num_cpus;
extern crate rand;
extern crate sha2;
extern crate r2d2;

mod guards;
mod auth;
mod api;
mod db;

fn main() {
    let static_files = rocket_contrib
        ::serve
        ::StaticFiles
        ::from("frontend/");

    rocket::ignite()
        .manage(db::connection::pool())
        .mount("/api", api::routes())
        .mount("/", static_files)
        .launch();
}
