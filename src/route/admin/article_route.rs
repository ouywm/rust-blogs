use crate::controller::admin::article_controller::ArticleController;
use crate::middleware::auth_middleware::permission_middleware;
use axum::{
    middleware,
    routing::{delete, get, post},
    Router,
};

pub struct ArticleRoute;

impl ArticleRoute {
    pub fn article_route() -> Router {
        Router::new()
            .route(
                "/article",
                post(ArticleController::create_article).route_layer(middleware::from_fn(
                    |req, next| permission_middleware("sys:article:add", req, next),
                )),
            )
            .route(
                "/article/{id}",
                post(ArticleController::update_article).route_layer(middleware::from_fn(
                    |req, next| permission_middleware("sys:article:edit", req, next),
                )),
            )
            .route(
                "/article/{id}",
                delete(ArticleController::delete_article).route_layer(middleware::from_fn(
                    |req, next| permission_middleware("sys:article:delete", req, next),
                )),
            )
            .route(
                "/articles",
                get(ArticleController::get_articles).route_layer(middleware::from_fn(
                    |req, next| permission_middleware("sys:article:list", req, next),
                )),
            )
            .route(
                "/article/{id}",
                get(ArticleController::get_article).route_layer(middleware::from_fn(
                    |req, next| permission_middleware("sys:article:view", req, next),
                )),
            )
            .route(
                "/article/{id}/view",
                post(ArticleController::increment_view).route_layer(middleware::from_fn(
                    |req, next| permission_middleware("sys:article:view", req, next),
                )),
            )
    }
} 