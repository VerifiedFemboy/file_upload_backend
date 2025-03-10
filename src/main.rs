use std::path::Path;
use actix_cors::Cors;
use actix_web::{App, HttpServer};

use crate::upload_file::{list_files, serve_file};

mod upload_file;
mod manage;

const URL_HOST : &str= "127.0.0.1:8080";

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    //Creates an upload path which files will be saved
    if !Path::new("./upload").exists() {
        tokio::fs::create_dir("./upload").await?
    }

    //Runs the server with CORS enabled
    HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .service(upload_file::upload_post)
            .service(list_files)
            .service(serve_file)
            .service(manage::manage_file)
    })
    .bind(URL_HOST)?.run().await
}


