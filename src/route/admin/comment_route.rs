use crate::controller::admin::comment_controller::CommentController;
use crate::middleware::auth_middleware::permission_middleware;
use axum::{
    middleware,
    routing::{delete, get, post},
    Router,
};

pub struct CommentRoute;

impl CommentRoute {
    pub fn comment_route() -> Router {
        Router::new()
            .route(
                "/comment",
                post(CommentController::create_comment).route_layer(middleware::from_fn(
                    |req, next| permission_middleware("sys:comment:add", req, next),
                )),
            )
            .route(
                "/comment/{id}",
                delete(CommentController::delete_comment).route_layer(middleware::from_fn(
                    |req, next| permission_middleware("sys:comment:delete", req, next),
                )),
            )
            .route(
                "/comment/update",
                post(CommentController::update_comment).route_layer(middleware::from_fn(
                    |req, next| permission_middleware("sys:comment:edit", req, next),
                )),
            )
            .route(
                "/comments",
                get(CommentController::get_comments).route_layer(middleware::from_fn(
                    |req, next| permission_middleware("sys:comment:list", req, next),
                )),
            )
            .route(
                "/article/{id}/comments",
                get(CommentController::get_comment_tree)
            )
    }
}