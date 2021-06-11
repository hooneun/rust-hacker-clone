use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web};
use tera::{Context, Tera};
use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize)]
struct Submission {
    title: String,
    link: String,
}

#[derive(Debug, Deserialize)]
struct LoginUser {
    username: String,
    password: String,
}

#[derive(Debug, Deserialize)]
struct User {
    username: String,
    email: String,
    password: String,
}

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

#[get("/signup")]
async fn signup(tera: web::Data<Tera>) -> impl Responder {
    let mut data = Context::new();
    data.insert("title", "Sign Up");

    let rendered = tera.render("signup.html", &data).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[post("/signup")]
async fn process_signup(data: web::Form<User>) -> impl Responder {
    println!("{:?}", data);
    HttpResponse::Ok().body(format!("Successfully saved user: {}", data.username))
}

#[get("/login")]
async fn login(tera: web::Data<Tera>) -> impl Responder {
    let mut data = Context::new();
    data.insert("title", "Login");

    let rendered = tera.render("login.html", &data).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[post("/login")]
async fn process_login(data: web::Form<LoginUser>) -> impl Responder {
    println!("{:?}", data);
    HttpResponse::Ok().body(format!("Logged in: {}", data.username))
}

#[get("/submission")]
async fn submission(tera: web::Data<Tera>) -> impl Responder {
    let mut data = Context::new();
    data.insert("title", "Submit a Post");

    let rendered = tera.render("submission.html", &data).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[post("/submission")]
async fn process_submission(data: web::Form<Submission>) -> impl Responder {
    println!("{:?}", data);
    HttpResponse::Ok().body(format!("Posted submission: {}", data.title))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .data(Tera::new("templates/**/*").unwrap())
            .service(index)
            .service(signup)
            .service(login)
            .service(process_login)
            .service(submission)
            .service(process_submission)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
