use reqwest;
use reqwest::Result as ReqwestResult;
use establish_connection;
use models::{create_saq_wine, parse_wine_color};
use std::str::FromStr;
use bigdecimal::BigDecimal;
use select::document::Document;
use select::node::Node;
use select::predicate::{Predicate, Attr, Class, Name};


fn get_document(url: &String) -> ReqwestResult<String> {
    let client = reqwest::Client::builder()
    .timeout(None)
    .build()?;
    let result = client.get(url).send()?.text()?;
    Ok(result)
}

pub fn crawl_saq(origin_url: &String) {
    let mut document = Document::from(&*get_document(origin_url).unwrap());
    let mut next_page = get_next_page(&document);
    while next_page.is_some() {
        for node in document.find(Attr("id", "resultatRecherche").descendant(Class("nom").descendant(Name("a")))) {
            crawl_saq_wine(node.attr("href").unwrap());
        }
        document = Document::from(&*get_document(&next_page.unwrap()).unwrap());
        next_page = get_next_page(&document);
    }
}

fn get_next_page(document: &Document) -> Option<String> {
    let next_page = document.find(Attr("id", "page-suivante-haut")).next();
    // there may not be a next page
    if next_page.is_none() {
        return None;
    }
    let onclick_attr = String::from(next_page.unwrap().parent().unwrap().attr("onclick").unwrap());
    // parse url out of onclick parameter
    let url_begin_index = onclick_attr.find("http").unwrap();
    let url_end_index = onclick_attr.rfind("'").unwrap();
    Some(onclick_attr[url_begin_index..url_end_index].to_string())
}

fn crawl_saq_wine(detail_page_url: &str) {
    let connection = establish_connection();
    let document = Document::from(&*get_document(&String::from(detail_page_url)).unwrap());

    let name = document.find(Class("product-description-title")).next().unwrap().text();
    let price = parse_price(&document);
    
    let default_parsing_func = |node: &Node| {Some(String::from(node.text().trim()))};
    // fetch info from detailed info in the web page.
    let country = parse_wine_info(&document, "Country", Box::new(default_parsing_func)).unwrap();
    let mut region = String::from("");
    let region_option = parse_wine_info(&document, "Region", Box::new(default_parsing_func));
    if region_option.is_some() {
        region = parse_wine_info(&document, "Region", Box::new(default_parsing_func)).unwrap();
    }
    let mut designation_of_origin = String::from("");
    let designation_of_origin_option = parse_wine_info(&document, "Designation of origin", Box::new(default_parsing_func));
    let regulated_designation_option = parse_wine_info(&document, "Regulated designation", Box::new(default_parsing_func));
    let regulated_designation = designation_of_origin_option.is_some() && regulated_designation_option.is_some() 
        && regulated_designation_option.unwrap() != "Table wine";
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
            &volume_text[..volume_text.find("L").unwrap()].trim()
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
        &producer, &BigDecimal::from_str(&volume).unwrap(), &BigDecimal::from_str(&price).unwrap(),
        &alcohol_percent, &parse_wine_color(&wine_color.to_lowercase()).unwrap(), &parse_grape_varieties(&document)
    );
    println!("SAQ Wine: {} was added", name);
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

fn parse_price(document: &Document) -> String {
    let price_info = document.find(Class("price")).next().unwrap().text().replace(",", "");
    let little_star = price_info.rfind("*");
    if little_star.is_some() {
        return price_info[(price_info.find("$").unwrap() + 1)..little_star.unwrap()].to_string();
    } else {
        return price_info[(price_info.find("$").unwrap() + 1)..].to_string();
    }
}

fn parse_grape_varieties(document: &Document) -> Vec<String> {
    let mut result = vec![];
    for node in document.find(Attr("id", "details").descendant(Name("li"))) {
        let left = node.find(Class("left").descendant(Name("span"))).next();
        let right = node.find(Class("right")).next();
        if left.is_some() && right.is_some() {
            if left.unwrap().text() == "Grape variety(ies)" {
                for col in right.unwrap().find(Class("col1")) {
                    result.push(col.text());
                }
                return result;
            }
        }
    }
    result
}
