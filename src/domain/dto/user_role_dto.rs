use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRoleDTO {
    pub user_id: i64,
    pub role_ids: Vec<i64>,
} 