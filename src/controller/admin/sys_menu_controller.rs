use crate::auth::JwtClaims;
use crate::context::CONTEXT;
use crate::domain::dto::menu_dto::{CreateMenuDTO, UpdateMenuDTO};
use crate::util::response_result::ResponseResult;
use axum::extract::{Json, Path, Request};
use axum::http::StatusCode;
use axum::{Extension, RequestExt};
use serde_json::json;

// 根据用户id获取菜单信息
pub async fn get_menu_by_user(request: Request) -> ResponseResult {
    // 从请求扩展中获取用户信息
    let claims = request
        .extensions()
        .get::<JwtClaims>()
        .cloned()
        .ok_or(StatusCode::UNAUTHORIZED)
        .expect("获取用户信息失败");

    let user_id = claims.sub;
    match CONTEXT.sys_menu_service.get_menu_by_user_id(user_id).await {
        Ok(menus) => ResponseResult::success("获取用户菜单成功", json!(menus)),
        Err(e) => ResponseResult::error(500, &e.to_string()),
    }
}
// 根据用户id获取菜单信息
pub async fn get_menu_by_role(Path(role_id): Path<i64>) -> ResponseResult {
    match CONTEXT.sys_menu_service.get_menu_by_role_id(role_id).await {
        Ok(menus) => ResponseResult::success("获取角色菜单成功", json!(menus)),
        Err(e) => ResponseResult::error(500, &e.to_string()),
    }
}

// 获取角色的菜单权限
pub async fn get_role_menu_permissions(Path(role_id): Path<i64>) -> ResponseResult {
    match CONTEXT
        .sys_menu_service
        .get_role_menu_permissions(role_id)
        .await
    {
        Ok((menus, checked_ids)) => ResponseResult::success(
            "获取角色菜单权限成功",
            json!({
                "menus": menus,        // 所有菜单树
                "checkedKeys": checked_ids  // 已选中的菜单ID
            }),
        ),
        Err(e) => ResponseResult::error(500, &e.to_string()),
    }
}
// 获取角色的菜单权限
pub async fn delete_menu(Path(menu_id): Path<i64>) -> ResponseResult {
    match CONTEXT.sys_menu_service.delete_menu(menu_id).await {
        Ok(_) => ResponseResult::success("删除菜单成功！", json!({})),
        Err(e) => ResponseResult::error(500, &e.to_string()),
    }
}

// 更新角色菜单权限
pub async fn update_role_menu_permissions(
    Path(role_id): Path<i64>,
    Json(menu_ids): Json<Vec<i64>>,
) -> ResponseResult {
    match CONTEXT
        .sys_menu_service
        .update_role_menu_permissions(role_id, menu_ids)
        .await
    {
        Ok(_) => ResponseResult::success("更新角色菜单权限成功", json!(null)),
        Err(e) => ResponseResult::error(500, &e.to_string()),
    }
}

// 创建菜单
pub async fn create_menu(
    Extension(claims): Extension<JwtClaims>,
    Json(menu_dto): Json<CreateMenuDTO>,
) -> ResponseResult {
    let user_id = claims.sub;
    match CONTEXT
        .sys_menu_service
        .create_menu(menu_dto, user_id)
        .await
    {
        Ok(_) => ResponseResult::success("创建菜单成功", json!(null)),
        Err(e) => ResponseResult::error(500, &e.to_string()),
    }
}
// 修改菜单
pub async fn update_menu(
    Extension(claims): Extension<JwtClaims>,
    Json(menu_dto): Json<UpdateMenuDTO>,
) -> ResponseResult {
    let user_id = claims.sub;
    match CONTEXT
        .sys_menu_service
        .update_menu(menu_dto, user_id)
        .await
    {
        Ok(_) => ResponseResult::success("修改菜单成功", json!(null)),
        Err(e) => ResponseResult::error(500, &e.to_string()),
    }
}
