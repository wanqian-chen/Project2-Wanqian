use cpython::{Python, PyDict, PyResult, ObjectProtocol, ToPyObject};
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

// using python package: nba_api
// from nba_api.stats.endpoints import playercareerstats
// python: career = playercareerstats.PlayerCareerStats(player_id='203999').get_json()
#[get("/career/{player_id}")]
async fn career(player_id: web::Path<String>) -> impl Responder {
    let gil = Python::acquire_gil();
    let py = gil.python();
    // convert player_id to python string
    let player_id = player_id.into_inner().to_py_object(py);
    let nba_api = py.import("nba_api.stats.endpoints").unwrap();
    let playercareerstats = nba_api.get(py, "playercareerstats").unwrap();
    let playercareerstats = playercareerstats.getattr(py, "PlayerCareerStats").unwrap();
    let playercareerstats = playercareerstats.call(py, (player_id,), None).unwrap();
    let career = playercareerstats.getattr(py, "get_json").unwrap();
    let career = career.extract::<String>(py).unwrap();
    HttpResponse::Ok().body(career)
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(career)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}