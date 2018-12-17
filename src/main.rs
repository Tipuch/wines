#![allow(proc_macro_derive_resolution_fallback)]
extern crate actix_web;
extern crate select;
extern crate reqwest;
extern crate futures;
extern crate csv;
extern crate serde;
extern crate argon2rs;
#[macro_use] extern crate failure;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate diesel;
extern crate bigdecimal;
extern crate chrono;
extern crate openssl;
mod errors;
mod schema;
mod models;
mod crawler;
mod controllers;
mod types;
use actix_web::middleware::identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{
    server, http, middleware, server, App
};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use controllers::*;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use std::env;
use chrono::Duration;

pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connection to {}", database_url))
}

fn main() {
    let domain: String = env::var("DOMAIN").unwrap_or_else(|_| "localhost".to_string());
    let secret_key = env::var("SECRET_KEY").expect("SECRET_KEY must be set");
    let app = App::new()
        .middleware(middleware::Logger::default())
        .middleware(IdentityService::new(
            CookieIdentityPolicy::new(secret_key.as_bytes())
                .name("auth")
                .path("/")
                .domain(domain.as_str())
                .max_age(Duration::days(30)) // just for testing
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
        });

    if domain == "localhost" {
        server::new(move || app)
            .bind("127.0.0.1:8080")
            .unwrap()
            .run();
    } else {
        let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
        builder.set_private_key_file("/etc/nginx/winecollections.ca.key", SslFiletype::PEM).unwrap();
        builder.set_certificate_chain_file("/etc/nginx/winecollections.ca.pem").unwrap();
        server::new(move || app)
            .bind_ssl("127.0.0.1:8080", builder)
            .unwrap()
            .start();
    }
   
}
