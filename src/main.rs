#[macro_use]
extern crate diesel;
pub mod models;
pub mod schema;

use actix_identity::{CookieIdentityPolicy, Identity, IdentityService};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use tera::{Context, Tera};

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use models::{LoginUser, NewPost, NewUser, Post, User};

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

#[derive(Debug, Deserialize)]
struct Submission {
    title: String,
    link: String,
}

#[derive(Deserialize)]
struct PostForm {
    title: String,
    link: String,
}

#[get("/")]
async fn index(tera: web::Data<Tera>) -> impl Responder {
    use schema::posts::dsl::posts;
    use schema::users::dsl::users;

    let connection = establish_connection();
    let all_posts: Vec<(Post, User)> = posts
        .inner_join(users)
        .load(&connection)
        .expect("Error retrieving all posts.");

    let mut data = Context::new();
    data.insert("title", "Hacker Clone");
    data.insert("posts_users", &all_posts);

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
async fn process_signup(data: web::Form<NewUser>) -> impl Responder {
    use schema::users;

    let connection = establish_connection();

    diesel::insert_into(users::table)
        .values(&*data)
        .get_result::<User>(&connection)
        .expect("Error registering user.");

    HttpResponse::Ok().body(format!("Successfully saved user: {}", data.username))
}

#[get("/login")]
async fn login(tera: web::Data<Tera>, id: Identity) -> impl Responder {
    let mut data = Context::new();
    data.insert("title", "Login");

    if let Some(_id) = id.identity() {
        return HttpResponse::Ok().body("Already logged in.");
    }
    let rendered = tera.render("login.html", &data).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[post("/login")]
async fn process_login(data: web::Form<LoginUser>, id: Identity) -> impl Responder {
    use schema::users::dsl::{username, users};

    let connection = establish_connection();
    let user = users
        .filter(username.eq(&data.username))
        .first::<User>(&connection);

    match user {
        Ok(u) => {
            if u.password == data.password {
                println!("{:?}", data);
                let session_token = String::from(u.username);
                id.remember(session_token);
                HttpResponse::Ok().body(format!("Logged in: {}", data.username))
            } else {
                HttpResponse::Ok().body("Password is incorrect.")
            }
        }
        Err(e) => {
            println!("{:?}", e);
            HttpResponse::Ok().body("User doesn't exist.")
        }
    }
}

#[post("/logout")]
async fn logout(id: Identity) -> impl Responder {
    id.forget();
    HttpResponse::Ok().body("Logged out.")
}

#[get("/submission")]
async fn submission(tera: web::Data<Tera>, id: Identity) -> impl Responder {
    let mut data = Context::new();
    data.insert("title", "Submit a Post");

    if let Some(_id) = id.identity() {
        let rendered = tera.render("submission.html", &data).unwrap();
        return HttpResponse::Ok().body(rendered);
    }

    HttpResponse::Ok().body("user not logged in.")
}

#[post("/submission")]
async fn process_submission(data: web::Form<PostForm>, id: Identity) -> impl Responder {
    if let Some(id) = id.identity() {
        use schema::users::dsl::{username, users};

        let connection = establish_connection();
        let user: Result<User, diesel::result::Error> =
            users.filter(username.eq(id)).first(&connection);

        match user {
            Ok(u) => {
                let new_post = NewPost::from_post_form(data.title.clone(), data.link.clone(), u.id);

                use schema::posts;

                diesel::insert_into(posts::table)
                    .values(&new_post)
                    .get_result::<Post>(&connection)
                    .expect("Error saving post.");

                return HttpResponse::Ok().body("Submitted.");
            }
            Err(e) => {
                println!("{:?}", e);
                return HttpResponse::Ok().body("Failed to find user.");
            }
        }
    }
    HttpResponse::Unauthorized().body("User not logged in.")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&[0; 32])
                    .name("auth-cookie")
                    .secure(false),
            ))
            .data(Tera::new("templates/**/*").unwrap())
            .service(index)
            .service(signup)
            .service(process_signup)
            .service(login)
            .service(process_login)
            .service(submission)
            .service(process_submission)
            .service(logout)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
