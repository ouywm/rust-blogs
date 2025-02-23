use crate::domain::entity::sys_menu::SysMenu;
use rbatis::rbdc::DateTime;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct MenuVO {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,

    #[serde(skip_serializing_if = "is_empty_str", rename = "name")]
    pub name: Option<String>,

    #[serde(rename = "meta")]
    pub meta: MetaVO,

    #[serde(skip_serializing_if = "Vec::is_empty", rename = "children")]
    pub children: Vec<MenuVO>,

    pub update_time: Option<DateTime>, // 创建时间

    pub permission: Option<String>, // 权限

    pub sort: Option<i64>, // 排序
}
#[derive(Debug, Serialize, Deserialize)]
pub struct MenuRoleVO {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub menu_id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,

    #[serde(skip_serializing_if = "is_empty_str", rename = "name")]
    pub name: Option<String>,

    #[serde(rename = "meta")]
    pub meta: MetaRoleVO,

    #[serde(skip_serializing_if = "Vec::is_empty", rename = "children")]
    pub children: Vec<MenuRoleVO>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct MetaVO {
    #[serde(rename = "title")]
    pub title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "title_en")]
    pub title_en: Option<String>,

    // #[serde(skip_serializing_if = "should_skip", rename = "icon")]
    pub icon: Option<String>,

    #[serde(
        skip_serializing_if = "should_skip_show_badge", // 新增判断逻辑
        rename = "showBadge"
    )]
    pub show_badge: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "showTextBadge")]
    pub show_text_badge: Option<String>,

    #[serde(
        rename = "isHide",
        // skip_serializing_if = "is_false_or_none",
        // deserialize_with = "deserialize_bool_int"
    )]
    pub is_hide: Option<bool>,

    #[serde(
        rename = "isHideTab",
        // skip_serializing_if = "is_false_or_none",
        // deserialize_with = "deserialize_bool_int"
    )]
    pub is_hide_tab: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "link")]
    pub link: Option<String>,

    #[serde(
        rename = "isIframe",
        // skip_serializing_if = "is_false_or_none",
        // deserialize_with = "deserialize_bool_int"
    )]
    pub is_iframe: Option<bool>,

    #[serde(rename = "keepAlive")]
    pub keep_alive: Option<bool>,
    #[serde(rename = "isEnable")]
    pub is_enable: Option<bool>, // 是否启用: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "authList")]
    pub auth_list: Option<Vec<AuthItem>>,
}

// 新增自定义判断方法
fn should_skip(value: &Option<String>) -> bool {
    match value {
        // 同时满足非None和非空字符串时才序列化
        Some(s) => match s {
            &_ => {
                if s.trim().is_empty() {
                    true
                } else {
                    false
                }
            }
        },
        None => true,
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MetaRoleVO {
    #[serde(rename = "title")]
    pub title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "titleEn")]
    pub title_en: Option<String>,

    #[serde(skip_serializing_if = "should_skip", rename = "icon")]
    pub icon: Option<String>,

    #[serde(
        skip_serializing_if = "should_skip_show_badge", // 新增判断逻辑
        rename = "showBadge"
    )]
    pub show_badge: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "showTextBadge")]
    pub show_text_badge: Option<String>,

    #[serde(
        rename = "isHide",
        skip_serializing_if = "is_false_or_none",
        deserialize_with = "deserialize_bool_int"
    )]
    pub is_hide: Option<bool>,

    #[serde(
        rename = "isHideTab",
        skip_serializing_if = "is_false_or_none",
        deserialize_with = "deserialize_bool_int"
    )]
    pub is_hide_tab: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "link")]
    pub link: Option<String>,

    #[serde(
        rename = "is_iframe",
        skip_serializing_if = "is_false_or_none",
        deserialize_with = "deserialize_bool_int"
    )]
    pub is_iframe: Option<bool>,

    #[serde(rename = "keepAlive")]
    pub keep_alive: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "authList")]
    pub auth_list: Option<Vec<AuthItem>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "menuType")]
    pub menu_type: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthItem {
    pub id: i64,
    pub title: String,
    pub auth_mark: String,
}
fn should_skip_show_badge(value: &Option<bool>) -> bool {
    match value {
        Some(false) => true, // 跳过 false
        None => true,        // 跳过 None
        Some(true) => false, // 保留 true
    }
}
// 序列化辅助函数
fn is_empty_str(value: &Option<String>) -> bool {
    value.as_ref().map_or(true, |s| s.trim().is_empty())
}

fn is_false_or_none(value: &Option<bool>) -> bool {
    match value {
        Some(true) => false,
        _ => true,
    }
}

fn deserialize_bool_int<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value: Option<i8> = Option::deserialize(deserializer)?;
    Ok(value.map(|v| v == 1))
}

impl MenuVO {
    pub fn build_menu_tree(menus: &[SysMenu]) -> Vec<MenuVO> {
        // 使用HashMap加速查找
        let mut menu_map: HashMap<i64, Vec<SysMenu>> = HashMap::new();

        // 按parent_id分组
        for menu in menus {
            menu_map
                .entry(menu.parent_id.unwrap_or_default())
                .or_default()
                .push(menu.clone());
        }

        // 从根节点开始构建 (parent_id = 0)
        Self::build_tree(&menu_map, 0)
    }

    fn build_tree(menu_map: &HashMap<i64, Vec<SysMenu>>, parent_id: i64) -> Vec<MenuVO> {
        let Some(children) = menu_map.get(&parent_id) else {
            return vec![];
        };

        let mut sorted_children = children.clone();
        // 按order_num排序
        sorted_children.sort_by_key(|m| m.order_num);

        sorted_children
            .into_iter()
            .filter(|m| {
                m.menu_type == Some("M".to_string()) || m.menu_type == Some("C".to_string())
            }) // 只处理目录和菜单
            .map(|menu| {
                // 递归构建子节点
                let children = Self::build_tree(menu_map, menu.menu_id.unwrap_or_default());

                // 处理权限按钮
                let auth_list = menu_map
                    .get(&menu.menu_id.unwrap_or_default())
                    .map(|items| {
                        items
                            .iter()
                            .filter(|m| m.menu_type == Some("F".to_string()))
                            .map(|btn| AuthItem {
                                id: btn.menu_id.expect("menu_id is None"),
                                title: btn.menu_name.clone().expect("menu_name is None"),
                                auth_mark: btn.permission.clone().expect("permission is None"),
                            })
                            .collect()
                    })
                    .filter(|v: &Vec<AuthItem>| !v.is_empty());

                // 转换MetaVO
                let meta = MetaVO {
                    title: Some(menu.menu_name.clone().unwrap_or_default()),
                    title_en: menu.title_en.clone(),
                    icon: menu.icon.clone(),
                    show_badge: menu.show_badge.map(|v| v == 0),
                    show_text_badge: menu.show_text_badge.clone(),
                    is_hide: menu.is_hide.map(|v| v == 0),
                    is_hide_tab: menu.is_hide_tab.map(|v| v == 0),
                    link: menu.link.clone(), // 根据需求补充
                    is_iframe: menu.is_frame.map(|v| v == 0),
                    keep_alive: menu.keep_alive.map(|v| v == 0),
                    is_enable: menu.is_enable.map(|v| v == 0),
                    auth_list,
                };

                // 构建MenuVO
                MenuVO {
                    id: menu.menu_id,
                    path: menu.path.clone(),
                    name: menu.route_name.clone(),
                    meta,
                    children,
                    update_time: menu.update_time,
                    permission: menu.permission,
                    sort: menu.order_num,
                }
            })
            .collect()
    }

    // 新增一个专门用于角色权限页面的菜单树构建方法
    pub fn build_permission_menu_tree(menus: &[SysMenu]) -> Vec<MenuRoleVO> {
        let mut menu_map: HashMap<i64, Vec<SysMenu>> = HashMap::new();

        // 按parent_id分组
        for menu in menus {
            menu_map
                .entry(menu.parent_id.unwrap_or_default())
                .or_default()
                .push(menu.clone());
        }

        // 从根节点开始构建
        Self::build_permission_tree(&menu_map, 0)
    }

    fn build_permission_tree(
        menu_map: &HashMap<i64, Vec<SysMenu>>,
        parent_id: i64,
    ) -> Vec<MenuRoleVO> {
        menu_map
            .get(&parent_id)
            .map(|children| {
                let mut sorted_children = children.clone();
                sorted_children.sort_by_key(|m| m.order_num);

                sorted_children
                    .into_iter()
                    .filter_map(|menu| {
                        let menu_id = menu.menu_id?;

                        Some(MenuRoleVO {
                            menu_id: Some(menu_id),
                            path: menu.path,
                            name: menu.route_name,
                            meta: MetaRoleVO {
                                title: Some(menu.menu_name.unwrap_or_default()),
                                title_en: menu.title_en,
                                icon: menu.icon,
                                show_badge: menu.show_badge.map(|v| v == 0),
                                show_text_badge: menu.show_text_badge,
                                is_hide: menu.is_hide.map(|v| v == 0),
                                is_hide_tab: menu.is_hide_tab.map(|v| v == 0),
                                link: menu.link,
                                is_iframe: menu.is_frame.map(|v| v == 0),
                                keep_alive: Some(menu.keep_alive.unwrap_or(0) == 0),
                                menu_type: menu.menu_type.clone(),
                                auth_list: None,
                            },
                            children: Self::build_permission_tree(menu_map, menu_id),
                        })
                    })
                    .collect()
            })
            .unwrap_or_default()
    }
}
