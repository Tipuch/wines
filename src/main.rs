#![allow(proc_macro_derive_resolution_fallback)]
extern crate actix_web;
extern crate select;
extern crate reqwest;
extern crate dotenv;
#[macro_use] extern crate diesel;
extern crate bigdecimal;
mod crawler;
mod schema;
mod models;
use crawler::crawl_saq;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;


// fn index(_req: &HttpRequest) -> &'static str {
//     "Hello world!"
// }

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
    // server::new(|| App::new().resource("/", |r| r.f(index)))
    //     .bind("127.0.0.1:8080")
    //     .unwrap()
    //     .run();

    let origin_url = format!("https://www.saq.com/webapp/wcs/stores/servlet/SearchDisplay?pageSize={PAGE_SIZE}&searchTerm=*&catalogId=50000&orderBy=1&facet=adi_f9%3A%221%22|adi_f9%3A%221%22&categoryIdentifier=06&beginIndex=0&langId=-1&showOnly=product&categoryId=39919&storeId=20002&metaData=", PAGE_SIZE=crawler::PAGE_SIZE);
    // let connection = establish_connection();
    // create_saq_wine(
    //     &connection, "test", "test", "test", "test", &true, "test", &BigDecimal::from_str(&"22.2").unwrap(),
    //     &22, &WineColorEnum::Red, &vec![]
    // );
    crawl_saq(&origin_url);

    println!("Success !");
}
