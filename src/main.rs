use actix_web::{App, HttpResponse, HttpServer, Responder, get, web};
use tera::{Context, Tera};
use serde::Serialize;

#[derive(Serialize)]
struct Post {
    title: String,
    link: String,
    author: String,
}

#[get("/")]
async fn index(tera: web::Data<Tera>) -> impl Responder {
    let mut data = Context::new();
    let posts = [
        Post {
            title: String::from("This is the first link"),
            link: String::from("https://example.com"),
            author: String::from("Bob")
        },
        Post {
            title: String::from("The Second Link"),
            link: String::from("https://example.com"),
            author: String::from("Alice")
        },
    ];

    data.insert("title", "Hacker Clone");
    data.insert("posts", &posts);

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
