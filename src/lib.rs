use actix_web::{web, App, HttpRequest, HttpServer, Responder, HttpResponse};
use actix_web::dev::Server;
use std::net::TcpListener;


async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Helllllo {}!", &name)
}
 
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}


#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String
}


async fn subscribe(form: web::Form<FormData>) -> HttpResponse {
    format!("Welcom {}!",form.email);
    format!("Wecom {}!",form.name);
    HttpResponse::Ok().finish()
}






pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    println!("starting server......");
    let server = HttpServer::new( || {
        App::new()
        .route("/", web::get().to(greet))
        .route("/health_check", web::get().to(health_check))
        .route("/subscriptions",web::post().to(subscribe))
    })
    .listen(listener)?
    .run();
    Ok(server)


}