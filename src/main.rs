#![allow(proc_macro_derive_resolution_fallback)]
extern crate actix_web;
extern crate select;
extern crate reqwest;
#[macro_use] extern crate diesel;
extern crate bigdecimal;
extern crate dotenv;
mod crawler;
pub mod schema;
pub mod models;
use select::document::Document;

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


fn main() {
    // server::new(|| App::new().resource("/", |r| r.f(index)))
    //     .bind("127.0.0.1:8080")
    //     .unwrap()
    //     .run();

    let origin_url = format!("https://www.saq.com/webapp/wcs/stores/servlet/SearchDisplay?pageSize={PAGE_SIZE}&searchTerm=*&catalogId=50000&orderBy=1&facet=adi_f9%3A%221%22|adi_f9%3A%221%22&categoryIdentifier=06&beginIndex=0&langId=-1&showOnly=product&categoryId=39919&storeId=20002&metaData=", PAGE_SIZE=crawler::PAGE_SIZE);
    let _document = Document::from(&*crawler::get_document(&origin_url).ok().unwrap());
    


    println!("Success !");
}
