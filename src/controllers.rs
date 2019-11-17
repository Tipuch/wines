use actix_files::NamedFile;
use actix_identity::Identity;
use actix_web::http::header::ContentType;
use actix_web::http::header::AUTHORIZATION;
use actix_web::{error, http, web, Error, FromRequest, HttpRequest, HttpResponse, ResponseError};
use bigdecimal::BigDecimal;
use crawler::crawl_saq;
use diesel::pg::expression::dsl::any;
use diesel::OptionalExtension;
use diesel::{
    BoolExpressionMethods, ExpressionMethods, JoinOnDsl, PgTextExpressionMethods, QueryDsl,
    RunQueryDsl, TextExpressionMethods,
};
use errors::LoginError;
use establish_connection;
use futures::{future, Future};
use models::{
    compute_salt, create_user, create_wine_recommendation, hash_password, NewWineRecommendation,
    User, WineRecommendation,
};
use schema::{saq_wines as saq, users, wine_recommendations as recos};
use std::io::Read;
use std::str::FromStr;
use std::{env, thread};
use types::WineColorEnum;
use utils::is_dup_wine;
#[derive(Deserialize)]
pub struct UserForm {
    email: String,
    admin: bool,
    password: String,
}

#[derive(Deserialize)]
pub struct LoginForm {
    email: String,
    password: String,
}

#[derive(Deserialize, AsChangeset)]
#[table_name = "recos"]
pub struct WineRecommendationForm {
    pub country: String,
    pub region: String,
    pub designation_of_origin: String,
    pub producer: String,
    pub rating: i32,
    pub color: WineColorEnum,
    pub grape_variety: String,
}

#[derive(Deserialize, Clone)]
pub struct WineCriteria {
    min_rating: Option<i32>,
    max_price: Option<String>,
    color: Option<WineColorEnum>,
    available_online: Option<bool>,
}

pub fn crawl_saq_controller(req: HttpRequest) -> Result<HttpResponse, error::Error> {
    let headers = req.headers();
    let secret_key = env::var("SECRET_KEY").expect("SECRET_KEY must be set");
    if !headers.contains_key(AUTHORIZATION)
        || headers.get(AUTHORIZATION).unwrap().to_str().unwrap() != secret_key
    {
        return Ok(HttpResponse::new(http::StatusCode::FORBIDDEN));
    }
    thread::spawn(move || {
        let origin_url = String::from("https://www.saq.com/webapp/wcs/stores/servlet/SearchDisplay?pageSize=20&searchTerm=*&catalogId=50000&orderBy=1&facet=adi_f9%3A%221%22%7Cadi_f9%3A%221%22&categoryIdentifier=06&beginIndex=0&langId=-1&showOnly=product&categoryId=39919&storeId=20002&metaData=");
        crawl_saq(&origin_url);
    });
    Ok(HttpResponse::Ok().body("Crawl has been started"))
}

pub fn index(_req: HttpRequest) -> Result<HttpResponse, error::Error> {
    let mut buffer = Vec::new();
    NamedFile::open("./static/html/index.html")?
        .file()
        .read_to_end(&mut buffer)?;
    Ok(HttpResponse::Ok().set(ContentType::html()).body(buffer))
}

pub fn register(req: HttpRequest) -> Box<dyn Future<Item = HttpResponse, Error = Error>> {
    let headers = req.headers();
    let secret_key = env::var("SECRET_KEY").expect("SECRET_KEY must be set");
    if !headers.contains_key(AUTHORIZATION)
        || headers.get(AUTHORIZATION).unwrap().to_str().unwrap() != secret_key
    {
        return Box::new(future::ok(HttpResponse::new(http::StatusCode::FORBIDDEN)));
    }
    Box::new(
        web::Json::<UserForm>::extract(&req).then(|user_form_result| {
            if user_form_result.is_err() {
                return future::ok(HttpResponse::new(http::StatusCode::BAD_REQUEST));
            }
            let user_form = user_form_result.unwrap();
            let connection = establish_connection();
            let salt = compute_salt(&user_form.email);
            let password = hash_password(&user_form.password, salt.clone());
            let user = create_user(
                &connection,
                &user_form.email,
                &user_form.admin,
                &salt,
                &password,
            );
            future::ok(HttpResponse::Ok().body(format!(
                "User with email {} has been created successfully!",
                user.email
            )))
        }),
    )
}

pub fn login(req: HttpRequest) -> Box<dyn Future<Item = HttpResponse, Error = Error>> {
    use schema::users::dsl::*;
    Box::new(
        web::Json::<LoginForm>::extract(&req).then(move |login_form_result| {
            if login_form_result.is_err() {
                return future::ok(HttpResponse::new(http::StatusCode::BAD_REQUEST));
            }
            let login_form = login_form_result.unwrap();
            let conn = establish_connection();
            let user_result = users
                .filter(email.eq(login_form.email.clone()))
                .first::<User>(&conn);
            if user_result.is_err() {
                return future::ok(HttpResponse::new(http::StatusCode::BAD_REQUEST));
            }
            let user = user_result.unwrap();
            if hash_password(&login_form.password, user.salt) == user.password {
                let identity = Identity::extract(&req).unwrap();
                // congrats you're in :)
                identity.remember(user.email);
                return future::ok(HttpResponse::Ok().finish());
            }
            future::ok(LoginError::ValidationError.error_response())
        }),
    )
}

pub fn logout(identity: Identity) -> Result<HttpResponse, error::Error> {
    identity.forget();
    Ok(HttpResponse::Ok().finish())
}

pub fn create_wine_reco(req: HttpRequest) -> Box<dyn Future<Item = HttpResponse, Error = Error>> {
    use schema::users::dsl::*;

    Box::new(
        web::Json::<NewWineRecommendation>::extract(&req).then(move |wine_reco_result| {
            let identity = Identity::extract(&req).unwrap();
            let conn = establish_connection();
            if identity.identity().is_some() {
                let user = users
                    .filter(email.eq(identity.identity().unwrap()))
                    .first::<User>(&conn)
                    .unwrap();
                if wine_reco_result.is_err() {
                    return future::ok(HttpResponse::new(http::StatusCode::BAD_REQUEST));
                }
                let mut new_wine_recommendation = wine_reco_result.unwrap();
                new_wine_recommendation.user_id = Some(user.id);
                create_wine_recommendation(&conn, &new_wine_recommendation);
                return future::ok(HttpResponse::new(http::StatusCode::CREATED));
            } else {
                return future::ok(HttpResponse::new(http::StatusCode::UNAUTHORIZED));
            }
        }),
    )
}

pub fn get_wine_reco(req: HttpRequest) -> Result<HttpResponse, error::Error> {
    use schema::users::dsl::*;
    let identity = Identity::extract(&req).unwrap();
    let conn = establish_connection();
    if identity.identity().is_some() {
        let user = users
            .filter(email.eq(identity.identity().unwrap()))
            .first::<User>(&conn)
            .unwrap();
        let query = recos::table
            .filter(recos::user_id.eq(user.id))
            .load::<WineRecommendation>(&conn)
            .expect("Error fetching wine recommendations.");
        return Ok(HttpResponse::Ok().json(query));
    } else {
        return Ok(HttpResponse::new(http::StatusCode::UNAUTHORIZED));
    }
}

pub fn update_wine_reco(req: HttpRequest) -> Box<dyn Future<Item = HttpResponse, Error = Error>> {
    use schema::users::dsl::*;
    use schema::wine_recommendations;

    Box::new(
        web::Json::<WineRecommendationForm>::extract(&req).then(move |wine_reco_result| {
            let identity = Identity::extract(&req).unwrap();
            let conn = establish_connection();
            let parsed_wine_reco_id = req
                .match_info()
                .get("wine_recommendation_id")
                .unwrap()
                .parse::<i32>();
            if parsed_wine_reco_id.is_err() {
                return future::ok(HttpResponse::new(http::StatusCode::NOT_FOUND));
            }
            let wine_recommendation_id = parsed_wine_reco_id.unwrap();
            if identity.identity().is_some() {
                let user = users
                    .filter(email.eq(identity.identity().unwrap()))
                    .first::<User>(&conn)
                    .unwrap();
                if wine_reco_result.is_err() {
                    return future::ok(HttpResponse::new(http::StatusCode::BAD_REQUEST));
                }
                let target = wine_recommendations::table.filter(
                    wine_recommendations::dsl::user_id
                        .eq(user.id)
                        .and(wine_recommendations::dsl::id.eq(wine_recommendation_id)),
                );
                let wine_recommendation_form: WineRecommendationForm =
                    wine_reco_result.unwrap().into_inner();
                let update_result = diesel::update(target)
                    .set(&wine_recommendation_form)
                    .get_result::<WineRecommendation>(&conn)
                    .optional();
                if update_result.is_err() {
                    return future::ok(HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR));
                }
                return future::ok(HttpResponse::new(http::StatusCode::OK));
            } else {
                return future::ok(HttpResponse::new(http::StatusCode::UNAUTHORIZED));
            }
        }),
    )
}

pub fn delete_wine_reco(req: HttpRequest) -> Result<HttpResponse, error::Error> {
    use schema::users::dsl::*;
    use schema::wine_recommendations;

    let identity = Identity::extract(&req).unwrap();
    let conn = establish_connection();
    let parsed_wine_reco_id = req
        .match_info()
        .get("wine_recommendation_id")
        .unwrap()
        .parse::<i32>();
    if parsed_wine_reco_id.is_err() {
        return Ok(HttpResponse::new(http::StatusCode::NOT_FOUND));
    }
    if identity.identity().is_some() {
        let user = users
            .filter(email.eq(identity.identity().unwrap()))
            .first::<User>(&conn)
            .unwrap();
        let target = wine_recommendations::table.filter(
            wine_recommendations::dsl::user_id
                .eq(user.id)
                .and(wine_recommendations::dsl::id.eq(parsed_wine_reco_id.unwrap())),
        );
        diesel::delete(target)
            .execute(&conn)
            .expect("Error deleting wine recommendation");
        return Ok(HttpResponse::new(http::StatusCode::OK));
    } else {
        return Ok(HttpResponse::new(http::StatusCode::UNAUTHORIZED));
    }
}

pub fn get_wines(req: HttpRequest) -> Result<HttpResponse, error::Error> {
    let wine_criteria_result = web::Query::<WineCriteria>::extract(&req);
    if wine_criteria_result.is_err() {
        return Ok(HttpResponse::new(http::StatusCode::BAD_REQUEST));
    }
    let wine_criteria = wine_criteria_result.unwrap();
    let identity = Identity::extract(&req).unwrap();
    let mut user = None;
    let conn = establish_connection();
    if identity.identity().is_some() {
        user = Some(
            users::table
                .filter(users::email.eq(identity.identity().unwrap()))
                .first::<User>(&conn)
                .unwrap(),
        );
    }
    let mut wines_query = saq::table
        .inner_join(
            recos::table.on(recos::country
                .eq("")
                .or(saq::country.ilike(recos::country))
                .and(
                    recos::region
                        .eq("")
                        .or(saq::region.ilike(recos::region))
                        .and(
                            recos::designation_of_origin
                                .eq("")
                                .or(saq::designation_of_origin
                                    .ilike(recos::designation_of_origin.concat("%")))
                                .and(
                                    recos::wine_name
                                        .eq("")
                                        .or(saq::name.ilike(recos::wine_name.concat("%")))
                                        .and(
                                            recos::producer
                                                .eq("")
                                                .or(saq::producer.ilike(recos::producer))
                                                .and(
                                                    recos::grape_variety
                                                        .eq("")
                                                        .or(recos::grape_variety
                                                            .ilike(any(saq::grape_varieties)))
                                                        .and(saq::color.eq(recos::color)),
                                                ),
                                        ),
                                ),
                        ),
                )),
        )
        .select((
            saq::id,
            saq::name,
            saq::available_online,
            saq::country,
            saq::region,
            saq::designation_of_origin,
            saq::producer,
            saq::color,
            saq::volume,
            saq::price,
            recos::rating,
        ))
        .order(saq::price / saq::volume)
        .into_boxed();
    if wine_criteria.color.is_some() {
        wines_query = wines_query.filter(saq::color.eq(wine_criteria.clone().color.unwrap()));
    }
    if wine_criteria.min_rating.is_some() {
        wines_query = wines_query.filter(recos::rating.ge(wine_criteria.min_rating.unwrap()));
    }
    if wine_criteria.max_price.is_some() {
        let max_price = BigDecimal::from_str(&wine_criteria.clone().max_price.unwrap());
        if max_price.is_err() {
            return Ok(HttpResponse::new(http::StatusCode::BAD_REQUEST));
        }
        // one bottle is considered to be 750 mL
        let max_bottle_price = max_price.unwrap() / BigDecimal::from_str("750").unwrap();
        wines_query = wines_query.filter((saq::price / saq::volume).le(max_bottle_price));
    }
    if wine_criteria.available_online.is_some() {
        wines_query =
            wines_query.filter(saq::available_online.eq(wine_criteria.available_online.unwrap()));
    }
    if user.is_some() {
        wines_query = wines_query.filter(recos::user_id.eq(user.unwrap().id));
    }

    let mut wines: Vec<(
        i32,
        String,
        bool,
        String,
        String,
        String,
        String,
        WineColorEnum,
        BigDecimal,
        BigDecimal,
        i32,
    )> = wines_query.load(&conn).unwrap();

    /* in some cases it's possible we have duplicates in our results
    (same bottle returned by different recommendations)
    we want to remove these duplicates and keep the recommendation
     with the bigger rating */

    let mut i = 0;
    while i != wines.len() {
        if is_dup_wine(&wines, &wines[i], i) {
            wines.remove(i);
        } else {
            i += 1;
        }
    }

    let results: Vec<(
        &i32,
        &String,
        &bool,
        &String,
        &String,
        &String,
        &String,
        &WineColorEnum,
        String,
        String,
        &i32,
    )> = wines
        .iter()
        .map(|wine| {
            (
                &wine.0,
                &wine.1,
                &wine.2,
                &wine.3,
                &wine.4,
                &wine.5,
                &wine.6,
                &wine.7,
                format!("{} ml", &wine.8),
                format!("${}", &wine.9),
                &wine.10,
            )
        })
        .collect();

    Ok(HttpResponse::Ok().json(json!({ "results": results })))
}

pub fn get_health(_req: HttpRequest) -> Result<HttpResponse, error::Error> {
    Ok(HttpResponse::Ok().finish())
}
