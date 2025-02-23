use crate::auth::err::AuthError as JwtAuthError;
use crate::auth::AuthManager;
use crate::context::CONTEXT;
use crate::domain::dto::user_dto::{CreateUserDTO, LoginResponseDTO, UpdateUserDTO, UserDTO};
use crate::domain::dto::user_role_dto::UserRoleDTO;
use crate::domain::entity::sys_menu::SysMenu;
use crate::domain::entity::sys_role::SysRole;
use crate::domain::entity::sys_user::UserWithRolesAndPerms;
use crate::domain::entity::sys_user::{SysUser, UserWithRoles};
use crate::domain::entity::sys_user_role::SysUserRole;
use crate::domain::vo::menu_vo::MenuVO;
use crate::pool;
use rbatis::rbdc::datetime::DateTime;
use rbatis::rbdc::db::ExecResult;
use rbatis::Error;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SysUserService;

impl SysUserService {
    // 根据用户id获取菜单信息
    pub async fn get_menu(&self, userid: i64) -> Result<Vec<MenuVO>, Error> {
        let menu_list = SysMenu::get_menu_by_user_id(pool!(), userid)
            .await
            .expect("菜单查询失败");
        let vec = MenuVO::build_menu_tree(&*menu_list);
        Ok(vec)
    }
    pub async fn sys_user_login(
        &self,
        login_user_dto: UserDTO,
    ) -> Result<LoginResponseDTO, AuthError> {
        let user = SysUser::select_by_username(pool!(), &login_user_dto.username)
            .await?
            .ok_or(AuthError::UserNotFound)?;

        // 判断用户的状态是否禁用
        if user.status.is_some() && user.status.eq(&Some(0)) {
            return Err(AuthError::StatusNo);
        }

        // 安全密码验证
        let input_password = login_user_dto.password;
        let stored_hash = user
            .password
            .as_deref()
            .ok_or(AuthError::InvalidCredentials)?;

        let password_valid = bcrypt::verify(input_password, stored_hash).expect("密码验证失败");

        if !password_valid {
            return Err(AuthError::InvalidCredentials);
        }

        // 获取用户角色和权限
        let user_id = user.user_id.ok_or(AuthError::UserNotFound)?;

        // 使用已有的方法获取用户角色和权限
        let roles = SysRole::get_user_roles_with_perms(pool!(), user_id).await?;

        // 获取所有权限标识
        let permissions: Vec<String> = roles
            .iter()
            .flat_map(|r| &r.permissions)
            // .filter_map(|p| p.permission.clone())
            .filter_map(|p| p.clone().permission.filter(|s| !s.is_empty()))
            .collect();

        // 转换为 HashSet 用于 JWT
        let role_codes: HashSet<String> =
            roles.iter().filter_map(|r| r.role_code.clone()).collect();
        let permission_set: HashSet<String> = permissions.clone().into_iter().collect();

        // 修改用户最后登录时间
        let now = DateTime::now();
        let update_user = SysUser {
            user_id: Some(user_id),
            last_login: Some(now),
            ..Default::default()
        };

        SysUser::update_by_column(pool!(), &update_user, "user_id")
            .await
            .expect("更新用户最后登录时间失败");

        // 生成JWT
        let config = CONTEXT.get_jwt_config().await;
        let manager = AuthManager::new(config);

        let token = manager
            .create_jwt(user_id, "PC", role_codes, permission_set)
            .await
            .map_err(|e| AuthError::JwtError(e.to_string()))?;

        let user_with_roles = UserWithRolesAndPerms {
            user_id: user.user_id,
            username: user.username,
            nickname: user.nickname,
            email: user.email,
            phone: user.phone,
            avatar: user.avatar,
            status: user.status,
            create_time: user.create_time,
            update_time: user.update_time,
            last_login: user.last_login,
            roles,
            permissions,
        };

        let dto = LoginResponseDTO::from_user_info(token, user_with_roles);

        Ok(dto)
    }

    pub async fn create_user(&self, dto: CreateUserDTO) -> Result<i64, Error> {
        let rb = pool!();
        let tx = rb.acquire_begin().await?;

        // 检查用户名是否存在
        let exists = SysUser::select_by_username(&tx, &dto.username)
            .await?
            .is_some();
        if exists {
            return Err(Error::from("用户名已存在"));
        }

        // 密码加密
        let hashed = bcrypt::hash(dto.password.as_bytes(), bcrypt::DEFAULT_COST)
            .map_err(|e| Error::from(format!("密码加密失败: {}", e)))?;

        let user = SysUser {
            user_id: None,
            username: Some(dto.username),
            password: Some(hashed),
            nickname: dto.nickname,
            email: dto.email,
            phone: dto.phone,
            status: dto.status,
            sex: dto.sex,
            create_time: Some(DateTime::now()),
            update_time: Some(DateTime::now()),
            last_login: None,
            avatar: Some("/avatar/default.png".to_string()),
        };

        // 创建用户
        let user_id = SysUser::create_user(&tx.rb, &user).await?;

        // 添加用户角色关系
        if !dto.role_ids.is_empty() {
            if let Err(e) = SysUserRole::batch_insert(&tx.rb, user_id, dto.role_ids).await {
                tx.rollback().await?;
                return Err(Error::from(format!("添加用户角色关系失败: {}", e)));
            }
        }

        tx.commit().await?;
        Ok(user_id)
    }

    pub async fn update_user(&self, dto: UpdateUserDTO) -> Result<(), Error> {
        let rb = pool!();
        let tx = rb.acquire_begin().await?;

        // 获取并更新用户信息
        let mut user = SysUser::select_by_id(&tx, dto.user_id)
            .await?
            .ok_or_else(|| Error::from("用户不存在"))?;

        user.nickname = dto.nickname;
        user.email = dto.email;
        user.phone = dto.phone;
        user.status = dto.status;
        user.sex = dto.sex;
        user.update_time = Some(DateTime::now());

        // 更新用户基本信息
        SysUser::update_user_info(&tx.rb, &user).await?;

        // 更新用户角色
        SysUserRole::delete_by_user_id(&tx.rb, dto.user_id).await?;
        if !dto.role_ids.is_empty() {
            SysUserRole::batch_insert(&tx.rb, dto.user_id, dto.role_ids).await?;
        }

        tx.commit().await?;
        Ok(())
    }

    pub async fn delete_user(&self, user_id: i64) -> Result<ExecResult, Error> {
        let exists = SysUser::select_by_id(pool!(), user_id).await?.is_some();
        if !exists {
            return Err(Error::from("用户不存在"));
        }

        // 使用 crud! 宏生成的 delete 方法
        SysUser::delete_by_column(pool!(), "user_id", user_id).await
    }

    pub async fn get_user(&self, user_id: i64) -> Result<Option<SysUser>, Error> {
        let mut user = SysUser::select_by_id(pool!(), user_id).await?;

        // 清除敏感信息
        if let Some(ref mut user) = user {
            user.password = None;
        }

        Ok(user)
    }

    pub async fn list_users_by_page(
        &self,
        page: u64,
        size: u64,
    ) -> Result<(Vec<UserWithRoles>, u64), Error> {
        let rb = pool!();

        let total = SysUser::get_count(rb).await?;
        let users = SysUser::select_page_with_roles(rb, page, size).await?;

        Ok((users, total))
    }

    pub async fn change_password(
        &self,
        user_id: i64,
        old_password: String,
        new_password: String,
    ) -> Result<(), Error> {
        let user = SysUser::select_by_id(pool!(), user_id)
            .await?
            .ok_or_else(|| Error::from("用户不存在"))?;

        // 验证旧密码
        let stored_hash = user
            .password
            .clone()
            .ok_or_else(|| Error::from("密码数据异常"))?;
        let password_valid = bcrypt::verify(&old_password, &stored_hash)
            .map_err(|e| Error::from(format!("密码验证失败: {}", e)))?;

        if !password_valid {
            return Err(Error::from("原密码错误"));
        }

        // 加密新密码
        let new_hash = bcrypt::hash(new_password.as_bytes(), bcrypt::DEFAULT_COST)
            .map_err(|e| Error::from(format!("密码加密失败: {}", e)))?;

        // 更新密码
        let mut user = user;
        user.password = Some(new_hash);
        user.update_time = Some(DateTime::now());

        SysUser::update_by_column(pool!(), &user, "user_id").await?;
        Ok(())
    }

    pub async fn reset_password(&self, user_id: i64) -> Result<String, Error> {
        let mut user = SysUser::select_by_id(pool!(), user_id)
            .await?
            .ok_or_else(|| Error::from("用户不存在"))?;

        // 生成随机密码
        let new_password = generate_random_password();
        let new_hash = bcrypt::hash(new_password.as_bytes(), bcrypt::DEFAULT_COST)
            .map_err(|e| Error::from(format!("密码加密失败: {}", e)))?;

        // 更新密码
        user.password = Some(new_hash);
        user.update_time = Some(DateTime::now());

        SysUser::update_by_column(pool!(), &user, "user_id").await?;
        Ok(new_password)
    }

    // 获取用户的角色ID列表
    pub async fn get_user_roles(&self, user_id: i64) -> Result<Vec<i64>, Error> {
        SysUserRole::select_role_ids_by_user(pool!(), user_id).await
    }

    // 更新用户的角色
    pub async fn update_user_roles(&self, dto: UserRoleDTO) -> Result<(), Error> {
        let rb = pool!();

        // 检查用户是否存在
        let exists = SysUser::select_by_id(rb, dto.user_id).await?.is_some();
        if !exists {
            return Err(Error::from("用户不存在"));
        }

        // 检查角色是否都存在
        for role_id in &dto.role_ids {
            let exists = SysRole::select_by_id(rb, *role_id).await?.is_some();
            if !exists {
                return Err(Error::from(format!("角色ID {} 不存在", role_id)));
            }
        }

        // 开始更新
        let tx = rb.acquire_begin().await.expect("事务获取失败");

        // 删除原有角色
        SysUserRole::delete_by_user_id(rb, dto.user_id).await?;

        // 添加新角色
        SysUserRole::batch_insert(rb, dto.user_id, dto.role_ids).await?;

        tx.commit().await?;
        Ok(())
    }
}

// 生成随机密码
fn generate_random_password() -> String {
    use rand::Rng;
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";
    let mut rng = rand::thread_rng();
    let password: String = (0..12)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    password
}

#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("用户不存在")]
    UserNotFound,
    #[error("账号或密码错误")]
    InvalidCredentials,
    #[error("权限不足")]
    PermissionDenied,
    #[error("数据库查询失败: {0}")]
    DbError(#[from] rbatis::Error),
    #[error("JWT错误: {0}")]
    JwtError(String),
    #[error("密码验证失败: {0}")]
    BcryptError(#[from] bcrypt::BcryptError),

    #[error("用户已被禁用")]
    StatusNo,
}

impl From<JwtAuthError> for AuthError {
    fn from(err: JwtAuthError) -> Self {
        AuthError::JwtError(err.to_string())
    }
}
