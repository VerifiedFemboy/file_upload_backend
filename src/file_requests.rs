use std::fs::{self as fs, remove_file};
use std::fs::read_dir;
use std::io::Write;
use std::path::Path;

use actix_files::NamedFile;
use actix_multipart::Multipart;
use actix_web::{delete, get, post, web, HttpResponse};
use futures_util::{StreamExt, TryStreamExt};
use serde::Serialize;

use crate::URL_HOST;

const DIRECTORY: &str = "./upload";

#[post("/upload")]
pub async fn upload_post(mut payload: Multipart) -> HttpResponse {
    while let Ok(Some(mut field)) = payload.try_next().await {
        //get data from payload
        let content = field.content_disposition();

        let file_name = content.get_filename().unwrap_or("unknown").to_string();
        let file_path = format!("{DIRECTORY}/{}", file_name);

        //create file
        let mut f = web::block(|| fs::File::create(file_path)).await.unwrap().unwrap();

        //write into the file
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            f = web::block(move || f.write_all(&data).map(|_| f)).await.unwrap().unwrap();
        }
    }

    println!("File uploaded successfully");
    HttpResponse::Ok().json(serde_json::json!({ "status": "success" }))
}

#[derive(Serialize)]
struct FileStruct {
    name: String,
    extention: String,
    url: String,
    size: u64,
}

#[get("/files")]
pub async fn list_files() -> HttpResponse {
    let dir = match read_dir(DIRECTORY) {
        Ok(dir) => dir,
        Err(_) => return HttpResponse::InternalServerError().json(serde_json::json!({ "status": "error", "message": "Failed to read directory" })),
    };

    let files: Vec<FileStruct> = dir.filter_map(|entry| {
        let entry = entry.ok()?;
        let path = entry.path();
        let ext = path.extension()?.to_str()?.to_string();
        let url = format!("http://{}/file/{}", URL_HOST, entry.file_name().into_string().ok()?);

        Some(FileStruct {
            name: entry.file_name().into_string().ok()?,
            extention: ext,
            url,
            size: entry.metadata().ok()?.len(),
        })
    }).collect();

    println!("Requested for Files successfully");
    HttpResponse::Ok().json(serde_json::json!({ "status": "success", "files": files }))
}

#[delete("/file/{filename:.*}")]
pub async fn delete_file(path: web::Path<String>) -> HttpResponse {
    let filename = path.as_str();
    let file_path = format!("{DIRECTORY}/{filename}");
    match remove_file(&file_path) {
        Ok(_) => {
            println!("File deleted successfully");
            return HttpResponse::Ok().body("File deleted successfully");
        },
        Err(err) => HttpResponse::InternalServerError().body(format!("Something went wrong, may file doesn't exist\n{}", err))
    }
}

#[get("/file/{filename:.*}")]
pub async fn serve_file(path: web::Path<String>) -> actix_web::Result<NamedFile> {
    let file_path = Path::new(DIRECTORY).join(&*path);
    Ok(NamedFile::open(file_path)?)
}
