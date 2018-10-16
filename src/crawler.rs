use reqwest::get;
use reqwest::Result as ReqwestResult;
use establish_connection;
use types::WineColorEnum;
use models::create_saq_wine;
use std::str::FromStr;
use std::error::Error;
use bigdecimal::BigDecimal;
use select::document::Document;
use select::node::Node;
use select::predicate::{Predicate, Attr, Class, Name};

pub const PAGE_SIZE: u8 = 20;

pub fn get_document(url: &String) -> ReqwestResult<String> {
    let result = get(url)?.text()?;
    Ok(result)
}

pub fn crawl_saq(origin_url: &String) {
    let mut document = Document::from(&*get_document(origin_url).ok().unwrap());
    let mut next_page = get_next_page(&document);
    while next_page.is_some() {
        for node in document.find(Attr("id", "resultatRecherche").descendant(Name("a"))) {
            if node.attr("name").unwrap() == "firstItemAnchor" {
                continue;
            }
            crawl_saq_wine(node.attr("href").unwrap());
        }
        document = Document::from(&*get_document(&next_page.unwrap()).ok().unwrap());
        next_page = get_next_page(&document);
    }
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

pub fn crawl_saq_wine(detail_page_url: &str) {
    let connection = establish_connection();
    let document = Document::from(&*get_document(&String::from(detail_page_url)).ok().unwrap());

    let name = document.find(Class("product-description-title")).next().unwrap().text();
    let default_parsing_func = |node: &Node| {Some(String::from(node.text().trim()))};
    // fetch info from detailed info in the web page.
    let country = parse_wine_info(&document, "Country", Box::new(default_parsing_func)).unwrap();
    let region = parse_wine_info(&document, "Region", Box::new(default_parsing_func)).unwrap();
    let mut designation_of_origin = String::from("");
    let designation_of_origin_option = parse_wine_info(&document, "Designation of origin", Box::new(default_parsing_func));
    let regulated_designation_label = parse_wine_info(&document, "Regulated designation", Box::new(default_parsing_func)).unwrap();
    let regulated_designation = designation_of_origin_option.is_some() && regulated_designation_label != "Table wine";
    if regulated_designation {
        designation_of_origin = designation_of_origin_option.unwrap();
    }
    let producer = parse_wine_info(&document, "Producer", Box::new(default_parsing_func)).unwrap();

    let volume = parse_wine_info(&document, "Size", Box::new(|node: &Node| {
        let volume_text = node.text();
        if volume_text.contains("ml") {
            return Some(volume_text[..volume_text.find("ml").unwrap()].trim().to_string());
        }
        let volume_liters = BigDecimal::from_str(
            &volume_text[..volume_text.find("ml").unwrap()].trim()
            ).unwrap();
        
        Some((volume_liters * BigDecimal::from_str("1000").unwrap()).to_string())
    })).unwrap();

    let alcohol_percent: BigDecimal = parse_wine_info(&document, "Degree of alcohol", Box::new(|node: &Node| {
        let alcohol_percent_text = node.text();
        let alcohol_percent = BigDecimal::from_str(alcohol_percent_text[..alcohol_percent_text.find("%").unwrap()]
            .trim()).unwrap();
        Some(alcohol_percent.to_string())
    })).unwrap().parse().unwrap();

    let wine_color = parse_wine_info(&document, "colour", Box::new(default_parsing_func)).unwrap();

    create_saq_wine(
        &connection, &name, &country, &region, &designation_of_origin, &regulated_designation,
        &producer, &BigDecimal::from_str(&volume).unwrap(),
        &alcohol_percent, &parse_wine_color(&wine_color.to_lowercase()).unwrap(), &vec![]
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

fn parse_wine_color(string: &str) -> Result<WineColorEnum, Box<Error>> {
    match string {
        "red" => Ok(WineColorEnum::Red),
        "white" => Ok(WineColorEnum::White),
        "pink" => Ok(WineColorEnum::Pink),
        _ => Err("Unrecognized enum variant".into()),
    }
}
