use std::sync::{Arc, Mutex};

use axum::{
    extract::Path,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use serde_json::to_string;

use crate::cache::{Cache, File, FileFormat, ImageFormat};

#[derive(Serialize)]
pub struct IndexResponse {
    pub bytes_cached: usize,
    data: Vec<File>,
}

pub async fn create_document() {}
pub async fn upload_content() {}
pub async fn other_routes() -> Response<String> {
    return Response::new("success".into());
}

pub async fn get_image(
    state: Arc<Mutex<Cache>>,
    Path((docid, assid)): Path<(String, String)>,
) -> impl IntoResponse {
    let cache = Arc::clone(&state);

    let cache_lock = cache.lock().unwrap();
    let file = cache_lock.get(docid.clone(), assid.clone());
    let mut mime = "image/png";

    if let Some(f) = file {
        match f.format.clone() {
            FileFormat::IMAGE(i) => match i {
                ImageFormat::JPEG => {
                    mime = "image/jpeg";
                }
                ImageFormat::PNG => {
                    mime = "image/png";
                }
            },

            _ => {}
        }

        let raw = f.contents.clone();

        return ([("content-type", mime)], raw);
    } else {
        return ([("content-type", mime)], Vec::new());
    }
}

pub async fn get_asset(
    state: Arc<Mutex<Cache>>,
    Path((docid, assid)): Path<(String, String)>,
) -> impl IntoResponse {
    let cache = state.lock().unwrap();
    let file = cache.get(docid.clone(), assid.clone());

    match file {
        None => {
            return (
                [("content-type", "text/plain")],
                Response::new("404".to_string()),
            );
        }
        Some(file) => {
            let contents = String::from_utf8_lossy(&file.contents).into_owned();
            let mut content_type = "text/html";

            match &file.format {
                FileFormat::IMAGE(i) => match i {
                    ImageFormat::PNG => content_type = "image/png",
                    ImageFormat::JPEG => {
                        content_type = "image/jpeg";
                    }
                },

                _ => {}
            }

            return ([("content-type", content_type)], Response::new(contents));
        }
    }
}

pub async fn get_all_assets(state: Arc<Mutex<Cache>>) -> Response<String> {
    let cache = state.lock().unwrap();
    let files = cache.as_vec();

    let r = IndexResponse {
        bytes_cached: cache.size(),
        data: files,
    };

    let data = to_string(&r).unwrap();
    Response::new(data.into())
}
