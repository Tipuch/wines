use crate::establish_connection;
use crate::models::{create_saq_wine, parse_wine_color};
use crate::schema::saq_wines;
use bigdecimal::{BigDecimal, ToPrimitive};
use diesel;
use diesel::RunQueryDsl;
use reqwest;
use select::document::Document;
use select::node::Node;
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
            .find(Attr("id", "resultatRecherche").descendant(Class("nom").descendant(Name("a"))))
        {
            crawl_saq_wine(node.attr("href").unwrap()).await;
        }
        document = Document::from(&*get_document(&next_page.unwrap()).await.unwrap());
        next_page = get_next_page(&document);
    }
    println!("Success ! :)");
}

fn get_next_page(document: &Document) -> Option<String> {
    let next_page = document.find(Attr("id", "page-suivante-haut")).next();
    // there may not be a next page
    if next_page.is_none() {
        return None;
    }
    let onclick_attr = String::from(
        next_page
            .unwrap()
            .parent()
            .unwrap()
            .attr("onclick")
            .unwrap(),
    );
    // parse url out of onclick parameter
    let url_begin_index = onclick_attr.find("http").unwrap();
    let url_end_index = onclick_attr.rfind("'").unwrap();
    Some(onclick_attr[url_begin_index..url_end_index].to_string())
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
        .find(Class("product-description-title"))
        .next()
        .unwrap()
        .text();
    let price = parse_price(&document);

    let default_parsing_func = |node: &Node| Some(node.text().trim().to_string());
    // fetch info from detailed info in the web page.
    let country = parse_wine_info(&document, "Country", Box::new(default_parsing_func)).unwrap();
    let mut region = String::from("");
    let region_option = parse_wine_info(&document, "Region", Box::new(default_parsing_func));
    if region_option.is_some() {
        region = parse_wine_info(&document, "Region", Box::new(default_parsing_func)).unwrap();
    }
    let available_online = document
        .find(Attr("alt", "This product is not available online"))
        .next()
        .is_none();
    let mut designation_of_origin = String::from("");
    let designation_of_origin_option = parse_wine_info(
        &document,
        "Designation of origin",
        Box::new(default_parsing_func),
    );
    let regulated_designation_option = parse_wine_info(
        &document,
        "Regulated designation",
        Box::new(default_parsing_func),
    );
    let regulated_designation = designation_of_origin_option.is_some()
        && regulated_designation_option.is_some()
        && regulated_designation_option.unwrap() != "Table wine";
    if regulated_designation {
        designation_of_origin = designation_of_origin_option.unwrap();
    }
    let producer = parse_wine_info(
        &document,
        "Producer",
        Box::new(|node: &Node| {
            let text = node.text();
            if text.contains("All products from this producer") {
                return Some(
                    text[..text.find("All products from this producer").unwrap()]
                        .trim()
                        .to_string(),
                );
            }
            Some(text.trim().to_string())
        }),
    )
    .unwrap();

    let volume = parse_wine_info(
        &document,
        "Size",
        Box::new(|node: &Node| {
            let volume_text = node.text();
            if volume_text.contains("ml") {
                return Some(
                    volume_text[..volume_text.find("ml").unwrap()]
                        .trim()
                        .to_string(),
                );
            }
            let volume_liters =
                BigDecimal::from_str(&volume_text[..volume_text.find("L").unwrap()].trim())
                    .unwrap();

            Some(
                (volume_liters * BigDecimal::from_str("1000").unwrap())
                    .to_u32()
                    .unwrap()
                    .to_string(),
            )
        }),
    )
    .unwrap();

    let alcohol_percent_option = parse_wine_info(
        &document,
        "Degree of alcohol",
        Box::new(|node: &Node| {
            let alcohol_percent_text = node.text();
            let alcohol_percent = BigDecimal::from_str(
                alcohol_percent_text[..alcohol_percent_text.find("%").unwrap()].trim(),
            )
            .unwrap();
            Some(alcohol_percent.to_string())
        }),
    );
    let alcohol_percent: BigDecimal;
    if alcohol_percent_option.is_some() {
        alcohol_percent = alcohol_percent_option.unwrap().parse().unwrap();
    } else {
        alcohol_percent = BigDecimal::from(0);
    }

    let wine_color = parse_wine_info(&document, "colour", Box::new(default_parsing_func)).unwrap();

    create_saq_wine(
        &connection,
        &name,
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
    println!("SAQ Wine: {} was added", name);
}

fn parse_wine_info(
    document: &Document,
    info_selector: &str,
    handling_func: Box<dyn (Fn(&Node) -> Option<String>)>,
) -> Option<String> {
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
    let price_info = document
        .find(Class("price"))
        .next()
        .unwrap()
        .text()
        .replace(",", "");
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
