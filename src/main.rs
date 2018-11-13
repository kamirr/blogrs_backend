#![feature(label_break_value)]
#![allow(proc_macro_derive_resolution_fallback)]

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate hyper;
extern crate sha2;

mod dynamic_content;
mod static_content;
mod manage_posts;
mod run_server;
mod connection;
mod content;
mod service;
mod schema;
mod models;

use crate::run_server::run_server;
use crate::content::Content;

fn main() {
    run_server(Content::new("frontend/"));
}
