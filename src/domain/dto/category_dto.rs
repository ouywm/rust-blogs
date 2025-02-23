use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCategoryDTO {
    pub name: String,
    pub icon_class: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateCategoryDTO {
    pub id: i64,
    pub name: String,
    pub icon_class: Option<String>,
} 