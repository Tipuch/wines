#![allow(proc_macro_derive_resolution_fallback)]
extern crate actix_web;
extern crate select;
extern crate reqwest;
extern crate dotenv;
extern crate futures;
extern crate csv;
extern crate serde;
extern crate argon2rs;
#[macro_use] extern crate failure;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;
extern crate bigdecimal;
mod errors;
mod schema;
mod models;
mod crawler;
mod controllers;
mod types;
use actix_web::{
    http, middleware, server, App
};
use controllers::*;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connection to {}", database_url))
}

fn main() {
    server::new(|| {
        App::new()
        .middleware(middleware::Logger::default())
        .resource("/", |r| {
            r.method(http::Method::GET).with(index);
            r.method(http::Method::POST).with(upload);
        }).resource("/crawl/", |r| {
            r.method(http::Method::POST).with(crawl_saq_controller);
        }).resource("/users/", |r| {
            r.method(http::Method::POST).with(register);
        }).resource("/login/", |r| {
            r.method(http::Method::POST).with(login);
        }).resource("/logout/", |r| {
            r.method(http::Method::POST).with(logout);
        }).resource("/wines/", |r| {
            r.method(http::Method::GET).with(get_wine_recommendations);
        })
        }).bind("127.0.0.1:8080")
        .unwrap()
        .run();
}
