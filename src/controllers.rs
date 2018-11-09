use csv::ReaderBuilder;
use actix_web::{
    dev, error, http, multipart, Error, FutureResponse, ResponseError,
    HttpMessage, HttpRequest, HttpResponse, Json, FromRequest
};
use actix_web::http::header::AUTHORIZATION;
use actix_web::middleware::identity::RequestIdentity;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use models::{
    NewWineRecommendation, create_wine_recommendations,
    compute_salt, hash_password, create_user, User
};
use errors::{LoginError};
use establish_connection;
use crawler::crawl_saq;
use dotenv::dotenv;
use std::{thread, env};
use futures::future;
use futures::{Future, Stream};

#[derive(Deserialize)]
pub struct UserForm {
    email: String,
    admin: bool,
    password: String
}

#[derive(Deserialize)]
pub struct LoginForm {
    email: String,
    password: String
}

fn save_records(
    field: multipart::Field<dev::Payload>
) -> Box<Future<Item = Vec<NewWineRecommendation>, Error = error::Error>> {
    Box::new(
        field
            .fold(vec![], |mut acc, bytes| -> Box<Future<Item = Vec<u8>, Error = error::MultipartError>> {
                for byte in bytes {
                    acc.push(byte);
                }
                Box::new(future::ok(acc))
            })
            .and_then(|result| {
                let mut rdr = ReaderBuilder::new()
                    .delimiter(b';')
                    .from_reader(&result[..]);
                let wine_recommendations: Vec<NewWineRecommendation> = rdr.deserialize().map(|r|{
                    r.unwrap()
                }).collect();
                future::ok(wine_recommendations)
            }).map_err(|_| {
                error::ErrorInternalServerError("This didn't work")
            })
    )
}

fn handle_multipart_item(
    item: multipart::MultipartItem<dev::Payload>
) -> Box<Stream<Item = Vec<NewWineRecommendation>, Error = Error>> {
    match item {
        multipart::MultipartItem::Field(field) => {
            Box::new(save_records(field).into_stream())
        }
        multipart::MultipartItem::Nested(mp) => Box::new(
            mp.map_err(error::ErrorInternalServerError)
                .map(handle_multipart_item)
                .flatten(),
        ),
    }
}

pub fn upload(req: HttpRequest) -> FutureResponse<HttpResponse> {
    use schema::users::dsl::*;
    let identity = req.identity();
    let mut user = None;
    let conn = establish_connection();
    if identity.is_some() {
        user = Some(users
            .filter(email.eq(identity.unwrap()))
            .first::<User>(&conn).unwrap());
    }
    
    Box::new(
        req.multipart()
            .map_err(error::ErrorInternalServerError)
            .map(handle_multipart_item)
            .flatten()
            .map(move |mut wine_recommendations| {
                if user.is_some() {
                    for wine_reco in wine_recommendations.iter_mut() {
                        wine_reco.user_id = Some(user.clone().unwrap().id);
                    }
                }
                create_wine_recommendations(&conn, &wine_recommendations);
            })
            .collect()
            .map(|_| HttpResponse::Ok().finish())
            .map_err(|e| {
                println!("failed: {}", e);
                e
            }),
    )
}

pub fn crawl_saq_controller(_req: HttpRequest) -> Result<HttpResponse, error::Error> {
    thread::spawn(move || {
        let origin_url = String::from("https://www.saq.com/webapp/wcs/stores/servlet/SearchDisplay?pageSize=20&searchTerm=*&catalogId=50000&orderBy=1&facet=adi_f9%3A%221%22%7Cadi_f9%3A%221%22&categoryIdentifier=06&beginIndex=0&langId=-1&showOnly=product&categoryId=39919&storeId=20002&metaData=");
        crawl_saq(&origin_url);
    });
    Ok(HttpResponse::Ok().body("Crawl has been started"))
}

pub fn index(_req: HttpRequest) -> Result<HttpResponse, error::Error> {
    let html = r#"<html>
        <head><title>Upload Test</title></head>
        <body>
            <form target="/" method="post" enctype="multipart/form-data">
                <input type="file" name="file"/>
                <input type="submit" value="Submit"></button>
            </form>
        </body>
    </html>"#;

    Ok(HttpResponse::Ok().body(html))
}

pub fn register(req: HttpRequest) -> FutureResponse<HttpResponse> {
    let headers = req.headers();
    dotenv().ok();
    let secret_key = env::var("SECRET_KEY").expect("SECRET_KEY must be set");
    if !headers.contains_key(AUTHORIZATION) || headers[AUTHORIZATION] != secret_key {
        return Box::new(future::ok(HttpResponse::new(http::StatusCode::FORBIDDEN)));
    }
    Box::new(Json::<UserForm>::extract(&req).then(|user_form_result| {
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
            &password
        );
        future::ok(HttpResponse::Ok().body(format!(
            "User with email {} has been created successfully!", user.email
        )))
    }))
}

pub fn login(req: HttpRequest) -> FutureResponse<HttpResponse> {
    use schema::users::dsl::*;
    Box::new(Json::<LoginForm>::extract(&req).then(move |login_form_result| {
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
            // congrats you're in :)
            req.remember(user.email);
            return future::ok(HttpResponse::Ok().finish());
        }
        future::ok(LoginError::ValidationError.error_response())
    }))
}

pub fn logout(req: HttpRequest) -> Result<HttpResponse, error::Error> {
    req.forget();
    Ok(HttpResponse::Ok().finish())
}
