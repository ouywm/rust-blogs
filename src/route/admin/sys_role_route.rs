use crate::{
    controller::admin::sys_role_controller, middleware::auth_middleware::permission_middleware,
};
use axum::{
    middleware,
    routing::{delete, get, post},
    Router,
};

pub struct SysRoleRoute;

impl SysRoleRoute {
    pub fn sys_role_route() -> Router {
        Router::new()
            // 角色列表接口（分页）
            .route(
                "/sys_role/list",
                get(sys_role_controller::list_roles_by_page).route_layer(middleware::from_fn(
                    |req, next| permission_middleware("sys:user:role", req, next),
                )),
            )
            // 创建角色
            .route(
                "/sys_role/create",
                post(sys_role_controller::create_role).route_layer(middleware::from_fn(
                    |req, next| permission_middleware("sys:role:add", req, next),
                )),
            )
            // 更新角色
            .route(
                "/sys_role/update",
                post(sys_role_controller::update_role).route_layer(middleware::from_fn(
                    |req, next| permission_middleware("sys:role:edit", req, next),
                )),
            )
            // 删除角色
            .route(
                "/sys_role/delete/{id}",
                delete(sys_role_controller::delete_role).route_layer(middleware::from_fn(
                    |req, next| permission_middleware("sys:role:delete", req, next),
                )),
            )
    }
}
