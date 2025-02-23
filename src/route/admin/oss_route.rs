use crate::controller::admin::oss_controller::OssController;
use axum::{
    routing::{post, delete},
    Router,
    extract::Path,
};

pub struct OssRoute;

impl OssRoute {
    pub fn oss_route() -> Router {
        Router::new()
            .route("/sys_oss/upload", post(OssController::upload_file))
            .route("/sys_oss/delete/{file_name}", delete(OssController::delete_file))
    }
} 