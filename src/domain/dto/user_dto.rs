use crate::domain::entity::sys_user::UserWithRolesAndPerms;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserDTO {
    pub username: String,

    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserDTO {
    pub username: String,
    #[serde(default = "default_password")]
    pub password: String,
    pub nickname: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub status: Option<i8>,
    pub sex: Option<i8>,
    pub role_ids: Vec<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserDTO {
    pub user_id: i64,
    pub nickname: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub status: Option<i8>,
    pub sex: Option<i8>,
    pub role_ids: Vec<i64>,
}

fn default_password() -> String {
    "123456".to_string()
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponseDTO {
    pub id: String,
    pub account: String,
    pub name: String,
    pub access_token: String,
    pub permissions: Vec<String>,
    pub roles: Vec<RoleWithPermDTO>,
    pub create_date: String,
    pub state: i8,
    pub avatar: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoleWithPermDTO {
    pub id: String,
    pub name: String,
    pub permission_ids: Vec<String>,
    pub permissions: Vec<PermissionDTO>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PermissionDTO {
    pub id: String,
    pub name: String,
    pub permission: String,
    pub path: String,
    pub create_date: String,
}

impl LoginResponseDTO {
    pub fn from_user_info(token: String, user_info: UserWithRolesAndPerms) -> Self {
        Self {
            id: user_info.user_id.unwrap_or_default().to_string(),
            account: user_info.username.unwrap_or_default(),
            name: user_info.nickname.unwrap_or_default(),
            access_token: token,
            avatar: user_info.avatar,
            permissions: user_info.permissions,
            roles: user_info
                .roles
                .into_iter()
                .map(|role| RoleWithPermDTO {
                    id: role.role_id.unwrap_or_default().to_string(),
                    name: role.role_name.unwrap_or_default(),
                    permissions: role
                        .permissions
                        .clone()
                        .into_iter()
                        .map(|perm| PermissionDTO {
                            id: perm.menu_id.unwrap_or_default().to_string(),
                            name: perm.menu_name.expect(""),
                            permission: perm.permission.unwrap_or_default(),
                            path: perm.path.unwrap_or_default(),
                            create_date: perm.create_time.unwrap_or_default().to_string(),
                        })
                        .collect(),
                    permission_ids: role
                        .permissions
                        .iter()
                        .map(|p| p.menu_id.unwrap_or_default().to_string())
                        .collect(),
                })
                .collect(),
            create_date: user_info.create_time.unwrap_or_default().to_string(),
            state: user_info.status.unwrap_or(1),
        }
    }

}
