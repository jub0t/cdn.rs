use std::sync::{Arc, Mutex};

use axum::{
    extract::Path,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use serde_json::to_string;

use crate::cache::{
    types::{FileFormat, ImageFormat},
    Cache,
};

#[derive(Serialize)]
pub struct IndexResponse {
    pub bytes_cached: usize,
    pub total_files: usize,
}

pub async fn create_document() {}
pub async fn upload_content() {}
pub async fn other_routes() -> Response<String> {
    Response::new("success".into())
}

pub async fn get_asset(
    state: Arc<Mutex<Cache>>,
    Path((docid, assid)): Path<(String, String)>,
) -> impl IntoResponse {
    let cache = state.lock().unwrap();
    let file = cache.get(docid.clone(), assid.clone());

    match file {
        None => {
            // File is not found in cache.
            // Let's re-cache the file and send back.

            ([("content-type", "text/plain")], Vec::new())
        }
        Some(file) => {
            let contents = file.contents.clone();
            let mut content_type = "application/text";

            match &file.format {
                FileFormat::IMAGE(i) => match i {
                    ImageFormat::PNG => content_type = "image/png",
                    ImageFormat::JPEG => {
                        content_type = "image/jpeg";
                    }
                },

                FileFormat::HTML => {
                    content_type = "text/html";
                }

                FileFormat::JS => content_type = "text/javascript",
                FileFormat::CSS => content_type = "text/css",
                _ => {}
            }

            ([("Content-Type", content_type)], contents)
        }
    }
}

pub async fn get_all_assets(state: Arc<Mutex<Cache>>) -> Response<String> {
    let cache = state.lock().unwrap();

    let r = IndexResponse {
        bytes_cached: cache.size(),
        total_files: cache.item_count(),
    };

    let data = to_string(&r).unwrap();
    Response::new(data)
}
