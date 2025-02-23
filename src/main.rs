use crate::context::CONTEXT;
use crate::middleware::auth_middleware::auth_middleware;
use crate::route::admin::article_route::ArticleRoute;
use crate::route::admin::category_route::CategoryRoute;
use crate::route::admin::comment_route::CommentRoute;
use crate::route::admin::oss_route::OssRoute;
use crate::route::admin::sys_menu_route::SysMenuRoute;
use crate::route::admin::sys_role_route::SysRoleRoute;
use crate::route::admin::sys_user_route::SysUserRoute;
use axum::routing::get;
use axum::Router;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

mod auth;
mod config;
mod context;
mod controller;
mod domain;
mod middleware;
mod route;
mod service;
mod util;

#[tokio::main]
async fn main() {
    let context = CONTEXT.initialize().await;

    // 创建路由
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .merge(SysUserRoute::sys_user_route())
        .merge(SysRoleRoute::sys_role_route())
        .merge(SysMenuRoute::sys_menu_route())
        .merge(OssRoute::oss_route())
        .merge(CategoryRoute::category_route())
        .merge(ArticleRoute::article_route())
        .merge(CommentRoute::comment_route())
        .layer(axum::middleware::from_fn(auth_middleware))
        .layer(
            CorsLayer::new() // 允许所有来源的跨域请求
                .allow_origin(Any) // 允许所有请求头
                .allow_methods(Any) // 允许所有请求方法
                .allow_headers(Any), // 允许所有请求头
        );

    let addr = format!("{}:{}", context.config.host, context.config.port);

    let listener = TcpListener::bind(&addr)
        .await
        .unwrap_or_else(|_| panic!("Failed to bind on {}", addr));

    println!(
        "🚀 服务启动成功，请访问 http://{}",
        listener.local_addr().expect("Failed to get local address")
    );
    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}
