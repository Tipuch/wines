use reqwest::*;
use establish_connection;
use types::WineColorEnum;
use models::create_saq_wine;
use std::str::FromStr;
use bigdecimal::BigDecimal;
use select::document::Document;
use select::node::Node;
use select::predicate::{Predicate, Attr, Class, Name};

pub const PAGE_SIZE: u8 = 20;

pub fn get_document(url: &String) -> Result<String> {
    let result = get(url)?.text()?;
    Ok(result)
}

pub fn get_next_page(document: &Document) -> Option<String> {
    let next_page = document.find(Attr("id", "page-suivante-haut")).next().unwrap().parent();
    // there may not be a next page
    if next_page.is_none() {
        return None;
    }
    let onclick_attr = String::from(next_page.unwrap().attr("onclick").unwrap());
    // parse url out of onclick parameter
    let url_begin_index = onclick_attr.find("http").unwrap();
    let url_end_index = onclick_attr.rfind("'").unwrap();
    Some(onclick_attr[url_begin_index..url_end_index].to_string())
}

pub fn crawl_saq_wine(detail_page_url: &String) {
    let connection = establish_connection();
    let document = Document::from(&*get_document(detail_page_url).ok().unwrap());

    let name = document.find(Class("product-description-title")).next().unwrap().text();
    let default_parsing_func = |node: &Node| {Some(node.text())};
    // fetch info from detailed info in the web page.
    let country = parse_wine_info(&document, "Country", Box::new(default_parsing_func)).unwrap();
    let region = parse_wine_info(&document, "Region", Box::new(default_parsing_func)).unwrap();
    create_saq_wine(
        &connection, &name, &country, "test", "test", &true, "test", &BigDecimal::from_str(&"22.2").unwrap(),
        &22, &WineColorEnum::Red, &vec![]
    );
}

fn parse_wine_info(document: &Document, info_selector: &str,
    handling_func: Box<(Fn(&Node) -> Option<String>)>) -> Option<String> {
    for node in document.find(Attr("id", "details").descendant(Name("li"))) {
        let left = node.find(Class("left").descendant(Name("span"))).next();
        let right = node.find(Class("right")).next();

        if left.is_some() && right.is_some() {
            if left.unwrap().text() == info_selector {
                return handling_func(&right.unwrap());
            }
        }
    }
    None
}
