use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use reqwest::{Client, Response};
use select::document::Document;
use std::io::Cursor;

// import functions from lib.rs
use imdb_data::parse_info;

// A hello world handler
#[get("/")]
async fn index() -> impl Responder {
    println!("Hello world!");
    HttpResponse::Ok().body("Hello world!")
}

// A handler to get basic info of movie or tv show by id
#[get("/title/{id}")]
async fn title(id: web::Path<String>) -> impl Responder {
    let url = format!("https://www.imdb.com/title/{}/", id);
    // fetch the document of the url
    let client = Client::new();
    let res: Response = client.get(&url).send().await.unwrap();
    let body = res.bytes().await.unwrap();
    let dom = Document::from_read(Cursor::new(body)).unwrap();

    // parse the document
    let info = parse_info(&dom);

    // return the result
    HttpResponse::Ok().body(format!(
        "Title: {}, \nRate: {}, \nCast: {}",
        info["title"],
        info["rate"],
        format_args!(
            "Name: {}, Role: {}, \nName: {}, Role: {}",
            info["cast"][0]["name"],
            info["cast"][0]["role"],
            info["cast"][1]["name"],
            info["cast"][1]["role"]
        )
    ))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index).service(title))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
