use std::fs as fs;
use std::fs::read_dir;
use std::io::Write;
use std::path::Path;

use actix_files::NamedFile;
use actix_multipart::Multipart;
use actix_web::{get, HttpResponse, post, web};
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
}

#[get("/files")] //TODO: make more efficient 
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
        })
    }).collect();

    println!("Requested for Files successfully");
    HttpResponse::Ok().json(serde_json::json!({ "status": "success", "files": files }))
}

#[get("/file/{filename:.*}")]
pub async fn serve_file(path: web::Path<String>) -> actix_web::Result<NamedFile> {
    let file_path = Path::new(DIRECTORY).join(&*path);
    Ok(NamedFile::open(file_path)?)
}
