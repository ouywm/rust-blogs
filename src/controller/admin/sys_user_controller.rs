use crate::context::CONTEXT;
use crate::domain::dto::user_dto::{CreateUserDTO, UpdateUserDTO, UserDTO};
use crate::domain::dto::user_role_dto::UserRoleDTO;
use crate::util::response_result::ResponseResult;
use axum::extract::Query;
use axum::{extract::Path, Json};
use serde_json::Value;
use std::collections::HashMap;

pub async fn login(Json(login_dto): Json<UserDTO>) -> ResponseResult {
    match CONTEXT.sys_user_service.sys_user_login(login_dto).await {
        Ok(user_info) => ResponseResult::success(
            "登录成功",
            serde_json::to_value(user_info).expect("Failed to serialize response"),
        ),
        Err(e) => ResponseResult::error(500, &e.to_string()),
    }
}

// 创建用户
pub async fn create_user(Json(dto): Json<CreateUserDTO>) -> ResponseResult {
    match CONTEXT.sys_user_service.create_user(dto).await {
        Ok(id) => ResponseResult::success("创建成功", Value::from(id)),
        Err(e) => ResponseResult::error(400, &e.to_string()),
    }
}

// 更新用户
pub async fn update_user(Json(dto): Json<UpdateUserDTO>) -> ResponseResult {
    match CONTEXT.sys_user_service.update_user(dto).await {
        Ok(_) => ResponseResult::success("更新成功", Value::from(())),
        Err(e) => ResponseResult::error(400, &e.to_string()),
    }
}

// 删除用户
pub async fn delete_user(Path(user_id): Path<i64>) -> ResponseResult {
    match CONTEXT.sys_user_service.delete_user(user_id).await {
        Ok(_) => ResponseResult::success("删除成功", Value::from(())),
        Err(e) => ResponseResult::error(400, &e.to_string()),
    }
}

// 获取用户详情
pub async fn get_user(Path(user_id): Path<i64>) -> ResponseResult {
    match CONTEXT.sys_user_service.get_user(user_id).await {
        Ok(Some(user)) => ResponseResult::success(
            "获取成功",
            serde_json::to_value(user).unwrap_or(Value::Null),
        ),
        Ok(None) => ResponseResult::error(404, "用户不存在"),
        Err(e) => ResponseResult::error(400, &e.to_string()),
    }
}

// 分页查询用户列表
pub async fn list_users_by_page(Query(params): Query<HashMap<String, String>>) -> ResponseResult {
    let page = params.get("page").and_then(|s| s.parse().ok()).unwrap_or(1);
    let size = params
        .get("size")
        .and_then(|s| s.parse().ok())
        .unwrap_or(10);

    match CONTEXT
        .sys_user_service
        .list_users_by_page(page, size)
        .await
    {
        Ok((users, total)) => {
            let data = serde_json::json!({
                "list": users,
                "total": total
            });
            ResponseResult::success("查询成功", data)
        }
        Err(e) => ResponseResult::error(400, &e.to_string()),
    }
}

// 获取用户角色
pub async fn get_user_roles(Path(user_id): Path<i64>) -> ResponseResult {
    match CONTEXT.sys_user_service.get_user_roles(user_id).await {
        Ok(role_ids) => ResponseResult::success(
            "获取用户角色成功",
            serde_json::to_value(role_ids).unwrap_or(Value::Null),
        ),
        Err(e) => ResponseResult::error(500, &e.to_string()),
    }
}

// 更新用户角色
pub async fn update_user_roles(Json(dto): Json<UserRoleDTO>) -> ResponseResult {
    match CONTEXT.sys_user_service.update_user_roles(dto).await {
        Ok(_) => ResponseResult::success("更新用户角色成功", Value::Null),
        Err(e) => ResponseResult::error(500, &e.to_string()),
    }
}