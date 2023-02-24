use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use reqwest::{Client, Response};
use select::document::Document;
use std::io::Cursor;

// import functions from lib.rs
use imdb_data::{parse_info, parse_reviews, search_result};

// A hello world handler
#[get("/")]
async fn index() -> impl Responder {
    println!("Hello!");
    HttpResponse::Ok().body("Hello! This is an application to provide data from IMDB.")
}

// A handler to get info by name
#[get("/search/{name}")]
async fn search(name: web::Path<String>) -> impl Responder {
    let url = format!("https://www.imdb.com/find/?q={}", name);
    // fetch the document of the url
    let client = Client::new();
    let res: Response = client.get(&url).send().await.unwrap();
    let body = res.bytes().await.unwrap();
    let dom = Document::from_read(Cursor::new(body)).unwrap();

    // parse the document
    let info = search_result(&dom);

    // return the result
    HttpResponse::Ok().body(format!(
        "<html>
            <head>
                <title>IMDB Search</title>
            </head>
            <body>
                <h1><center>Search Result</center></h1>
                {}
            </body>
        </html>",
        info.iter()
            .map(|item| format!(
                "
                    <h3>Title: {}</h3>
                    <p><i>Time: {}</i></p>
                    <button onclick=\"window.location.href='/title/{}'\">More Info</button>
                    <button onclick=\"window.location.href='/reviews/{}'\">Reviews</button>
                ",
                item["title"], item["time"], item["id"], item["id"]
            ))
            .collect::<Vec<String>>()
            .join("")
    ))
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
                <p><b>Title:</b> {}</p>
                <p><b>Rate:</b> {} / 10.0</p>
                <p><b>Top 5 Cast:</b></p>
                <ul>
                    {}
                </ul>
                <p><b>Origin:</b> {}</p>
                <p><b>Language:</b> {}</p>
            </body>
        </html>",
        info["title"],
        info["rate"],
        info["cast"]
            .as_array()
            .unwrap()
            .iter()
            .map(|cast| format!("<li>{}</li>", cast))
            .collect::<Vec<String>>()
            .join(""),
        info["origin"],
        info["language"]
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
                <h1><center>IMDB Reviews</center></h1>
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
