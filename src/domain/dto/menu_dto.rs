use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateMenuDTO {
    pub title: Option<String>,      // 菜单名称
    pub path: Option<String>,       // 路由地址
    pub route_name: Option<String>, // 路由名称
    pub icon: Option<String>,       // 图标
    pub order_num: Option<i64>,     // 排序
    pub is_enable: Option<i64>,     // 是否启用
    pub menu_type: Option<String>,  // 菜单类型：M-目录，C-菜单，F-按钮
    pub keep_alive: Option<i8>,     // 是否缓存
    pub is_hide: Option<i64>,       // 是否隐藏
    pub link: Option<String>,       // 外链接
    pub is_iframe: Option<i64>,     // 是否内嵌
    pub parent_id: Option<i64>,     // 父级ID
    pub permission: Option<String>, // 权限标识
    pub remark: Option<String>,     // 备注
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateMenuDTO {
    pub menu_id: i64,          // 菜单ID
    pub title: Option<String>, // 菜单名称
    pub path: Option<String>,  // 路由地址
    // pub route_name: Option<String>,    // 路由名称
    pub icon: Option<String>,      // 图标
    pub order_num: Option<i64>,    // 排序
    pub is_enable: Option<i64>,    // 是否启用
    pub menu_type: Option<String>, // 菜单类型：M-目录，C-菜单，F-按钮
    pub keep_alive: Option<i8>,    // 是否缓存
    pub is_hide: Option<i8>,      // 是否隐藏
    pub link: Option<String>,      // 外链接
    pub is_iframe: Option<i8>,    // 是否内嵌
    pub parent_id: Option<i64>,    // 父级ID
    pub permission: Option<String>, // 权限标识
    // pub remark: Option<String>,        // 备注
}
