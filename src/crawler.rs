use crate::establish_connection;
use crate::models::{create_saq_wine, parse_wine_color};
use crate::schema::saq_wines;
use bigdecimal::{BigDecimal, ToPrimitive};
use diesel;
use diesel::RunQueryDsl;
use reqwest;
use regex::Regex;
use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};
use std::str::FromStr;

async fn get_document(url: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::builder().build()?;
    let result = client.get(url).send().await?.text().await?;
    Ok(result)
}

pub async fn crawl_saq(origin_url: &String) {
    let connection = establish_connection();
    // delete all previous saq_wines present
    diesel::delete(saq_wines::table)
        .execute(&connection)
        .expect("Error deleting saq_wines");
    let mut document = Document::from(&*get_document(origin_url).await.unwrap());
    let mut next_page = get_next_page(&document);
    while next_page.is_some() {
        for node in document
            .find(Name("div").and(Class("product-item-info")).descendant(Name("a").and(Class("product-item-photo"))))
        {
            crawl_saq_wine(node.attr("href").unwrap()).await;
        }
        document = Document::from(&*get_document(&next_page.unwrap()).await.unwrap());
        next_page = get_next_page(&document);
    }
    println!("Success ! :)");
}

fn get_next_page(document: &Document) -> Option<String> {
    let next_page = document.find(Name("a").and(Class("next"))).next();
    // there may not be a next page
    if next_page.is_none() {
        return None;
    }
    Some(String::from(next_page.unwrap().attr("href").unwrap()))
}

async fn crawl_saq_wine(detail_page_url: &str) {
    let connection = establish_connection();
    let response = get_document(detail_page_url);
    let result = response.await;
    if result.is_err() {
        println!("There was an error fetching wine {}", detail_page_url);
        return;
    }
    let document = Document::from(&*result.unwrap());

    let name = document
        .find(Name("h1").and(Class("page-title")))
        .next()
        .unwrap()
        .text();
    let price = parse_price(&document);

    // fetch info from detailed info in the web page.
    let country = parse_wine_info(&document, "Country").unwrap();

    let mut region = String::from("");
    let region_option = parse_wine_info(&document, "Region");
    if region_option.is_some() {
        region = parse_wine_info(&document, "Region").unwrap();
    }

    let available_online = document
        .find(Class("out-of-stock-online"))
        .next()
        .is_none();
    
    let mut designation_of_origin = String::from("");
    let designation_of_origin_option = parse_wine_info(
        &document,
        "Designation of origin"
    );

    let regulated_designation_option = parse_wine_info(
        &document,
        "Regulated Designation"
    );
    let regulated_designation = designation_of_origin_option.is_some()
        && regulated_designation_option.is_some()
        && regulated_designation_option.unwrap() != "Table wine";
    if regulated_designation {
        designation_of_origin = designation_of_origin_option.unwrap();
    }

    let producer = parse_wine_info(
        &document,
        "Producer"
    )
    .unwrap();

    let volume_node = document.find(Attr("data-th", "Size")).next().unwrap();
    let volume_text = volume_node.text();
    let volume;
    if volume_text.contains("ml") {
        volume = volume_text[..volume_text.find("ml").unwrap()]
            .trim()
            .to_string();
    } else {
        let volume_liters =
        BigDecimal::from_str(&volume_text[..volume_text.find("L").unwrap()].trim())
            .unwrap();
        volume = (volume_liters * BigDecimal::from_str("1000").unwrap())
            .to_u32()
            .unwrap()
            .to_string();
    }

    let alcohol_percent_option = document.find(Attr("data-th", "Degree of alcohol")).next();
    let alcohol_percent: BigDecimal;
    if alcohol_percent_option.is_some() {
        let alcohol_percent_text = alcohol_percent_option.unwrap().text();
        alcohol_percent = BigDecimal::from_str(
            alcohol_percent_text[..alcohol_percent_text.find("%").unwrap()].trim(),
        ).unwrap();
    } else {
        alcohol_percent = BigDecimal::from(0);
    }

    let wine_color = parse_wine_info(&document, "Color").unwrap();

    create_saq_wine(
        &connection,
        &name.trim(),
        &country,
        &region,
        &designation_of_origin,
        &regulated_designation,
        &producer,
        &BigDecimal::from_str(&volume).unwrap(),
        &BigDecimal::from_str(&price).unwrap(),
        &alcohol_percent,
        &parse_wine_color(&wine_color.to_lowercase()).unwrap(),
        &parse_grape_varieties(&document),
        &available_online,
    );
    println!("SAQ Wine: {} was added", name.trim());
}

fn parse_wine_info(
    document: &Document,
    info_selector: &str
) -> Option<String> {
    let info_node = document.find(Attr("data-th", info_selector)).next();
    if info_node.is_some() {
        return Some(String::from(info_node.unwrap().text().trim()));
    }
    None
}

fn parse_price(document: &Document) -> String {
    String::from(document
        .find(Attr("data-price-type", "finalPrice"))
        .next().unwrap().attr("data-price-amount").unwrap())
}

fn parse_grape_varieties(document: &Document) -> Vec<String> {
    let info_node = document.find(Attr("data-th", "Grape variety")).next();
    if info_node.is_none() {
        return vec![];
    }
    let info_node_text = info_node.unwrap().text();
    let re = Regex::new(r"\s[0-9]+\s%").unwrap();
    let grape_varieties_text = re.replace_all(&info_node_text, "");
    let grape_varieties = grape_varieties_text.split(", ").collect::<Vec<&str>>();
    grape_varieties.iter().map(|grape_variety| grape_variety.trim().to_string()).collect()
}
