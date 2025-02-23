use crate::context::CONTEXT;
use crate::domain::dto::role_dto::{CreateRoleDTO, UpdateRoleDTO};
use crate::util::response_result::ResponseResult;
use axum::{
    extract::{Path, Query},
    Json,
};
use serde::Deserialize;
use serde_json::{json, Value};


#[derive(Debug, Deserialize)]
pub struct RoleListQuery {
    pub page: Option<u64>,
    pub size: Option<u64>,
    pub role_name: Option<String>,
}

// 分页查询角色列表
pub async fn list_roles_by_page(Query(query): Query<RoleListQuery>) -> ResponseResult {
    let page = query.page.unwrap_or(1);
    let size = query.size.unwrap_or(10);
    
    match CONTEXT.sys_role_service.list_roles_by_page(page, size, query.role_name).await {
        Ok((list, total)) => ResponseResult::success(
            "获取角色列表成功",
            json!({
                "list": list,
                "total": total
            })
        ),
        Err(e) => ResponseResult::error(500, &e.to_string()),
    }
}

// 创建角色
pub async fn create_role(Json(dto): Json<CreateRoleDTO>) -> ResponseResult {
    match CONTEXT.sys_role_service.create_role(dto).await {
        Ok(id) => ResponseResult::success("创建角色成功", Value::from(id)),
        Err(e) => ResponseResult::error(500, &e.to_string()),
    }
}

// 更新角色
pub async fn update_role(Json(dto): Json<UpdateRoleDTO>) -> ResponseResult {
    match CONTEXT.sys_role_service.update_role(dto).await {
        Ok(_) => ResponseResult::success("更新角色成功", Value::Null),
        Err(e) => ResponseResult::error(500, &e.to_string()),
    }
}

// 删除角色
pub async fn delete_role(Path(role_id): Path<i64>) -> ResponseResult {
    match CONTEXT.sys_role_service.delete_role(role_id).await {
        Ok(_) => ResponseResult::success("删除角色成功", Value::Null),
        Err(e) => ResponseResult::error(500, &e.to_string()),
    }
}

