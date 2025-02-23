use rbatis::executor::Executor;
use rbatis::rbdc::DateTime;
use rbatis::{crud, impl_delete, impl_select, Error, RBatis};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SysMenu {
    pub menu_id: Option<i64>, // id

    pub menu_name: Option<String>, // 组件名

    pub title_en: Option<String>, // 英文名称

    pub parent_id: Option<i64>, // 父级id

    pub order_num: Option<i64>, // 排序

    pub path: Option<String>, // 路由地址

    pub route_name: Option<String>, // 路由名称

    pub keep_alive: Option<i8>, // 是否缓存

    pub is_frame: Option<i8>, // 是否iframe

    pub menu_type: Option<String>, // 类型

    pub is_hide: Option<i8>, // 是否隐藏

    pub icon: Option<String>, // 图标

    pub remark: Option<String>, // 备注

    pub show_badge: Option<i8>, // 显示小红点

    pub show_text_badge: Option<String>, // 显示文本小红点

    pub is_hide_tab: Option<i8>, // 是否隐藏tab

    pub link: Option<String>, // 是否iframe

    pub create_by: Option<String>, // 创建人

    pub create_time: Option<DateTime>, // 创建时间

    pub update_by: Option<String>, // 更新人

    pub update_time: Option<DateTime>, // 更新时间

    pub permission: Option<String>, // 权限

    pub is_enable: Option<i64>, // 是否启用
}

// 基础 CRUD
crud!(SysMenu {}, "sys_menu");
impl_select!(SysMenu {select_by_id(menu_id:i64)-> Option => "`where menu_id = #{menu_id}`"});
impl_delete!(SysMenu {delete_by_menu_id(menu_id:i64) => "`where menu_id = #{menu_id}`"});

#[derive(Debug, Serialize, Deserialize)]
struct MenuIdWrapper {
    menu_id: i64,
}
impl SysMenu {
    // 根据父级id查询菜单
    pub async fn select_by_parent_id(rb: &RBatis, menu_id: i64) -> Result<Vec<SysMenu>, Error> {
        rb.query_decode(
            r#"
            SELECT * FROM sys_menu WHERE parent_id = ?
            "#,
            vec![rbs::to_value!(menu_id)],
        )
        .await
    }

    // 根据用户id查询菜单
    pub async fn get_menu_by_user_id(rb: &RBatis, user_id: i64) -> Result<Vec<SysMenu>, Error> {
        let sql = r#"
            SELECT  m.*
            FROM sys_menu m
            JOIN sys_role_menu rm ON m.menu_id = rm.menu_id
            JOIN sys_user_role ur ON rm.role_id = ur.role_id
            WHERE ur.user_id = ?
            ORDER BY m.parent_id, m.order_num
        "#;
        rb.query_decode(sql, vec![rbs::to_value!(user_id)])
            .await
            .map_err(|e| {
                Error::from(format!(
                    "查询菜单失败: {}, SQL: {}, 参数: {}",
                    e, sql, user_id
                ))
            })
    }

    // 根据角色id查询菜单
    pub async fn get_menu_by_role_id(rb: &RBatis, role_id: i64) -> Result<Vec<SysMenu>, Error> {
        let sql = r#"
            SELECT DISTINCT m.*
            FROM sys_menu m
            JOIN sys_role_menu rm ON m.menu_id = rm.menu_id
            JOIN sys_user_role ur ON rm.role_id = ur.role_id
            WHERE ur.role_id = ?
            ORDER BY m.parent_id, m.order_num
        "#;
        rb.query_decode(sql, vec![rbs::to_value!(role_id)])
            .await
            .map_err(|e| {
                Error::from(format!(
                    "查询菜单失败: {}, SQL: {}, 参数: {}",
                    e, sql, role_id
                ))
            })
    }

    // 获取所有菜单(不依赖角色)
    pub async fn get_all_menus(rb: &RBatis) -> Result<Vec<SysMenu>, Error> {
        rb.query_decode(
            r#"
            SELECT DISTINCT m.*
            FROM sys_menu m
            ORDER BY m.parent_id, m.order_num ASC
            "#,
            vec![],
        )
        .await
    }

    // 获取角色已分配的菜单ID列表
    pub async fn get_role_menu_ids(rb: &RBatis, role_id: i64) -> Result<Vec<i64>, Error> {
        // 先用 MenuIdWrapper 接收结果
        let menu_wrappers: Vec<MenuIdWrapper> = rb
            .query_decode(
                r#"
            SELECT menu_id
            FROM sys_role_menu
            WHERE role_id = ?
            "#,
                vec![rbs::to_value!(role_id)],
            )
            .await?;

        // 转换成 Vec<i64>
        Ok(menu_wrappers
            .into_iter()
            .map(|wrapper| wrapper.menu_id)
            .collect())
    }
}
