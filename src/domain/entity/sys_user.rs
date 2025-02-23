use crate::domain::entity::sys_role::SysRole;
use chrono::{Local, TimeZone};
use rbatis::rbdc::types::datetime::DateTime;
use rbatis::{crud, impl_select, impl_update, Error, RBatis};
use rbs::Value;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SysUser {
    pub user_id: Option<i64>,
    pub username: Option<String>,
    pub password: Option<String>, // 不返回给前端
    pub nickname: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub avatar: Option<String>,
    pub status: Option<i8>,       // 0=禁用, 1=启用
    pub last_login: Option<DateTime>,
    pub create_time: Option<DateTime>,
    pub update_time: Option<DateTime>,
    pub sex: Option<i8>
}

#[derive(Deserialize)]
pub struct Count {
    pub count: u64,
}

// 基础 CRUD 和简单查询
crud!(SysUser {}, "sys_user");

// 使用 impl_select! 宏定义查询方法
impl_select!(SysUser{select_by_username(username:&str) -> Option => "`where username = #{username}`"});
impl_select!(SysUser{select_by_id(user_id:i64) -> Option => "`where user_id = #{user_id}`"});
impl_select!(SysUser{select_page_by_size(page_size:u64, offset:u64) -> Vec => 
    "`order by create_time desc limit #{page_size} offset #{offset}`"});

// 添加更新方法
impl_update!(SysUser{update_last_login(user_id:i64, last_login:DateTime) => 
    "update sys_user set last_login = #{last_login} where user_id = #{user_id}"});

impl SysUser {
    const INSERT_SQL: &'static str = r#"
        INSERT INTO sys_user (
            username, password, nickname, email, phone, 
            status, sex, create_time, update_time, avatar
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
    "#;

    const UPDATE_SQL: &'static str = r#"
        UPDATE sys_user SET 
            nickname = ?, 
            email = ?, 
            phone = ?, 
            status = ?, 
            sex = ?, 
            update_time = ? 
        WHERE user_id = ?
    "#;

    const SELECT_WITH_ROLES_SQL: &'static str = r#"
        SELECT 
            u.*,
            COALESCE(GROUP_CONCAT(DISTINCT ur.role_id), '') as role_ids
        FROM sys_user u
        LEFT JOIN sys_user_role ur ON u.user_id = ur.user_id
        GROUP BY 
            u.user_id, u.username, u.password, u.nickname, 
            u.email, u.phone, u.status, u.sex, 
            u.create_time, u.update_time, u.last_login, u.avatar
        ORDER BY u.create_time DESC
        LIMIT ? OFFSET ?
    "#;

    pub async fn select_page(
        rb: &RBatis,
        page_no: u64,
        page_size: u64,
    ) -> Result<Vec<SysUser>, Error> {
        let offset = (page_no - 1) * page_size;
        Self::select_page_by_size(rb, page_size, offset).await
    }

    pub async fn get_count(rb: &RBatis) -> Result<u64, Error> {
        let count:u64 = rb.query_decode("select count(*) as count from sys_user", vec![]).await.expect("get count error");
        Ok(count)
    }

    // 创建用户并返回ID
    pub async fn create_user(rb: &RBatis, user: &SysUser) -> Result<i64, Error> {
        let params = vec![
            rbs::to_value!(&user.username),
            rbs::to_value!(&user.password),
            rbs::to_value!(&user.nickname),
            rbs::to_value!(&user.email),
            rbs::to_value!(&user.phone),
            rbs::to_value!(&user.status),
            rbs::to_value!(&user.sex),
            rbs::to_value!(&user.create_time),
            rbs::to_value!(&user.update_time),
            rbs::to_value!(&user.avatar),
        ];

        let result = rb.exec(Self::INSERT_SQL, params).await?;
        
        match result.last_insert_id {
            Value::I64(id) => Ok(id),
            Value::U64(id) => Ok(id as i64),
            _ => Err(Error::from("无法获取插入ID")),
        }
    }

    // 更新用户基本信息
    pub async fn update_user_info(rb: &RBatis, user: &SysUser) -> Result<(), Error> {
        let params = vec![
            rbs::to_value!(&user.nickname),
            rbs::to_value!(&user.email),
            rbs::to_value!(&user.phone),
            rbs::to_value!(&user.status),
            rbs::to_value!(&user.sex),
            rbs::to_value!(&user.update_time),
            rbs::to_value!(&user.user_id),
        ];

        rb.exec(Self::UPDATE_SQL, params).await?;
        Ok(())
    }

    // 分页查询用户列表（包含角色信息）
    pub async fn select_page_with_roles(rb: &RBatis, page: u64, size: u64) -> Result<Vec<UserWithRoles>, Error> {
        let offset = (page - 1) * size;
        
        let rows: Vec<Value> = rb.query_decode(
            Self::SELECT_WITH_ROLES_SQL,
            vec![rbs::to_value!(size), rbs::to_value!(offset)]
        ).await?;
        
        let mut users = Vec::with_capacity(rows.len());
        for row in rows {
            let mut query_result: QueryResult = rbs::from_value(row)?;
            
            let role_ids = query_result.role_ids
                .unwrap_or_default()
                .split(',')
                .filter(|s| !s.is_empty())
                .filter_map(|id| id.parse::<i64>().ok())
                .collect();
            query_result.user.password = Some("******".to_string());
            users.push(UserWithRoles {
                user: query_result.user,
                role_ids,
            });
        }

        Ok(users)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserWithRolesAndPerms {
    pub user_id: Option<i64>,
    pub username: Option<String>,
    pub nickname: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub avatar: Option<String>,
    pub status: Option<i8>,
    pub create_time: Option<DateTime>,
    pub update_time: Option<DateTime>,
    pub last_login: Option<DateTime>,
    pub roles: Vec<SysRole>,
    pub permissions: Vec<String>,
}

impl Default for SysUser {
    fn default() -> Self {
        Self {
            user_id: None,
            username: None,
            password: None,
            nickname: None,
            email: None,
            phone: None,
            avatar: None,
            status: None,
            create_time: None,
            update_time: None,
            last_login: None,
            sex: None,
        }
    }
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserWithRoles {
    #[serde(flatten)]
    pub user: SysUser,
    pub role_ids: Vec<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
struct QueryResult {
    #[serde(flatten)]
    user: SysUser,
    role_ids: Option<String>,
}
