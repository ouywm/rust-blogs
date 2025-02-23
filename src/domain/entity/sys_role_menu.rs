use rbatis::{crud, impl_delete, impl_select, Error, RBatis};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SysRoleMenu {
    /// 角色ID（逻辑外键->sys_role.role_id）
    pub role_id: Option<i64>,

    /// 权限ID（逻辑外键->sys_permission.perm_id）
    pub menu_id: Option<i64>,
}

crud!(SysRoleMenu {}, "sys_role_menu");
impl_select!(SysRoleMenu {select_by_menu_id_and_role_id(role_id:i64,menu_id:i64) -> Option => "`where role_id = #{role_id} and menu_id = #{menu_id}`"});
impl_delete!(SysRoleMenu {delete_by_menu_id(menu_id:i64) => "`where menu_id = #{menu_id}`"});
#[derive(Debug, Serialize, Deserialize)]
struct MenuIdWrapper {
    menu_id: i64,
}

impl SysRoleMenu {
    // 查询角色的权限ID列表
    pub async fn select_perm_ids_by_role(rb: &RBatis, role_id: i64) -> Result<Vec<i64>, Error> {
        rb.query_decode(
            "select menu_id from sys_role_menu where role_id = ?",
            vec![rbs::to_value!(role_id)],
        )
        .await
    }

    pub async fn get_menu_id_by_role_id(
        rb: &RBatis,
        role_ids: Vec<i64>,
    ) -> Result<Vec<i64>, Error> {
        rb.query_decode(
            r#"
            SELECT m.menu_id
            FROM sys_menu m
            JOIN sys_role_menu rm ON m.menu_id = rm.menu_id
            WHERE rm.role_id in (?)
            "#,
            vec![rbs::to_value!(role_ids)],
        )
        .await
    }

    // 获取角色已分配的菜单ID列表
    pub async fn get_role_menu_ids(rb: &RBatis, role_id: i64) -> Result<Vec<i64>, Error> {
        rb.query_decode(
            r#"
            SELECT CAST(menu_id AS SIGNED) as menu_id
            FROM sys_role_menu 
            WHERE role_id = ?
            "#,
            vec![rbs::to_value!(role_id)],
        )
        .await
    }
}
