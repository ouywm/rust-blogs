use crate::{
    controller::admin::sys_user_controller, middleware::auth_middleware::permission_middleware,
};
use axum::{
    middleware,
    routing::{delete, get, post},
    Router,
};

pub struct SysUserRoute;

impl SysUserRoute {
    pub fn sys_user_route() -> Router {
        Router::new()
            // 登录接口不需要认证
            .route("/sys_user/login", post(sys_user_controller::login))
            // 用户列表接口需要认证和权限
            .route(
                "/sys_user/list",
                get(sys_user_controller::list_users_by_page).route_layer(middleware::from_fn(
                    |req, next| permission_middleware("sys:user:view", req, next),
                )),
            )
            .route(
                "/sys_user/create",
                post(sys_user_controller::create_user).route_layer(middleware::from_fn(
                    |req, next| permission_middleware("sys:user:add", req, next),
                )),
            )
            .route(
                "/sys_user/update",
                post(sys_user_controller::update_user).route_layer(middleware::from_fn(
                    |req, next| permission_middleware("sys:user:edit", req, next),
                )),
            )
            .route(
                "/sys_user/delete/{id}",
                delete(sys_user_controller::delete_user).route_layer(middleware::from_fn(
                    |req, next| permission_middleware("sys:user:delete", req, next),
                )),
            )
            .route(
                "/sys_user/{id}",
                get(sys_user_controller::get_user).route_layer(middleware::from_fn(|req, next| {
                    permission_middleware("sys:user:view", req, next)
                })),
            )
            .route(
                "/sys_user/roles/{id}",
                get(sys_user_controller::get_user_roles).route_layer(middleware::from_fn(
                    |req, next| permission_middleware("sys:user:view", req, next),
                )),
            )
            .route(
                "/sys_user/roles",
                post(sys_user_controller::update_user_roles).route_layer(middleware::from_fn(
                    |req, next| permission_middleware("sys:user:edit", req, next),
                )),
            )
    }
}
