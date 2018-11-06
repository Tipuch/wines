use bigdecimal::BigDecimal;
use diesel;
use diesel::query_dsl::RunQueryDsl;
use diesel::prelude::PgConnection;
use std::error::Error;
use std::env;
use schema::{saq_wines, wine_recommendations, users};
use argon2rs::argon2i_simple;
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
    pub price: BigDecimal,
    pub alcohol_percent: BigDecimal,
    pub color: WineColorEnum,
    pub grape_varieties: Vec<String>,
    pub available_online: bool
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
    pub price: &'a BigDecimal,
    pub alcohol_percent: &'a BigDecimal,
    pub color: &'a WineColorEnum,
    pub grape_varieties: &'a Vec<String>,
    pub available_online: &'a bool
}

pub fn create_saq_wine<'a>(conn: &PgConnection, name: &'a str, country: &'a str, region: &'a str, 
designation_of_origin: &'a str, regulated_designation: &'a bool, producer: &'a str,
volume: &'a BigDecimal, price: &'a BigDecimal, alcohol_percent: &'a BigDecimal,
color: &'a WineColorEnum, grape_varieties: &'a Vec<String>, available_online: &'a bool) -> SaqWine {

    let new_saq_wine = NewSaqWine {
        name: name,
        country: country,
        region: region,
        designation_of_origin: designation_of_origin,
        regulated_designation: regulated_designation,
        producer: producer,
        volume: volume,
        price: price,
        alcohol_percent: alcohol_percent,
        color: color,
        grape_varieties: grape_varieties,
        available_online: available_online
    };

    diesel::insert_into(saq_wines::table)
        .values(&new_saq_wine)
        .get_result(conn)
        .expect("Error saving new SAQ Wine.")
}

#[derive(Queryable)]
pub struct WineRecommendation {
    pub id: i32,
    pub country: String,
    pub region: String,
    pub designation_of_origin: String,
    pub producer: String,
    pub rating: i32,
    pub color: WineColorEnum,
    pub grape_variety: String
}

#[derive(Insertable, Deserialize)]
#[table_name="wine_recommendations"]
pub struct NewWineRecommendation {
    pub country: String,
    pub region: String,
    pub designation_of_origin: String,
    pub producer: String,
    pub rating: i32,
    pub color: WineColorEnum,
    pub grape_variety: String
}

pub fn create_wine_recommendations<'a>(conn: &PgConnection, new_wine_recommendations: &'a Vec<NewWineRecommendation>) -> Vec<WineRecommendation> {

    diesel::insert_into(wine_recommendations::table)
        .values(new_wine_recommendations)
        .get_results(conn)
        .expect("Error saving new wine recommendation.")
}

pub fn parse_wine_color(string: &str) -> Result<WineColorEnum, Box<Error>> {
    match string {
        "red" => Ok(WineColorEnum::Red),
        "white" => Ok(WineColorEnum::White),
        "pink" => Ok(WineColorEnum::Pink),
        _ => Err("Unrecognized enum variant".into()),
    }
}

#[derive(Queryable)]
pub struct User {
    pub email: String,
    pub admin: bool,
    pub salt: Vec<u8>,
    pub password: Vec<u8>
}

#[derive(Insertable, Deserialize)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub email: &'a str,
    pub admin: &'a bool,
    pub salt: &'a Vec<u8>,
    pub password: &'a Vec<u8>
}

pub fn compute_salt(email: &String) -> Vec<u8> {
    let secret_key = env::var("SECRET_KEY").expect("SECRET_KEY must be set");
    argon2i_simple(email, &secret_key).to_vec()
}

pub fn hash_password(password: &String, salt: &Vec<u8>) -> Vec<u8> {
    argon2i_simple(password, &String::from_utf8(salt).unwrap()).to_vec()
}

pub fn create_user<'a>(conn: &PgConnection, email: &'a str, admin: &'a bool,
salt: &'a Vec<u8>, password: &'a Vec<u8>) -> User{
    let user = NewUser {
        email: email,
        admin: admin,
        salt: salt,
        password: password
    };
    diesel::insert_into(users::table)
        .values(&user)
        .get_result(conn)
        .expect("Error saving new User.")
}
