use crate::domain::dto::menu_dto::CreateMenuDTO;
use crate::domain::dto::menu_dto::UpdateMenuDTO;
use crate::domain::entity::sys_menu::SysMenu;
use crate::domain::entity::sys_role_menu::SysRoleMenu;
use crate::domain::entity::sys_user::SysUser;
use crate::domain::vo::menu_vo::{MenuRoleVO, MenuVO};
use crate::pool;
use rbatis::rbatis_codegen::ops::AsProxy;
use rbatis::rbdc::DateTime;
use rbatis::Error;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SysMenuService;

impl SysMenuService {
    pub async fn get_menu_by_user_id(&self, user_id: i64) -> Result<Vec<MenuVO>, Error> {
        let menus = SysMenu::get_menu_by_user_id(pool!(), user_id).await?;

        Ok(MenuVO::build_menu_tree(&menus))
    }
    pub async fn get_menu_by_role_id(&self, role_id: i64) -> Result<Vec<MenuVO>, Error> {
        let menus = SysMenu::get_menu_by_role_id(pool!(), role_id).await?;
        Ok(MenuVO::build_menu_tree(&menus))
    }

    // 获取角色的菜单权限(包含所有菜单和已选中的菜单ID)
    pub async fn get_role_menu_permissions(
        &self,
        role_id: i64,
    ) -> Result<(Vec<MenuRoleVO>, Vec<i64>), Error> {
        let rb = pool!();

        // 1. 获取所有菜单
        let all_menus = SysMenu::get_all_menus(rb).await?;

        // 2. 获取角色已分配的菜单ID
        let role_menu_ids = SysMenu::get_role_menu_ids(rb, role_id).await?;

        // 3. 构建权限菜单树
        let menu_tree = MenuVO::build_permission_menu_tree(&all_menus);

        Ok((menu_tree, role_menu_ids))
    }

    // 更新角色菜单权限
    pub async fn update_role_menu_permissions(
        &self,
        role_id: i64,
        menu_ids: Vec<i64>,
    ) -> Result<(), Error> {
        let rb = pool!();

        // 开启事务
        let executor = rb.acquire_begin().await?;

        // 1. 删除原有的角色-菜单关联
        // SysRoleMenu::delete_by_role_id(&executor, role_id).await?;

        // 1. 删除原有的角色-菜单关联
        executor
            .exec(
                "DELETE FROM sys_role_menu WHERE role_id = ?",
                vec![rbs::to_value!(role_id)],
            )
            .await?;
        // 2. 插入新的角色-菜单关联
        if !menu_ids.is_empty() {
            let mut values = Vec::new();
            let mut params = Vec::new();

            for menu_id in menu_ids {
                values.push("(?, ?)");
                params.push(rbs::to_value!(role_id));
                params.push(rbs::to_value!(menu_id));
            }

            let sql = format!(
                "INSERT INTO sys_role_menu (role_id, menu_id) VALUES {}",
                values.join(",")
            );
            executor.exec(&sql, params).await?;
        }
        // 提交事务
        executor.commit().await?;
        Ok(())
    }

    pub async fn create_menu(&self, menu_dto: CreateMenuDTO, user_id: i64) -> Result<(), Error> {
        let rb = pool!();

        let tx = rb.acquire_begin().await?;
        // 构造菜单实体
        let menu = SysMenu {
            menu_id: None,
            menu_name: menu_dto.title,
            title_en: None,
            parent_id: Some(menu_dto.parent_id.unwrap_or(0)), // 如果parent_id为空，默认为0
            order_num: menu_dto.order_num,
            path: menu_dto.path,
            route_name: menu_dto.route_name,
            keep_alive: menu_dto.keep_alive,
            is_frame: None, // 默认值
            menu_type: Some(menu_dto.menu_type.unwrap_or_else(|| "M".to_string())), // 直接使用前端传来的menu_type
            is_hide: menu_dto.is_hide.map(|v| v as i8),
            icon: menu_dto.icon,
            remark: menu_dto.remark, // 默认值
            show_badge: None,        // 默认值
            show_text_badge: None,   // 默认值
            is_hide_tab: None,       // 默认值
            link: menu_dto.link,
            create_by: None, // 默认值
            create_time: Some(DateTime::now()),
            update_by: None, // 默认值
            update_time: Some(DateTime::now()),
            permission: menu_dto.permission, // 权限标识
            is_enable: menu_dto.is_enable,
        };

        // 调用实体的创建方法
        let result = SysMenu::insert(&tx.rb, &menu).await?;
        let menu_id = result.last_insert_id;
        let menu_id = menu_id.i64();

        // 这里判断用户是不是admin，如果是admin，就往sys_role_menu插入一条记录
        let user_db = SysUser::select_by_id(&tx.rb, user_id)
            .await
            .expect("查询用户失败")
            .unwrap();
        if user_db.username == Some("admin".to_string()) {
            let role_menu = SysRoleMenu {
                role_id: Some(1),
                menu_id: Some(menu_id),
            };
            SysRoleMenu::insert(&tx.rb, &role_menu).await?;
        }
        tx.commit().await.expect("提交事务失败");
        Ok(())
    }
    pub async fn update_menu(&self, menu_dto: UpdateMenuDTO, user_id: i64) -> Result<(), Error> {
        let rb = pool!();

        let tx = rb.acquire_begin().await?;

        // 1. 检查菜单是否存在
        let exists = SysMenu::select_by_id(&tx.rb, menu_dto.menu_id)
            .await
            .map_err(|_| Error::from("菜单不存在"))
            .expect("菜单不存在")
            .unwrap();

        // 2. 构造更新的菜单实体
        let menu = SysMenu {
            menu_id: Some(menu_dto.menu_id),
            menu_name: menu_dto.title,
            title_en: None,
            parent_id: Some(menu_dto.parent_id.unwrap_or(0)),
            order_num: menu_dto.order_num,
            path: menu_dto.path,
            route_name: None,
            // route_name: menu_dto.route_name,
            keep_alive: menu_dto.keep_alive,
            is_frame: menu_dto.is_iframe,
            menu_type: Some(menu_dto.menu_type.unwrap_or_else(|| "M".to_string())),
            is_hide: menu_dto.is_hide,
            icon: menu_dto.icon,
            // remark: menu_dto.remark,
            remark: None,
            show_badge: None,
            show_text_badge: None,
            is_hide_tab: None,
            link: menu_dto.link,
            create_by: exists.create_by,          // 保持原有值
            create_time: exists.create_time,      // 保持原有值
            update_by: Some(user_id.to_string()), // 更新修改人
            update_time: Some(DateTime::now()),   // 更新修改时间
            permission: menu_dto.permission,
            is_enable: menu_dto.is_enable,
        };

        // 3. 更新菜单
        SysMenu::update_by_column(&tx.rb, &menu, "menu_id").await?;

        // 4. 如果是admin用户，确保admin角色有此菜单权限
        let user_db = SysUser::select_by_id(&tx.rb, user_id)
            .await?
            .ok_or_else(|| Error::from("用户不存在"))?;

        if user_db.username == Some("admin".to_string()) {
            // 检查admin角色是否已有此菜单权限
            let exists =
                SysRoleMenu::select_by_menu_id_and_role_id(&tx.rb, 1, menu.menu_id.unwrap())
                    .await
                    .expect("查询角色菜单失败")
                    .unwrap();

            if exists.menu_id.is_none() {
                // 如果没有，则添加
                let role_menu = SysRoleMenu {
                    role_id: Some(1),
                    menu_id: Some(menu_dto.menu_id),
                };
                SysRoleMenu::insert(&tx.rb, &role_menu).await?;
            }
        }

        tx.commit().await.expect("提交事务失败");
        Ok(())
    }

    // 删除菜单
    pub async fn delete_menu(&self, menu_id: i64) -> Result<(), Error> {
        let rb = pool!();
        // 开启事务
        let mut tx = rb.acquire_begin().await?;

        // 1. 检查是否有子菜单
        let children = SysMenu::select_by_parent_id(&tx.rb, menu_id).await?;

        if !children.is_empty() {
            return Err(Error::from("该菜单下有子菜单，不能删除"));
        }

        // 2. 删除角色-菜单关联
        SysRoleMenu::delete_by_menu_id(&tx.rb, menu_id).await?;

        // 3. 删除菜单
        SysMenu::delete_by_menu_id(&tx.rb, menu_id).await?;

        // 提交事务
        tx.commit().await?;

        Ok(())
    }
}
