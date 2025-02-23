use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCommentDTO {
    pub post_id: i64,
    pub content: String,
    pub parent_id: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateCommentDTO {
    pub comment_id: i64,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommentQueryDTO {
    pub post_id: i64,
    pub page: Option<u64>,
    pub size: Option<u64>,
} 