use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use reqwest::{Client, Response};
use select::document::Document;
use std::io::Cursor;

// import functions from lib.rs
use imdb_data::{parse_reviews, parse_info};

// A hello world handler
#[get("/")]
async fn index() -> impl Responder {
    println!("Hello!");
    HttpResponse::Ok().body("Hello! This is an application to provide data from IMDB.")
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
        "<html>
            <head>
                <title>IMDB Info</title>
            </head>
            <body>
                <h1><center>Movie/TV Show Info</center></h1>
                <p>Title: {}</p>
                <p>Rate: {} / 10.0</p>
                <p>Cast: </p>
                <ul>
                    {}
                </ul>
                <p>Origin: {}</p>
                <p>Language: {}</p>
            </body>
        </html>",
        info["title"], info["rate"], info["cast"].as_array().unwrap().iter().map(|cast| format!("<li>{}</li>", cast)).collect::<Vec<String>>().join(""), info["origin"], info["language"]
    ))
}

// A handler to reviews
#[get("/reviews/{id}")]
async fn reviews(id: web::Path<String>) -> impl Responder {
    let url = format!("https://www.imdb.com/title/{}/reviews", id);
    // fetch the document of the url
    let client = Client::new();
    let res: Response = client.get(&url).send().await.unwrap();
    let body = res.bytes().await.unwrap();
    let dom = Document::from_read(Cursor::new(body)).unwrap();

    // parse the document
    let reviews = parse_reviews(&dom);

    // return the result
    HttpResponse::Ok().body(format!(
        "<html>
            <head>
                <title>IMDB Reviews</title>
            </head>
            <body>
                <h1>IMDB Reviews</h1>
                {}
            </body>
        </html>",
        reviews
            .iter()
            .map(|review| format!(
                "<div>
                    <h3><center>{}</center></h3>
                    <p>Rate: {} / 10.0</p>
                    <p>{}</p>
                    <p><i>User: {}</i></p>
                    <p><i>Date: {}</i></p>
                </div>",
                review["title"], review["rate"], review["content"], review["author"], review["date"]
            ))
            .collect::<Vec<String>>()
            .join("<br>")
    ))
    
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index).service(title).service(reviews))
        .bind("0.0.0.0:8081")?
        .run()
        .await
}
