use rbatis::rbdc::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogUser {
    /// 用户ID（数据库自增主键）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,

    /// 认证方式类型 
    /// 0=邮箱 1=GitHub 2=Google 3=其他
    pub auth_type: i8,

    /// 认证标识（邮箱/第三方ID）
    pub auth_id: String,

    /// 密码哈希值（bcrypt格式）
    #[serde(skip_serializing)]
    pub password_hash: Option<String>,

    /// 用户显示名称
    pub nickname: String,

    /// 头像URL路径
    pub avatar_url: String,

    /// 用户状态 0=正常 1=锁定
    pub user_status: i8,

    /// 注册时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<DateTime>,

    /// 最后更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<DateTime>,

    /// 第三方平台用户ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_uid: Option<String>
}

impl BlogUser {
    /// 通用创建方法（基础字段初始化）
    pub fn new(auth_type: i8, auth_id: &str) -> Self {
        Self {
            user_id: None,
            auth_type,
            auth_id: auth_id.to_string(),
            password_hash: None,
            nickname: auth_id.split('@').next().unwrap_or("user").to_string(),
            avatar_url: "/static/default_avatar.png".into(),
            user_status: 0,
            create_time: None,
            update_time: None,
            provider_uid: None
        }
    }

}

/// 常用常量定义
pub mod auth_const {
    /// 认证类型
    pub mod auth_type {
        pub const EMAIL: i8 = 0;
        pub const GITHUB: i8 = 1;
        pub const GOOGLE: i8 = 2;
    }

    /// 用户状态
    pub mod user_status {
        pub const ACTIVE: i8 = 0;
        pub const LOCKED: i8 = 1;
    }
}
