use crate::context::CONTEXT;
use oss_rust_sdk::prelude::*;
use std::collections::HashMap;
use rbatis::Error;

#[derive(Debug,Clone)]
pub struct OssService;

impl OssService {
    // 获取 OSS 实例
    fn get_oss_instance() -> OSS<'static> {
        let config = &CONTEXT.config;
        OSS::new(
            &config.access_key_id,
            &config.access_key_secret,
            &config.endpoint,
            &config.bucket,
        )
    }

    // 上传文件
    pub async fn upload_file(&self, file_bytes: &[u8], file_name: &str) -> Result<String, Error> {
        let oss = Self::get_oss_instance();
        
        // 设置文件类型
        let mut headers = HashMap::new();
        if let Some(content_type) = Self::get_content_type(file_name) {
            headers.insert("content-type", content_type);
        }

        // 上传文件
        match oss.put_object_from_buffer(file_bytes, file_name, Some(headers), None) {
            Ok(_) => {
                // 返回文件访问URL
                let url = format!("{}/{}", CONTEXT.config.bucket_domain, file_name);
                Ok(url)
            }
            Err(e) => Err(Error::from(format!("文件上传失败: {}", e))),
        }
    }

    // 删除文件
    pub async fn delete_file(&self, file_name: &str) -> Result<(), Error> {
        let oss = Self::get_oss_instance();
        
        match oss.delete_object(file_name) {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::from(format!("文件删除失败: {}", e))),
        }
    }

    // 获取文件的 Content-Type
    fn get_content_type(file_name: &str) -> Option<&'static str> {
        let ext = file_name.split('.').last()?;
        match ext.to_lowercase().as_str() {
            "jpg" | "jpeg" => Some("image/jpeg"),
            "png" => Some("image/png"),
            "gif" => Some("image/gif"),
            "pdf" => Some("application/pdf"),
            "doc" | "docx" => Some("application/msword"),
            "xls" | "xlsx" => Some("application/vnd.ms-excel"),
            "txt" => Some("text/plain"),
            _ => Some("application/octet-stream"),
        }
    }
} 