#![allow(proc_macro_derive_resolution_fallback)]
extern crate actix_files;
extern crate actix_identity;
extern crate actix_rt;
extern crate actix_web;
extern crate argon2rs;
extern crate csv;
extern crate futures;
extern crate reqwest;
extern crate select;
extern crate serde;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate diesel;
extern crate bigdecimal;
extern crate dotenv;
extern crate openssl;
mod controllers;
mod crawler;
mod errors;
mod models;
mod schema;
mod types;
mod utils;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::*;
use actix_web::{middleware, App, HttpServer};
use controllers::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use std::env;

pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connection to {}", database_url))
}

fn main() {
    let domain: String = env::var("DOMAIN").unwrap_or_else(|_| "localhost".to_string());
    let bind_address = "127.0.0.1:8080";
    let is_localhost = domain == "localhost";
    if is_localhost {
        dotenv::dotenv().ok();
    }
    let secret_key = env::var("SECRET_KEY").expect("SECRET_KEY must be set");
    let sys = actix_rt::System::new("wines");
    let serv = HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(secret_key.as_bytes())
                    .name("auth")
                    .path("/")
                    .domain(domain.as_str())
                    .max_age(2592000)
                    .secure(true),
            ))
            .service(web::resource("/").route(web::get().to(index)))
            .service(actix_files::Files::new("/static/js", "./static/dist/").show_files_listing())
            .service(actix_files::Files::new("/static/css", "./static/css/").show_files_listing())
            .service(actix_files::Files::new("/static/img", "./static/img/").show_files_listing())
            .service(web::resource("/health/").route(web::get().to(get_health)))
            .service(web::resource("/crawl/").route(web::post().to(crawl_saq_controller)))
            .service(web::resource("/users/").route(web::post().to(register)))
            .service(web::resource("/login/").route(web::post().to(login)))
            .service(web::resource("/logout/").route(web::post().to(logout)))
            .service(web::resource("/wines/").route(web::get().to(get_wines)))
            .service(
                web::resource("/winerecommendations/")
                    .route(web::post().to(create_wine_reco))
                    .route(web::get().to(get_wine_reco)),
            )
            .service(
                web::resource("/winerecommendations/{wine_recommendation_id}/")
                    .route(web::put().to(update_wine_reco))
                    .route(web::delete().to(delete_wine_reco)),
            )
    });

    if is_localhost {
        serv.bind(bind_address).unwrap().start();
    } else {
        let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
        builder
            .set_private_key_file("/etc/nginx/winecollections.ca.key", SslFiletype::PEM)
            .unwrap();
        builder
            .set_certificate_chain_file("/etc/nginx/winecollections.ca.pem")
            .unwrap();
        serv.bind_ssl(bind_address, builder).unwrap().start();
    }
    println!("Started http server: {}", bind_address);
    let _ = sys.run();
}
