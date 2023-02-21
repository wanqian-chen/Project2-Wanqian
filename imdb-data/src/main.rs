use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;

// Import the package.json file using the `include_str!` macro
const PACKAGE_JSON: &'static str = include_str!("../../package.json");

// Define a struct to hold the package.json data
#[derive(Serialize)]
struct PackageJson {
    status: String,
    name: String,
    description: String,
    version: String,
    repository: String,
    author: String,
    license: String,
    postman: String,
    postman_collection_json: String,
}

// Define the handler for the root route
#[get("/")]
async fn index() -> impl Responder {
    // Deserialize the package.json data into a struct
    let package: PackageJson = serde_json::from_str(PACKAGE_JSON).unwrap();
    // return the data as a JSON response
    HttpResponse::Ok().json(package)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
