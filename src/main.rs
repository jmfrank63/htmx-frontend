use actix_files::Files;
use actix_web::http::header;
use actix_web::{post, middleware::Logger, App, HttpResponse, HttpServer, Responder};

#[post("/_ai")]
async fn demo() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body("Content loaded from Actix!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(demo)
            .service(Files::new("/", "./static").index_file("index.html"))
            .wrap(
                actix_web::middleware::DefaultHeaders::new()
                    .add((header::ACCESS_CONTROL_ALLOW_ORIGIN, "*"))
                    .add((header::ACCESS_CONTROL_ALLOW_METHODS, "GET")),
            )
    })
    .bind("localhost:5500")?
    .run()
    .await
}
