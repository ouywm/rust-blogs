use crate::domain::dto::category_dto::{CreateCategoryDTO, UpdateCategoryDTO};
use crate::service::category_service::CategoryService;
use crate::util::response_result::ResponseResult;
use axum::{
    extract::{Json, Path},
    response::IntoResponse,
};
use serde_json::{json, Value};
use crate::context::CONTEXT;

pub struct CategoryController;

impl CategoryController {
    // 创建分类
    pub async fn create_category(Json(dto): Json<CreateCategoryDTO>) -> ResponseResult {
        match CONTEXT.category_service.create_category(dto).await {
            Ok(id) => ResponseResult::success("创建成功", Value::from(id)),
            Err(e) => ResponseResult::error(500, &e.to_string()),
        }
    }

    // 更新分类
    pub async fn update_category(Json(dto): Json<UpdateCategoryDTO>) -> ResponseResult {
        match CONTEXT.category_service.update_category(dto).await {
            Ok(_) => ResponseResult::success("更新成功", Value::from(())),
            Err(e) => ResponseResult::error(500, &e.to_string()),
        }
    }

    // 删除分类
    pub async fn delete_category(Path(id): Path<i64>) -> ResponseResult {
        match CONTEXT.category_service.delete_category(id).await {
            Ok(_) => ResponseResult::success("删除成功", Value::from(())),
            Err(e) => ResponseResult::error(500, &e.to_string()),
        }
    }

    // 获取分类列表
    pub async fn get_categories() -> ResponseResult {
        match CONTEXT.category_service.get_categories().await {
            Ok(categories) => ResponseResult::success("获取成功", json!(categories)),
            Err(e) => ResponseResult::error(500, &e.to_string()),
        }
    }

    // 获取单个分类
    pub async fn get_category(Path(id): Path<i64>) -> ResponseResult {
        match CONTEXT.category_service.get_category(id).await {
            Ok(category) => ResponseResult::success("获取成功", json!(category)),
            Err(e) => ResponseResult::error(500, &e.to_string()),
        }
    }
}
