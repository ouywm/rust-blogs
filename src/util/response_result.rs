use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize, Serializer};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseResult<T: Serialize = Value> {
    pub code: i32,
    pub message: String,
    pub data: T,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaginationResult<T = Value> {
    pub current_page: i32, // 当前页
    pub page_size: i32,    // 每页条数
    pub last_page: i32,    // 总页数
    pub total: String,     // 总条数
    pub message: String,   // 提示信息
    pub code: i32,         // 提示信息
    pub data: T,
}
impl<T: Serialize> IntoResponse for PaginationResult<T> {
    fn into_response(self) -> axum::response::Response {
        let status = match self.code {
            200 => StatusCode::OK,
            400 => StatusCode::BAD_REQUEST,
            401 => StatusCode::UNAUTHORIZED,
            404 => StatusCode::NOT_FOUND,
            500 => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::IM_A_TEAPOT,
        };

        Json(self).into_response()
    }
}
impl<T> PaginationResult<T> {
    /// 通用构造方法
    pub fn new(
        code: i32,
        message: String,
        data: T,
        current_page: i32,
        page_size: i32,
        total_records: i64,
    ) -> Self {
        let last_page = ((total_records as f64) / page_size as f64).ceil() as i32;
        Self {
            code,
            message,
            current_page,
            page_size,
            last_page,
            total: total_records.to_string(),
            data,
        }
    }

    /// 快速成功响应（固定消息）
    pub fn success(data: T, current_page: i32, page_size: i32, total: i64) -> Self {
        Self::new(
            200,
            "操作成功".to_string(),
            data,
            current_page,
            page_size,
            total,
        )
    }

    /// 自定义消息的成功响应
    pub fn success_msg(
        message: String,
        data: T,
        current_page: i32,
        page_size: i32,
        total: i64,
    ) -> Self {
        Self::new(200, message, data, current_page, page_size, total)
    }
}
impl ResponseResult<Value> {
    pub fn error(code: i32, message: &str) -> Self {
        ResponseResult {
            code,
            message: message.to_string(),
            data: Value::Null,
        }
    }
}

impl<T: Serialize> ResponseResult<T> {
    pub fn new(code: i32, message: String, data: T) -> Self {
        ResponseResult {
            code,
            message,
            data,
        }
    }

    pub fn success(message: &str, data: T) -> Self {
        ResponseResult {
            code: 200,
            message: message.to_string(),
            data,
        }
    }

    pub fn success_msg(msg: String, data: T) -> Self {
        ResponseResult {
            code: 200,
            message: msg,
            data,
        }
    }
}

// 为错误情况提供一个单独的实现
impl PaginationResult<Value> {
    pub fn error(code: i32, message: &str) -> Self {
        PaginationResult {
            current_page: 0,
            page_size: 0,
            last_page: 0,
            code,
            message: message.to_string(),
            data: Value::Null,
            total: "".to_string(),
        }
    }
}

impl<T: Serialize> IntoResponse for ResponseResult<T> {
    fn into_response(self) -> axum::response::Response {
        let status = match self.code {
            200 => StatusCode::OK,
            400 => StatusCode::BAD_REQUEST,
            401 => StatusCode::UNAUTHORIZED,
            404 => StatusCode::NOT_FOUND,
            500 => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::IM_A_TEAPOT,
        };

        Json(self).into_response()
    }
}
