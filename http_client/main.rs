use actix_web::{get, App, HttpServer, HttpResponse, Responder};
use reqwest;

#[get("/v1/retrieve")]
async fn get_external_data() -> impl Responder {
    let url = "https://jsonplaceholder.typicode.com/users";

    match reqwest::get(url).await {
        Ok(response) => match response.text().await {
            Ok(body) => HttpResponse::Ok()
                .content_type("application/json")
                .body(body),
            Err(err) => {
                eprintln!("❌ Error reading response: {}", err);
                HttpResponse::InternalServerError().body("Failed process data")
            }
        },
        Err(err) => {
            eprintln!("❌ Error fetching data: {}", err);
            HttpResponse::BadGateway().body("Failed retrieve data")
        }
    }
}

#[get("/ping")]
fn ping() -> impl Responder {
    HttpResponse::Ok().body("pong")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let host = "127.0.0.1";
    let port = 8080;

    println!("🚀 Server running at http://{}:{}", host, port);

    HttpServer::new(|| {
        App::new()
            .service(get_external_data)
            .service(ping)
    })
    .bind((host, port))?
    .run()
    .await
}