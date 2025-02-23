use crate::domain::dto::article_dto::{ArticleQueryDTO, CreateArticleDTO, UpdateArticleDTO};
use crate::util::response_result::{ResponseResult, PaginationResult};
use axum::{
    extract::{Json, Path, Query},
    response::IntoResponse,
};
use serde_json::{json, Value};
use crate::context::CONTEXT;

pub struct ArticleController;

impl ArticleController {
    // 创建文章
    pub async fn create_article(Json(dto): Json<CreateArticleDTO>) -> ResponseResult {
        match CONTEXT.article_service.create_article(dto).await {
            Ok(id) => ResponseResult::success("创建成功", Value::from(id)),
            Err(e) => ResponseResult::error(500, &e.to_string()),
        }
    }

    // 更新文章
    pub async fn update_article(Json(dto): Json<UpdateArticleDTO>) -> ResponseResult {
        match CONTEXT.article_service.update_article(dto).await {
            Ok(_) => ResponseResult::success("更新成功", Value::from(())),
            Err(e) => ResponseResult::error(500, &e.to_string()),
        }
    }

    // 删除文章
    pub async fn delete_article(Path(id): Path<i64>) -> ResponseResult {
        match CONTEXT.article_service.delete_article(id).await {
            Ok(_) => ResponseResult::success("删除成功", Value::from(())),
            Err(e) => ResponseResult::error(500, &e.to_string()),
        }
    }

    // 获取文章列表
    pub async fn get_articles(Query(query): Query<ArticleQueryDTO>) -> PaginationResult {
        match CONTEXT.article_service.get_articles(query.clone()).await {
            Ok((articles, total)) => PaginationResult::success(
                json!(articles),
                query.page.unwrap_or(1) as i32,
                query.size.unwrap_or(10) as i32,
                total as i64,
            ),
            Err(e) => PaginationResult::error(
                500,
                &e.to_string(),
            ),
        }
    }

    // 获取单个文章
    pub async fn get_article(Path(id): Path<i64>) -> ResponseResult {
        match CONTEXT.article_service.get_article(id).await {
            Ok(article) => ResponseResult::success("获取成功", json!(article)),
            Err(e) => ResponseResult::error(500, &e.to_string()),
        }
    }

    // 增加浏览量
    pub async fn increment_view(Path(id): Path<i64>) -> ResponseResult {
        match CONTEXT.article_service.increment_view_count(id).await {
            Ok(_) => ResponseResult::success("操作成功", Value::from(())),
            Err(e) => ResponseResult::error(500, &e.to_string()),
        }
    }
} 