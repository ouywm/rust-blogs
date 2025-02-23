use chrono::Local;
use config::{Config, Environment, File};
use fast_log::consts::LogSize;
use fast_log::plugin::file_split::{DateType, KeepType, LogPack, Rolling, RollingType};
use fast_log::plugin::packer::LogPacker;
use jsonwebtoken::{Algorithm, Header};
use serde::Deserialize;
use std::ops::Deref;
use std::str::FromStr;
use thiserror::Error;

// JWT配置结构
#[derive(Clone, Debug, Deserialize, Default)]
pub struct JwtConfig {
    pub secret: String,
    pub expiration: u64, // 秒
    pub algorithm: JwtAlgorithm,
    pub issuer: String,
}
#[derive(Debug, Deserialize, Clone, Default)]
#[serde(try_from = "String")]
pub struct JwtAlgorithm(pub(crate) Algorithm);

impl TryFrom<String> for JwtAlgorithm {
    type Error = String;

    fn try_from(value: String) -> Result<JwtAlgorithm, Self::Error> {
        Algorithm::from_str(&value)
            .map(JwtAlgorithm)
            .map_err(|_| format!("不支持的JWT算法: {}", value))
    }
}
// 添加Deref实现以便直接访问算法
impl Deref for JwtAlgorithm {
    type Target = Algorithm;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// 应用配置结构
#[derive(Deserialize, Debug, Clone)]
pub struct ApplicationConfig {
    // jwt
    pub secret: String,
    pub expiration: u64, // 秒
    pub algorithm: JwtAlgorithm,
    pub issuer: String,

    // 数据库
    pub db_url: String,
    pub db_pool_timeout: u64,
    pub db_pool_len: u64,

    // 日志
    pub debug: bool,
    pub log_level: String,

    // web
    pub port: u16,
    pub host: String,
    pub name: String,

    // 白名单路径
    pub whitelist_paths: Vec<String>,

    // oss
    pub access_key_id: String,
    pub access_key_secret: String,
    pub bucket: String,
    pub endpoint: String,
    pub bucket_domain: String,
}
impl ApplicationConfig {
    pub fn init() -> ApplicationConfig {
        // let env = std::env::var("APP_ENV").unwrap_or_else(|_| "dev".into());
        // log::debug!("当前环境: {}", env);
        let env = "dev";
        let config_ = Config::builder()
            .add_source(File::with_name(&format!(
                "rust-oy-blogs/config/{}.toml",
                env
            )))
            .build()
            .expect("构建配置错误");
        let config: ApplicationConfig = config_.try_deserialize().expect("反序列化配置文件错误");

        // 获取当前日期作为文件名
        let date = Local::now().format("%Y-%m-%d").to_string();
        let log_path = format!("target/logs/app.{}.log", date);
        // 构建日志配置
        let log_config = fast_log::Config::new()
            .chan_len(Some(100000))
            .console()
            .file_split(
                &log_path,
                Rolling::new(RollingType::ByDate(DateType::Day)),
                KeepType::KeepNum(20),
                LogPacker {},
            ) // 使用带日期的文件名
            .level(config.log_level.parse().expect("日志等级解析错误"));

        // 初始化日志系统
        fast_log::init(log_config).expect("rbatis initConfig fail");

        log::debug!("日志初始化成功! 日志级别: {}", config.log_level);
        log::debug!("当前配置: \r\n{:#?}", config);
        config
    }
}

impl Default for ApplicationConfig {
    fn default() -> Self {
        Self {
            secret: "".to_string(),
            expiration: 0,
            algorithm: Default::default(),
            issuer: "".to_string(),
            db_url: "".to_string(),
            db_pool_timeout: 0,
            db_pool_len: 0,
            debug: true,
            log_level: "info".to_string(),
            port: 3000,
            host: "127.0.0.1".to_string(),
            name: "web".to_string(),
            whitelist_paths: vec![],
            access_key_id: "".to_string(),
            access_key_secret: "".to_string(),
            bucket: "".to_string(),
            endpoint: "".to_string(),
            bucket_domain: "".to_string(),
        }
    }
}
