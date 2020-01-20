use crate::schema::{saq_wines, users, wine_recommendations};
use crate::types::WineColorEnum;
use argon2rs::{argon2i_simple, defaults, Argon2, Variant};
use bigdecimal::BigDecimal;
use diesel;
use diesel::prelude::PgConnection;
use diesel::query_dsl::RunQueryDsl;
use std::env;
use std::error::Error;

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
    pub available_online: bool,
}

#[derive(Insertable)]
#[table_name = "saq_wines"]
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
    pub available_online: &'a bool,
}

pub fn create_saq_wine<'a>(
    conn: &PgConnection,
    name: &'a str,
    country: &'a str,
    region: &'a str,
    designation_of_origin: &'a str,
    regulated_designation: &'a bool,
    producer: &'a str,
    volume: &'a BigDecimal,
    price: &'a BigDecimal,
    alcohol_percent: &'a BigDecimal,
    color: &'a WineColorEnum,
    grape_varieties: &'a Vec<String>,
    available_online: &'a bool,
) -> SaqWine {
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
        available_online: available_online,
    };

    diesel::insert_into(saq_wines::table)
        .values(&new_saq_wine)
        .get_result(conn)
        .expect("Error saving new SAQ Wine.")
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug, Serialize, Deserialize)]
#[belongs_to(User)]
pub struct WineRecommendation {
    pub id: i32,
    pub country: String,
    pub region: String,
    pub designation_of_origin: String,
    pub producer: String,
    pub rating: i32,
    pub color: WineColorEnum,
    pub grape_variety: String,
    pub user_id: Option<i32>,
    pub name: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "wine_recommendations"]
pub struct NewWineRecommendation {
    pub country: String,
    pub region: String,
    pub designation_of_origin: String,
    pub producer: String,
    pub rating: i32,
    pub color: WineColorEnum,
    pub grape_variety: String,
    pub user_id: Option<i32>,
    pub wine_name: String,
}

pub fn create_wine_recommendation<'a>(
    conn: &PgConnection,
    new_wine_recommendation: &'a NewWineRecommendation,
) -> WineRecommendation {
    diesel::insert_into(wine_recommendations::table)
        .values(new_wine_recommendation)
        .get_result(conn)
        .expect("Error saving new wine recommendation.")
}

pub fn parse_wine_color(string: &str) -> Result<WineColorEnum, Box<dyn Error>> {
    match string {
        "red" => Ok(WineColorEnum::Red),
        "white" => Ok(WineColorEnum::White),
        "pink" => Ok(WineColorEnum::Pink),
        _ => Err("Unrecognized enum variant".into()),
    }
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug, Clone)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub admin: bool,
    pub salt: Vec<u8>,
    pub password: Vec<u8>,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub email: &'a str,
    pub admin: &'a bool,
    pub salt: &'a Vec<u8>,
    pub password: &'a Vec<u8>,
}

pub fn hash_password(password: &String, salt: Vec<u8>) -> Vec<u8> {
    let mut out = [0; defaults::LENGTH];
    let a2 = Argon2::default(Variant::Argon2i);
    a2.hash(&mut out, password.as_bytes(), &salt, &[], &[]);
    out.to_vec()
}

pub fn compute_salt(email: &String) -> Vec<u8> {
    let secret_key = env::var("SECRET_KEY").expect("SECRET_KEY must be set");
    argon2i_simple(email, &secret_key).to_vec()
}

pub fn create_user<'a>(
    conn: &PgConnection,
    email: &'a str,
    admin: &'a bool,
    salt: &'a Vec<u8>,
    password: &'a Vec<u8>,
) -> User {
    let user = NewUser {
        email: email,
        admin: admin,
        salt: salt,
        password: password,
    };
    diesel::insert_into(users::table)
        .values(&user)
        .get_result(conn)
        .expect("Error saving new User.")
}
