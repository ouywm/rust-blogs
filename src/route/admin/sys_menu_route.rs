use crate::controller::admin::sys_menu_controller;
use crate::middleware::auth_middleware::permission_middleware;
use axum::routing::{delete, post, put};
use axum::{middleware, routing::get, Router};

pub struct SysMenuRoute;

impl SysMenuRoute {
    pub fn sys_menu_route() -> Router {
        Router::new()
            .route(
                "/sys_menu/list",
                get(sys_menu_controller::get_menu_by_user),
                //     .route_layer(middleware::from_fn(
                //     |req, next| permission_middleware("sys:user:view", req, next),
                // )),
            )
            .route(
                "/sys_menu/role/{role_id}",
                get(sys_menu_controller::get_menu_by_role).route_layer(middleware::from_fn(
                    |req, next| permission_middleware("sys:menu:view", req, next),
                )),
            )
            .route(
                "/sys_menu/role/{role_id}/permissions",
                get(sys_menu_controller::get_role_menu_permissions)
                    .route_layer(middleware::from_fn(
                        |req, next| permission_middleware("sys:role:perm", req, next),
                    )),
            )
            .route(
                "/sys_role/menus/{role_id}",
                post(sys_menu_controller::update_role_menu_permissions).route_layer(middleware::from_fn(
                    |req, next| permission_middleware("sys:role:perm", req, next),
                )),
            )
            .route(
                "/sys_menu/create",
                post(sys_menu_controller::create_menu).route_layer(middleware::from_fn(
                    |req, next| permission_middleware("sys:menu:add", req, next),
                )),
            )
            // 根据菜单id删除菜单
            .route(
                "/sys_menu/delete/{menu_id}",
                delete(sys_menu_controller::delete_menu).route_layer(middleware::from_fn(
                    |req, next| permission_middleware("sys:menu:delete", req, next),
                )),
            )
            // 修改菜单
            .route(
                "/sys_menu/update",
                put(sys_menu_controller::update_menu).route_layer(middleware::from_fn(
                    |req, next| permission_middleware("sys:menu:update", req, next),
                ))
            )
    }
}
