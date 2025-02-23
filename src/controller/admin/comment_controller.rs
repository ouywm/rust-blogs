use crate::domain::dto::comment_dto::{CommentQueryDTO, CreateCommentDTO, UpdateCommentDTO};
use crate::util::response_result::{ResponseResult, PaginationResult};
use axum::{
    extract::{Json, Path, Query},
    Extension,
};
use serde_json::{json, Value};
use crate::auth::JwtClaims;
use crate::context::CONTEXT;

pub struct CommentController;

impl CommentController {
    // 创建评论
    pub async fn create_comment(
        Extension(claims): Extension<JwtClaims>,
        Json(dto): Json<CreateCommentDTO>,
    ) -> ResponseResult {
        match CONTEXT.comment_service.create_comment(claims.sub, dto).await {
            Ok(id) => ResponseResult::success("评论成功", Value::from(id)),
            Err(e) => ResponseResult::error(500, &e.to_string()),
        }
    }

    // 更新评论
    pub async fn update_comment(
        Extension(claims): Extension<JwtClaims>,
        Json(dto): Json<UpdateCommentDTO>,
    ) -> ResponseResult {
        match CONTEXT.comment_service.update_comment(claims.sub, dto).await {
            Ok(_) => ResponseResult::success("更新成功", Value::from(())),
            Err(e) => ResponseResult::error(500, &e.to_string()),
        }
    }

    // 删除评论
    pub async fn delete_comment(
        Extension(_claims): Extension<JwtClaims>,
        Path(comment_id): Path<i64>,
    ) -> ResponseResult {
        match CONTEXT.comment_service.delete_comment(comment_id).await {
            Ok(_) => ResponseResult::success("删除成功", Value::from(())),
            Err(e) => ResponseResult::error(500, &e.to_string()),
        }
    }

    // 获取评论列表
    pub async fn get_comments(Query(query): Query<CommentQueryDTO>) -> PaginationResult {
        match CONTEXT.comment_service.get_comments(query.clone()).await {
            Ok((comments, total)) => PaginationResult::success(
                json!(comments),
                query.page.unwrap_or(1) as i32,
                query.size.unwrap_or(10) as i32,
                total as i64,
            ),
            Err(e) => PaginationResult::error(500, &e.to_string()),
        }
    }

    // 获取文章评论树
    pub async fn get_comment_tree(Path(post_id): Path<i64>) -> ResponseResult {
        match CONTEXT.comment_service.get_comment_tree(post_id).await {
            Ok(comments) => ResponseResult::success("获取成功", json!(comments)),
            Err(e) => ResponseResult::error(500, &e.to_string()),
        }
    }
} 