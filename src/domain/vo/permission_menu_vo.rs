use crate::domain::entity::sys_menu::SysMenu;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct PermissionMenuVO {
    #[serde(rename = "menu_id")]
    pub menu_id: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "name")]
    pub name: Option<String>,

    pub meta: PermissionMetaVO,

    #[serde(default)]
    pub children: Vec<PermissionMenuVO>,

    #[serde(rename = "type")]
    pub menu_type: String,  // 新增字段，用于区分目录(M)、菜单(C)、按钮(F)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PermissionMetaVO {
    pub title: String,

    #[serde(skip_serializing_if = "Option::is_none", rename = "titleEn")]
    pub title_en: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
}

impl PermissionMenuVO {
    pub fn build_tree(menus: &[SysMenu]) -> Vec<PermissionMenuVO> {
        let mut menu_map: HashMap<i64, Vec<SysMenu>> = HashMap::new();

        for menu in menus {
            menu_map
                .entry(menu.parent_id.unwrap_or_default())
                .or_default()
                .push(menu.clone());
        }

        Self::build_tree_recursive(&menu_map, 0)
    }

    fn build_tree_recursive(menu_map: &HashMap<i64, Vec<SysMenu>>, parent_id: i64) -> Vec<PermissionMenuVO> {
        menu_map.get(&parent_id)
            .map(|children| {
                let mut sorted_children = children.clone();
                sorted_children.sort_by_key(|m| m.order_num);

                sorted_children.into_iter()
                    .filter_map(|menu| {
                        let menu_id = menu.menu_id?;
                        
                        Some(PermissionMenuVO {
                            menu_id,
                            path: menu.path,
                            name: menu.route_name,
                            meta: PermissionMetaVO {
                                title: menu.menu_name.unwrap_or_default(),
                                title_en: menu.title_en,
                                icon: menu.icon,
                            },
                            children: Self::build_tree_recursive(menu_map, menu_id),
                            menu_type: menu.menu_type.unwrap_or_default(),
                        })
                    })
                    .collect()
            })
            .unwrap_or_default()
    }
} 