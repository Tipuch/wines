#![allow(proc_macro_derive_resolution_fallback)]
extern crate actix_web;
extern crate select;
extern crate reqwest;
extern crate dotenv;
extern crate futures;
extern crate csv;
extern crate serde;
extern crate argon2rs;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;
extern crate bigdecimal;
mod schema;
mod models;
mod crawler;
mod controllers;
use actix_web::{
    http, middleware, server, App
};
use controllers::{index, upload, crawl_saq_controller, register};
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
    use std::fmt;
    use serde::{Deserialize, Deserializer};
    use serde::de::{self, Visitor};

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

    struct WineColorVisitor;

    impl<'de> Visitor<'de> for WineColorVisitor {
        type Value = WineColorEnum;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a lowercase string red, white or pink.")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            match value.to_lowercase().as_ref() {
                "red" => Ok(WineColorEnum::Red),
                "white" => Ok(WineColorEnum::White),
                "pink" => Ok(WineColorEnum::Pink),
                _ => Err(de::Error::custom(format!("invalid wine color: {}", value)))
            }
        }

    }

    impl<'de> Deserialize<'de> for WineColorEnum {
        fn deserialize<D>(deserializer: D) -> Result<WineColorEnum, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_str(WineColorVisitor)
        }
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
        }).resource("/crawl/", |r| {
            r.method(http::Method::POST).with(crawl_saq_controller);
        }).resource("/users/", |r| {
            r.method(http::Method::POST).with(register);
        })
        }).bind("127.0.0.1:8080")
        .unwrap()
        .run();
}
