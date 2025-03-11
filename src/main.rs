use std::path::Path;
use actix_cors::Cors;
use actix_web::{App, HttpServer};
use file_requests::{delete_file, upload_post};

use crate::file_requests::{list_files, serve_file};

mod file_requests;

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
            .service(upload_post)
            .service(delete_file)
            .service(list_files)
            .service(serve_file)
    })
    .bind(URL_HOST)?.run().await
}


