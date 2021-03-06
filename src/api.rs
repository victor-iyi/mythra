use crate::engines::freemp3cloud;
use crate::engines::mp3red;
use crate::types::MusicRequest;

use actix_web::{http::StatusCode, web, App, HttpResponse, HttpServer};
//use actix_cors::Cors;
use actix_web::middleware::Logger;
use log::{debug, error};

async fn search(web::Query(info): web::Query<MusicRequest>) -> HttpResponse {
    debug!(
        "Request for client with engine={} and query={}!",
        info.engine, info.query
    );
    let query = info.query.clone();
    let engine = info.engine.clone();
    let engine_match = engine.as_str();
    match engine_match {
        "mp3red" => {
            let e = mp3red::MP3Red {};
            let res = e.search(query).await.ok();
            HttpResponse::Ok().json(res.unwrap())
        }
        "freemp3cloud" => {
            let e = freemp3cloud::FreeMP3Cloud {};
            let res = e.search(query).await.ok();
            HttpResponse::Ok().json(res.unwrap())
        }
        _ => {
            error!("Engine {} is unsupported", engine_match);
            HttpResponse::new(StatusCode::NOT_FOUND)
        }
    }
}

pub async fn api(port: &str) -> std::io::Result<()> {
    let address: &str = &(format!("0.0.0.0:{}", port))[..];
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(web::resource("/search").route(web::get().to(search)))
    })
    .bind(address)
    .unwrap()
    .run()
    .await
}
