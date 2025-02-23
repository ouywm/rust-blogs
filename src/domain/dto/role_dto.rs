use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRoleDTO {
    pub role_code: String,
    pub role_name: String,
    pub role_desc: Option<String>,
    pub order_num: Option<i32>,
    pub status: Option<i8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateRoleDTO {
    pub role_id: i64,
    pub role_name: String,
    pub role_desc: Option<String>,
    pub order_num: Option<i32>,
    pub status: Option<i8>,
} 