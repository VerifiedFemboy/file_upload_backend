use std::path::Path;
use actix_web::{App, HttpServer};

use crate::upload_file::{list_files, serve_file};

mod upload_file;
mod manage;

#[actix_web::main]
async fn main() -> std::io::Result<()> {


    //Creates an upload path which files will be saved
    if !Path::new("./upload").exists() {
        tokio::fs::create_dir("./upload").await?
    }

    //Runs the server
    HttpServer::new(move || App::new()
        .service(upload_file::upload_post)
        .service(list_files)
        .service(serve_file)
        .service(manage::manage_file))
        .bind("127.0.0.1:8080")?.run().await
}


