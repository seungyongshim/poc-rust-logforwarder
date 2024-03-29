use actix_web::{App, get, HttpResponse, HttpServer, post, Responder, web};
use actix::prelude::*;

mod MyActor;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    actix_rt::spawn(async {
        let addr = MyActor::MyActor::new().start();
        addr.send(MyActor::Ping(10)).await.unwrap();
    });

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
