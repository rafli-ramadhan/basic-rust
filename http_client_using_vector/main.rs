use actix_web::{web, App, HttpServer, HttpResponse, Responder, get, post};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Serialize, Deserialize, Clone)]
struct User {
    id: u32,
    name: String,
    email: String,
}

struct AppState {
    users: Mutex<Vec<User>>,
}

#[post("/users")]
async fn create_user(data: web::Data<AppState>, new_user: web::Json<User>) -> impl Responder {
    let mut users = data.users.lock().unwrap();
    users.push(new_user.into_inner());
    HttpResponse::Ok().body("User added")
}

#[get("/users")]
async fn get_users(data: web::Data<AppState>) -> impl Responder {
    let users = data.users.lock().unwrap();
    HttpResponse::Ok().json(&*users)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        users: Mutex::new(Vec::new()),
    });

    println!("🚀 Server running on http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(create_user)
            .service(get_users)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
