use std::cell::Cell;
use std::fs;
use std::io::Write;

use csv::ReaderBuilder;
use actix_web::{
    dev, error, multipart, Error, FutureResponse,
    HttpMessage, HttpRequest, HttpResponse,
};

use futures::future;
use futures::{Future, Stream};

#[derive(Debug, Deserialize)]
struct WineRecommendationRecord {
    country: String,
    region: String,
    designation_of_origin: String,
    producer: String,
    rating: i32,
    color: String,
    grape_variety: String
}

pub fn save_file(
    field: multipart::Field<dev::Payload>,
) -> Box<Future<Item = i64, Error = error::Error>> {
    let file_path_string = "upload.png";
    let mut file = match fs::File::create(file_path_string) {
        Ok(file) => file,
        Err(e) => return Box::new(future::err(error::ErrorInternalServerError(e))),
    };
    Box::new(
        field
            .fold(vec![], |mut acc, bytes| -> Box<Future<Item = Vec<u8>, Error = error::MultipartError>> {
                for byte in bytes {
                    acc.push(byte);
                }
                Box::new(future::ok(acc))
            }).map_err(|e| {
                error::ErrorInternalServerError("This didn't work")
            })
            .and_then(|result| {
                // println!("result: {}", result);
                let mut rdr = ReaderBuilder::new()
                    .delimiter(b';')
                    .from_reader(&result[..]);
                for r in rdr.records() {
                    let record = r.unwrap();
                    println!("{:?}", record);
                }
                future::ok(200)
            })
    )
}

pub fn handle_multipart_item(
    item: multipart::MultipartItem<dev::Payload>,
) -> Box<Stream<Item = i64, Error = Error>> {
    match item {
        multipart::MultipartItem::Field(field) => {
            Box::new(save_file(field).into_stream())
        }
        multipart::MultipartItem::Nested(mp) => Box::new(
            mp.map_err(error::ErrorInternalServerError)
                .map(handle_multipart_item)
                .flatten(),
        ),
    }
}

pub fn upload(req: HttpRequest) -> FutureResponse<HttpResponse> {
    Box::new(
        req.multipart()
            .map_err(error::ErrorInternalServerError)
            .map(handle_multipart_item)
            .flatten()
            .collect()
            .map(|sizes| HttpResponse::Ok().json(sizes))
            .map_err(|e| {
                println!("failed: {}", e);
                e
            }),
    )
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