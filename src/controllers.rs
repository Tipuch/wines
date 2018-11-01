use csv::ReaderBuilder;
use actix_web::{
    dev, error, multipart, Error, FutureResponse,
    HttpMessage, HttpRequest, HttpResponse,
};
use models::{NewWineRecommendation, create_wine_recommendations};
use establish_connection;
use futures::future;
use futures::{Future, Stream};

pub fn save_records(
    field: multipart::Field<dev::Payload>,
) -> Box<Future<Item = bool, Error = error::Error>> {
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
                let connection = establish_connection();
                create_wine_recommendations(&connection, &wine_recommendations);
                future::ok(true)
            }).map_err(|_| {
                error::ErrorInternalServerError("This didn't work")
            })
    )
}

pub fn handle_multipart_item(
    item: multipart::MultipartItem<dev::Payload>,
) -> Box<Stream<Item = bool, Error = Error>> {
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