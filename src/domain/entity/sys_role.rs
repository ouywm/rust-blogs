use crate::auth::err::AuthError;
use crate::domain::entity::sys_menu::SysMenu;
use crate::pool;
use rbatis::executor::Executor;
use rbatis::rbdc::types::datetime::DateTime;
use rbatis::{crud, htmlsql, impl_select, Error, RBatis};
use rbs::Value;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SysRole {
    /// 角色唯一标识 (primary key, auto increment)
    pub role_id: Option<i64>,

    /// 角色编码（英文唯一键）
    pub role_code: Option<String>,

    /// 角色显示名称
    pub role_name: Option<String>,

    /// 角色描述
    pub role_desc: Option<String>,

    /// 显示排序（升序，default: 0）
    pub order_num: Option<i32>,

    /// 状态: 0=禁用, 1=启用 (default: 1)
    pub status: Option<i8>,

    /// 记录创建时间 (default: CURRENT_TIMESTAMP)
    pub create_time: Option<DateTime>,

    /// 最后更新时间 (default: CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP)
    pub update_time: Option<DateTime>,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub permissions: Vec<SysMenu>,
}

#[derive(Debug, Deserialize)]
pub struct RoleCodeResult {
    #[serde(rename = "role_code")]
    pub role_code: String,
}

// 基础 CRUD
crud!(SysRole {}, "sys_role");

// 查询方法
impl_select!(SysRole{select_by_id(role_id:i64) -> Option => "`where role_id = #{role_id}`"});

impl SysRole {
    // 根据角色编码查询
    pub async fn select_by_role_code(
        rb: &RBatis,
        role_code: &str,
    ) -> Result<Option<SysRole>, Error> {
        rb.query_decode(
            "select * from sys_role where role_code = ?",
            vec![rbs::to_value!(role_code)],
        )
        .await
    }

    // 分页查询
    pub async fn select_page(
        rb: &RBatis,
        page_no: u64,
        page_size: u64,
    ) -> Result<Vec<SysRole>, Error> {
        let offset = (page_no - 1) * page_size;
        rb.query_decode(
            "select * from sys_role order by order_num asc, create_time desc limit ? offset ?",
            vec![rbs::to_value!(page_size), rbs::to_value!(offset)],
        )
        .await
    }

    // 查询总数
    pub async fn get_count(rb: &RBatis) -> Result<u64, Error> {
        #[derive(Deserialize)]
        struct Count {
            count: u64,
        }

        let count: Option<Count> = rb
            .query_decode("select count(1) as count from sys_role", vec![])
            .await?;

        Ok(count.map(|c| c.count).unwrap_or(0))
    }

    // 获取用户角色编码
    htmlsql!(
    query_user_roles(
        rb: &dyn Executor,
        user_id: i64
    ) -> Result<Vec<RoleCodeResult>, Error> => "templates/role.html");

    pub async fn get_user_role_codes(
        rb: &dyn Executor,
        user_id: i64,
    ) -> Result<HashSet<String>, AuthError> {
        let roles = Self::query_user_roles(rb, user_id)
            .await
            .expect("Failed to query user roles");
        Ok(roles.into_iter().map(|r| r.role_code).collect())
    }

    pub async fn get_user_roles_with_perms(
        rb: &RBatis,
        user_id: i64,
    ) -> Result<Vec<SysRole>, Error> {
        // 获取用户的角色
        let mut roles: Vec<SysRole> = rb
            .query_decode(
                r#"
            SELECT DISTINCT
                r.role_id, r.role_code, r.role_name, r.role_desc,
                r.order_num, r.status, r.create_time, r.update_time
            FROM sys_role r
            JOIN sys_user_role ur ON r.role_id = ur.role_id
            WHERE ur.user_id = ?
            "#,
                vec![rbs::to_value!(user_id)],
            )
            .await?;

        log::debug!("查询到的角色: {:#?}", roles);

        // 获取每个角色的权限
        for role in &mut roles {
            let permissions: Vec<SysMenu> = rb
                .query_decode(
                    r#"

	            SELECT DISTINCT
                    m.*
                FROM sys_menu m
                JOIN sys_role_menu rm ON m.menu_id = rm.menu_id
                WHERE rm.role_id = ?
                ORDER BY m.order_num ASC
                "#,
                    vec![rbs::to_value!(role.role_id.unwrap_or_default())],
                )
                .await?;

            log::debug!(
                "角色 {} 的权限: {:#?}",
                role.role_id.unwrap_or_default(),
                permissions
            );
            role.permissions = permissions;
        }

        Ok(roles)
    }

    pub async fn select_by_user_id(rb: &RBatis, user_id: i64) -> Result<Vec<SysRole>, Error> {
        let sql = r#"
            SELECT r.* 
            FROM sys_role r
            INNER JOIN sys_user_role ur ON r.role_id = ur.role_id
            WHERE ur.user_id = ?
            AND r.status = 1
            ORDER BY r.order_num ASC
        "#;

        rb.query_decode(sql, vec![rbs::to_value!(user_id)]).await
    }

    pub async fn get_role_permissions(rb: &RBatis, role_id: i64) -> Result<Vec<String>, Error> {
        let sql = r#"
            SELECT DISTINCT p.perm_code
            FROM sys_permission p
            INNER JOIN sys_menu_permission mp ON p.perm_id = mp.perm_id
            INNER JOIN sys_role_menu rm ON mp.menu_id = rm.menu_id
            WHERE rm.role_id = ? AND p.status = 1
        "#;

        rb.query_decode(sql, vec![rbs::to_value!(role_id)]).await
    }

    // 分页查询
    pub async fn select_role_page_by_name(
        rb: &RBatis,
        page: u64,
        size: u64,
        role_name: Option<String>,
    ) -> Result<(Vec<SysRole>, u64), Error> {
        // 构建查询条件
        let mut where_clause = String::new();
        let mut params: Vec<Value> = vec![];

        if let Some(name) = role_name {
            where_clause.push_str(" WHERE role_name LIKE ?");
            params.push(rbs::to_value!(format!("%{}%", name)));
        }

        // 计算总数
        let count_sql = format!("SELECT COUNT(*) as total FROM sys_role{}", where_clause);
        let total: u64 = rb.query_decode(&count_sql, params.clone()).await?;

        // 分页查询
        let offset = (page - 1) * size;
        let list_sql = format!(
            "SELECT * FROM sys_role{} ORDER BY create_time DESC LIMIT ? OFFSET ?",
            where_clause
        );
        params.push(rbs::to_value!(size));
        params.push(rbs::to_value!(offset));

        let roles: Vec<SysRole> = rb.query_decode(&list_sql, params).await?;

        Ok((roles, total))
    }


}

impl Default for SysRole {
    fn default() -> Self {
        Self {
            role_id: None,
            role_code: None,
            role_name: None,
            role_desc: None,
            order_num: None,
            status: None,
            create_time: None,
            update_time: None,
            permissions: vec![],
        }
    }
}
