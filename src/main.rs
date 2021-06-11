use actix_web::{App, HttpResponse, HttpServer, Responder, get, web};
use tera::{Context, Tera};

#[get("/")]
async fn index(tera: web::Data<Tera>) -> impl Responder {
    let mut data = Context::new();
    data.insert("title", "Hacker Clone");
    data.insert("name", "hooneun");

    let rendered = tera.render("index.html", &data).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .data(Tera::new("templates/**/*").unwrap())
            .service(index)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
