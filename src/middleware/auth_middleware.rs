use crate::{
    auth::{AuthManager, JwtClaims},
    context::CONTEXT,
};
use axum::extract::Request;
use axum::http::{header, StatusCode};
use axum::{body::Body, middleware::Next, response::Response};
use bytes::Bytes;
use http_body_util::BodyExt;

pub async fn auth_middleware<B>(request: Request<B>, next: Next) -> Result<Response, StatusCode>
where
    B: axum::body::HttpBody<Data = Bytes> + Send + 'static,
    B::Error: std::fmt::Display,
{
    // 检查是否是白名单路径
    let path = request.uri().path();

    // 如果请求的路径在白名单中，则直接放行
    if CONTEXT.config.whitelist_paths.iter().any(|p| p == path) {
        // 收集原始请求体数据
        let (parts, body) = request.into_parts();
        let bytes = body
            .collect()
            .await
            .map_err(|_| StatusCode::BAD_REQUEST)?
            .to_bytes();
        let request = Request::from_parts(parts, Body::from(bytes));
        return Ok(next.run(request).await);
    }

    let mut request = request;

    // 从请求头获取token
    let token = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // 创建认证管理器
    let config = CONTEXT.get_jwt_config().await;
    let auth_manager = AuthManager::new(config);

    // 验证JWT
    match auth_manager.validate_jwt(token).await {
        Ok(claims) => {
            // 将用户信息添加到请求扩展中
            request.extensions_mut().insert(claims);

            // 收集原始请求体数据
            let (parts, body) = request.into_parts();
            let bytes = body
                .collect()
                .await
                .map_err(|_| StatusCode::BAD_REQUEST)?
                .to_bytes();
            let request = Request::from_parts(parts, Body::from(bytes));

            Ok(next.run(request).await)
        }
        Err(_) => Err(StatusCode::UNAUTHORIZED),
    }
}

// 权限检查中间件
pub async fn permission_middleware<B>(
    required_permission: &str,
    request: Request<B>,
    next: Next,
) -> Result<Response, StatusCode>
where
    B: axum::body::HttpBody<Data = Bytes> + Send + 'static,
    B::Error: std::fmt::Display,
{
    // 从请求扩展中获取用户信息
    let claims = request
        .extensions()
        .get::<JwtClaims>()
        .cloned()
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // 分割权限标识为三部分
    let permission_parts: Vec<&str> = required_permission.split(':').collect();
    if permission_parts.len() != 3 {
        return Err(StatusCode::BAD_REQUEST);
    }
    let module = permission_parts[0];
    let operation = permission_parts[1];
    let wildcard_permission = format!("{}:{}:*", module, operation);

    // 如果说这个操作要的权限是 sys:user:add,但是我有sys:user:*权限，则通过
    // 检查用户权限是否包含所需权限或通配符权限
    let has_permission = claims.perm.iter().any(|p| {
        p == required_permission || p == &wildcard_permission
    });
    if has_permission {
        let (parts, body) = request.into_parts();
        let bytes = body
            .collect()
            .await
            .map_err(|_| StatusCode::BAD_REQUEST)?
            .to_bytes();
        let request = Request::from_parts(parts, Body::from(bytes));
        Ok(next.run(request).await)
    } else {
        Err(StatusCode::FORBIDDEN)
    }

}
