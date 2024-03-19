pub mod auth;
pub mod responses;

use std::sync::{Arc, Mutex};

use axum::{
    extract::Path,
    response::{IntoResponse, Response},
};
use serde_json::to_string;

use crate::cache::{format::format_to_mime, Cache};

use self::responses::{DiagnosticsResponse, IndexResponse};

pub async fn create_document() {}
pub async fn upload_content() {}
pub async fn other_routes() -> Response<String> {
    Response::new(to_string(&IndexResponse { success: true }).unwrap())
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

            ([("content-type", "text/plain".to_string())], Vec::new())
        }
        Some(file) => {
            let contents = file.contents.clone(); // The cached contents
            let content_type = format_to_mime(file.format.clone());

            ([("Content-Type", content_type)], contents)
        }
    }
}

pub async fn diagnostics(state: Arc<Mutex<Cache>>) -> Response<String> {
    let cache = state.lock().unwrap();

    let r = DiagnosticsResponse {
        bytes_cached: cache.size(),
        total_files: cache.item_count(),
    };

    let data = to_string(&r).unwrap();
    Response::new(data)
}
