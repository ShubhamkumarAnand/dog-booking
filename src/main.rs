use actix_web::{get, HttpServer, App, HttpResponse, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello Dog booking Service!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello))
    .bind(("localhost", 5001))?
    .run()
    .await
}
