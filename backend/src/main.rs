use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn search() -> impl Responder {
    HttpResponse::Ok().body("Hello, this is the search endpoint!")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/search", web::get().to(search))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}