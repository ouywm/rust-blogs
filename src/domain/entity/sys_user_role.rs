use crate::domain::entity::sys_role::SysRole;
use rbatis::rbdc::datetime::DateTime;
use rbatis::{crud, impl_select, Error, RBatis};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SysUserRole {
    /// 关联记录ID (primary key, auto increment)
    pub id: Option<i64>,

    /// 用户ID（逻辑外键->sys_user.user_id）
    pub user_id: i64,

    /// 角色ID（逻辑外键->sys_role.role_id）
    pub role_id: i64,

    /// 授权时间 (default: CURRENT_TIMESTAMP)
    pub create_time: Option<DateTime>,
}

// 基础 CRUD
crud!(SysUserRole {});

impl SysUserRole {
    // 查询用户的角色ID列表
    pub async fn select_role_by_user(rb: &RBatis, user_id: i64) -> Result<Vec<SysRole>, Error> {
        rb.query_decode(
            "select * from sys_user_role where user_id = ?",
            vec![rbs::to_value!(user_id)]
        ).await
    }
    // 查询用户的角色ID列表
    pub async fn select_role_ids_by_user(rb: &RBatis, user_id: i64) -> Result<Vec<i64>, Error> {
        rb.query_decode(
            "select role_id from sys_user_role where user_id = ?",
            vec![rbs::to_value!(user_id)]
        ).await
    }

    // 删除用户的所有角色
    pub async fn delete_by_user_id(rb: &RBatis, user_id: i64) -> Result<(), Error> {
        rb.exec(
            "delete from sys_user_role where user_id = ?",
            vec![rbs::to_value!(user_id)]
        ).await?;
        Ok(())
    }

    // 修改批量插入方法
    pub async fn batch_insert(rb: &RBatis, user_id: i64, role_ids: Vec<i64>) -> Result<(), Error> {
        if role_ids.is_empty() {
            return Ok(());
        }

        // 构建批量插入的 SQL
        let mut sql = String::from(
            "INSERT INTO sys_user_role (user_id, role_id, create_time) VALUES "
        );
        
        let now = DateTime::now();
        let mut params = Vec::new();
        
        for (i, role_id) in role_ids.iter().enumerate() {
            if i > 0 {
                sql.push_str(", ");
            }
            sql.push_str("(?, ?, ?)");
            params.push(rbs::to_value!(user_id));
            params.push(rbs::to_value!(role_id));
            params.push(rbs::to_value!(now.clone()));
        }

        // 添加 ON DUPLICATE KEY UPDATE 子句来处理唯一键约束
        sql.push_str(" ON DUPLICATE KEY UPDATE create_time = VALUES(create_time)");

        println!("执行SQL: {}", sql); // 添加日志
        println!("参数: {:?}", params); // 添加日志

        rb.exec(&sql, params).await?;
        Ok(())
    }
}

impl_select!(SysUserRole{select_by_userid(userid:&str) -> Option => "`where user_id = #{userid}`"});
