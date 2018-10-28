#![allow(proc_macro_derive_resolution_fallback)]
extern crate actix_web;
extern crate select;
extern crate reqwest;
extern crate dotenv;
extern crate futures;
extern crate csv;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;
extern crate bigdecimal;
mod schema;
mod models;
mod controllers;
use actix_web::{
    http, middleware, server, App
};
use controllers::{index, upload};
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;


mod types {
    use diesel::pg::{Pg};
    use diesel::deserialize;
    use diesel::deserialize::FromSql;
    use diesel::serialize;
    use diesel::serialize::{Output, ToSql, IsNull};
    use std::io::Write;

    #[derive(SqlType)]
    #[postgres(type_name = "wine_color")]
    pub struct WineColor;

    #[derive(Debug, PartialEq, FromSqlRow, AsExpression)]
    #[sql_type = "WineColor"]
    pub enum WineColorEnum {
        Red,
        White,
        Pink
    }

    impl ToSql<WineColor, Pg> for WineColorEnum {
        fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
            match *self {
                WineColorEnum::Red => out.write_all(b"red")?,
                WineColorEnum::White => out.write_all(b"white")?,
                WineColorEnum::Pink => out.write_all(b"pink")?,
            }
            Ok(IsNull::No)
        }
    }

    impl FromSql<WineColor, Pg> for WineColorEnum {
        fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
            match not_none!(bytes) {
                b"red" => Ok(WineColorEnum::Red),
                b"white" => Ok(WineColorEnum::White),
                b"pink" => Ok(WineColorEnum::Pink),
                _ => Err("Unrecognized enum variant".into()),
            }
        }
    }
}

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
        })})
        .bind("127.0.0.1:8080")
        .unwrap()
        .run();
}
