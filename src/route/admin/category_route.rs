use crate::controller::admin::category_controller::CategoryController;
use crate::middleware::auth_middleware::permission_middleware;
use axum::{
    middleware,
    routing::{delete, get, post},
    Router,
};

pub struct CategoryRoute;

impl CategoryRoute {
    pub fn category_route() -> Router {
        Router::new()
            .route(
                "/category",
                post(CategoryController::create_category).route_layer(middleware::from_fn(
                    |req, next| permission_middleware("sys:category:add", req, next),
                )),
            )
            .route(
                "/category/{id}",
                post(CategoryController::update_category).route_layer(middleware::from_fn(
                    |req, next| permission_middleware("sys:category:edit", req, next),
                )),
            )
            .route(
                "/category/{id}",
                delete(CategoryController::delete_category).route_layer(middleware::from_fn(
                    |req, next| permission_middleware("sys:category:delete", req, next),
                )),
            )
            .route(
                "/categories",
                get(CategoryController::get_categories).route_layer(middleware::from_fn(
                    |req, next| permission_middleware("sys:category:list", req, next),
                )),
            )
            .route(
                "/category/{id}",
                get(CategoryController::get_category).route_layer(middleware::from_fn(
                    |req, next| permission_middleware("sys:category:view", req, next),
                )),
            )
    }
}
