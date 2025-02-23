pub mod err;
use crate::auth::err::AuthError;
use crate::config::JwtConfig;
use crate::context::CONTEXT;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::RwLock;
use uuid::Uuid;

// JWT Claims
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JwtClaims {
    pub sub: i64,              // 用户ID
    iat: u64,                  // 签发时间
    exp: u64,                  // 过期时间
    jti: Uuid,                 // JWT ID
    dvc: String,               // 设备标识
    iss: String,               // 签发者
    pub rol: HashSet<String>,  // 角色名称集合
    pub perm: HashSet<String>, // 最终权限集合
}

#[derive(Clone)]
pub struct AuthManager {
    // id_to_tokens: Arc<RwLock<HashMap<i64, HashMap<String, Uuid>>>>,
    // 新增JWT相关
    jwt_config: JwtConfig,
    decoding_key: DecodingKey,
    encoding_key: EncodingKey,
}

impl AuthManager {
    pub fn new(jwt_config: JwtConfig) -> Self {
        let encoding_key = EncodingKey::from_secret(jwt_config.secret.as_ref());
        let decoding_key = DecodingKey::from_secret(jwt_config.secret.as_ref());
        Self {
            // id_to_tokens: Arc::new(RwLock::new(HashMap::new())),
            jwt_config,
            decoding_key,
            encoding_key,
        }
    }
    pub async fn create_jwt(
        &self,
        user_id: i64,
        device: &str,
        roles: HashSet<String>,
        permissions: HashSet<String>,
    ) -> Result<String, AuthError> {
        // 生成JWT唯一标识
        let jti = Uuid::new_v4();
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // 构建claims
        let claims = JwtClaims {
            sub: user_id,
            iat: now,
            exp: now + self.jwt_config.expiration,
            jti,
            dvc: device.to_string(),
            iss: self.jwt_config.issuer.clone(),
            rol: roles,
            perm: permissions,
        };
        // 生成JWT
        let token = encode(
            &Header::new(self.jwt_config.algorithm.0.clone()),
            &claims,
            &self.encoding_key,
        )
        .map_err(|_| AuthError::JwtCreationError)?;

        // 更新设备映射
        // let mut id_map = self.id_to_tokens.write().await;
        // id_map
        //     .entry(user_id)
        //     .or_default()
        //     .insert(device.to_string(), jti);

        Ok(token)
    }

    // JWT验证方法
    pub async fn validate_jwt(&self, token: &str) -> Result<JwtClaims, AuthError> {
        // 解码验证
        let token_data = decode::<JwtClaims>(
            token,
            &self.decoding_key,
            &Validation::new(CONTEXT.config.algorithm.0),
        )
        .map_err(|e| match e.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => AuthError::JwtExpired,
            _ => AuthError::InvalidToken,
        })?;
        Ok(token_data.claims)
    }
}
