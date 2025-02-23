use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateArticleDTO {
    pub category_id: i64,
    pub title: String,
    pub html_content: String,
    pub home_img: Option<String>,
    pub brief: Option<String>,
    pub is_visible: Option<bool>,
    pub is_top: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct UpdateArticleDTO {
    pub id: i64,
    pub category_id: i64,
    pub title: String,
    pub html_content: String,
    pub home_img: Option<String>,
    pub brief: Option<String>,
    pub is_visible: Option<bool>,
    pub is_top: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ArticleQueryDTO {
    pub page: Option<u64>,
    pub size: Option<u64>,
    pub category_id: Option<i64>,
    pub title: Option<String>,
    pub year: Option<String>,
} 