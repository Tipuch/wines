use bigdecimal::BigDecimal;
use diesel;
use diesel::query_dsl::RunQueryDsl;
use diesel::prelude::PgConnection;
use schema::saq_wines;
use types::WineColorEnum;

#[derive(Queryable)]
pub struct SaqWine {
    pub id: i32,
    pub name: String,
    pub country: String,
    pub region: String,
    pub designation_of_origin: String,
    pub regulated_designation: bool,
    pub producer: String,
    // in liters
    pub volume: BigDecimal,
    pub alcohol_percent: i32,
    pub color: WineColorEnum,
    pub grape_varieties: Vec<String>
}

#[derive(Insertable)]
#[table_name="saq_wines"]
pub struct NewSaqWine<'a> {
    pub name: &'a str,
    pub country: &'a str,
    pub region: &'a str,
    pub designation_of_origin: &'a str,
    pub regulated_designation: &'a bool,
    pub producer: &'a str,
    pub volume: &'a BigDecimal,
    pub alcohol_percent: &'a i32,
    pub color: &'a WineColorEnum,
    pub grape_varieties: &'a Vec<String>
}

pub fn create_saq_wine<'a>(conn: &PgConnection, name: &'a str, country: &'a str, region: &'a str, 
designation_of_origin: &'a str, regulated_designation: &'a bool, producer: &'a str, volume: &'a BigDecimal, alcohol_percent: &'a i32,
color: &'a WineColorEnum, grape_varieties: &'a Vec<String>) -> SaqWine {

    let new_saq_wine = NewSaqWine {
        name: name,
        country: country,
        region: region,
        designation_of_origin: designation_of_origin,
        regulated_designation: regulated_designation,
        producer: producer,
        volume: volume,
        alcohol_percent: alcohol_percent,
        color: color,
        grape_varieties: grape_varieties
    };

    diesel::insert_into(saq_wines::table)
        .values(&new_saq_wine)
        .get_result(conn)
        .expect("Error saving new SAQ Wine.")
}