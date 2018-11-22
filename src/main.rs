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
extern crate chrono;
mod errors;
mod schema;
mod models;
mod crawler;
mod controllers;
mod types;
use actix_web::middleware::identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{
    http, middleware, server, App
};
use controllers::*;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use chrono::Duration;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connection to {}", database_url))
}

fn main() {
    dotenv().ok();
    let domain: String = env::var("DOMAIN").unwrap_or_else(|_| "localhost".to_string());
    let secret_key = env::var("SECRET_KEY").expect("SECRET_KEY must be set");

    server::new(move || {
        App::new()
        .middleware(middleware::Logger::default())
        .middleware(IdentityService::new(
            CookieIdentityPolicy::new(secret_key.as_bytes())
                .name("auth")
                .path("/")
                .domain(domain.as_str())
                .max_age(Duration::days(1)) // just for testing
                .secure(false),
        ))
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
