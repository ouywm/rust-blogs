use crate::util::response_result::ResponseResult;
use axum::{
    extract::{Multipart, Path},
    response::IntoResponse,
};
use uuid::Uuid;
use crate::context::CONTEXT;
use chrono::Local;
use serde_json::Value;

pub struct OssController;

impl OssController {
    pub async fn upload_file(mut multipart: Multipart) -> ResponseResult {
        // 处理上传的文件
        while let Some(field) = multipart.next_field().await.unwrap() {
            // 获取原始文件名和扩展名
            let file_name = match field.file_name() {
                Some(name) => name.to_string(),
                None => {
                    log::error!("未能获取到文件名");
                    return ResponseResult::error(400, "未能获取到文件名");
                }
            };
            log::debug!("原始文件名: {}", file_name);

            // 获取扩展名
            let ext = match file_name.split('.').last() {
                Some(ext) => ext.to_lowercase(),
                None => {
                    log::error!("未能获取到文件扩展名");
                    return ResponseResult::error(400, "未能获取到文件扩展名");
                }
            };
            log::debug!("扩展名: {}", ext);

            // 验证扩展名
            if !["jpg", "jpeg", "png", "gif", "webp"].contains(&ext.as_str()) {
                log::error!("不支持的文件类型: {}", ext);
                return ResponseResult::error(400, "只支持 jpg、jpeg、png、gif、webp 格式的图片");
            }

            // 生成日期路径
            let now = Local::now();
            let date_path = now.format("%Y/%m/%d").to_string();

            // 生成完整的文件路径
            let new_file_name = format!("image/{}/{}.{}", 
                date_path,
                Uuid::new_v4(),
                ext
            );

            // 读取文件内容
            let data = match field.bytes().await {
                Ok(data) => data,
                Err(e) => {
                    log::error!("读取文件内容失败: {}", e);
                    return ResponseResult::error(500, "读取文件内容失败");
                }
            };

            // 上传到 OSS
            match CONTEXT.oss_service.upload_file(&data, &new_file_name).await {
                Ok(url) => {
                    return ResponseResult::success("上传成功", Value::from(url));
                }
                Err(e) => {
                    log::error!("上传到OSS失败: {}", e);
                    return ResponseResult::error(500, &format!("上传失败: {}", e));
                }
            }
        }

        ResponseResult::error(400, "没有文件上传")
    }

    pub async fn delete_file(Path(file_name): Path<String>) -> ResponseResult {
        match CONTEXT.oss_service.delete_file(&file_name).await {
            Ok(_) => ResponseResult::success("删除成功", Value::from(())),
            Err(e) => ResponseResult::error(500, &format!("删除失败: {}", e)),
        }
    }
} 