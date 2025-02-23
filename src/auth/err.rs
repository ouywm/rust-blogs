use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("无效的令牌")]
    InvalidToken,

    #[error("用户未登录")]
    NotLogin,

    #[error("用户不存在")]
    UserNotFound,

    #[error("角色不存在")]
    RoleNotFound,

    #[error("权限不足")]
    PermissionDenied,

    #[error("并发访问冲突")]
    ConcurrentAccess(#[from] tokio::sync::TryLockError),

    #[error("系统错误")]
    SystemError(#[from] std::io::Error),

    #[error("JWT创建失败")]
    JwtCreationError,

    #[error("JWT已过期")]
    JwtExpired,

    #[error("令牌已被撤销")]
    TokenRevoked,

    #[error("会话已过期")]
    SessionExpired,

    #[error("权限不存在")]
    PermissionNotFound,

    #[error("无效的令牌签名")]
    InvalidSignature,

    #[error("签发者不匹配")]
    InvalidIssuer,

    #[error("设备标识无效")]
    InvalidDevice,

    #[error("权限集合为空")]
    EmptyPermissions,

    #[error("设备不匹配")]
    DeviceMismatch,
}


// 错误响应格式
#[derive(Serialize)]
struct ErrorResponse {
    code: u16,
    message: String,
}

