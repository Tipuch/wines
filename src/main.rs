extern crate actix_web;
extern crate select;
extern crate reqwest;
use select::document::Document;
mod crawler;

// fn index(_req: &HttpRequest) -> &'static str {
//     "Hello world!"
// }

fn main() {
    // server::new(|| App::new().resource("/", |r| r.f(index)))
    //     .bind("127.0.0.1:8080")
    //     .unwrap()
    //     .run();

    let origin_url = format!("https://www.saq.com/webapp/wcs/stores/servlet/SearchDisplay?pageSize={PAGE_SIZE}&searchTerm=*&catalogId=50000&orderBy=1&facet=adi_f9%3A%221%22|adi_f9%3A%221%22&categoryIdentifier=06&beginIndex=0&langId=-1&showOnly=product&categoryId=39919&storeId=20002&metaData=", PAGE_SIZE=crawler::PAGE_SIZE);
    let _document = Document::from(&*crawler::get_document(&origin_url).ok().unwrap());
    println!("Success !");
}
