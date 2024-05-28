use std::io::Result;
use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};

#[get("/hello")]
async fn hello () -> impl Responder{
    HttpResponse::Ok().body("hello world")
}
#[post("/form")]
async fn form(req_body:String) -> impl Responder{
    println!("{}",req_body);
    HttpResponse::Ok().body(req_body)
}
#[actix_web::main]
async fn main() -> Result<()> {
    HttpServer::new(||{
        App::new()
        .service(hello).service(form)
    })
    .bind(("127.0.0.1",8080))?
    .run()
    .await
}
