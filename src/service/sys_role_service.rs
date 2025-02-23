use crate::domain::dto::role_dto::{CreateRoleDTO, UpdateRoleDTO};
use crate::domain::entity::sys_role::SysRole;
use crate::domain::entity::sys_role_menu::SysRoleMenu;
use crate::pool;
use rbatis::rbdc::datetime::DateTime;
use rbatis::rbdc::db::ExecResult;
use rbatis::Error;
use rbs::Value;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SysRoleService;

impl SysRoleService {
    // 创建角色
    pub async fn create_role(&self, dto: CreateRoleDTO) -> Result<i64, Error> {
        // 检查角色编码是否存在
        let exists = SysRole::select_by_role_code(pool!(), &dto.role_code)
            .await?
            .is_some();
        if exists {
            return Err(Error::from("角色编码已存在"));
        }

        let role = SysRole {
            role_id: None,
            role_code: Some(dto.role_code),
            role_name: Some(dto.role_name),
            role_desc: dto.role_desc,
            order_num: dto.order_num,
            status: Some(dto.status.unwrap_or(1)),
            create_time: Some(DateTime::now()),
            update_time: Some(DateTime::now()),
            permissions: vec![],
        };

        let result = SysRole::insert(pool!(), &role).await?;
        let id = match result.last_insert_id {
            Value::I64(id) => id,
            _ => return Err(Error::from("插入角色失败：无法获取插入ID")),
        };

        Ok(id)
    }


    // 更新角色信息
    pub async fn update_role(&self, dto: UpdateRoleDTO) -> Result<ExecResult, Error> {
        let mut role = SysRole::select_by_id(pool!(), dto.role_id)
            .await?
            .ok_or_else(|| Error::from("角色不存在"))?;

        // 更新字段
        role.role_name = Some(dto.role_name);
        role.role_desc = dto.role_desc;
        role.order_num = dto.order_num;
        role.status = dto.status;
        role.update_time = Some(DateTime::now());

        SysRole::update_by_column(pool!(), &role, "role_id").await
    }

    // 删除角色
    pub async fn delete_role(&self, role_id: i64) -> Result<ExecResult, Error> {
        let exists = SysRole::select_by_id(pool!(), role_id).await?.is_some();
        if !exists {
            return Err(Error::from("角色不存在"));
        }

        SysRole::delete_by_column(pool!(), "role_id", role_id).await
    }

    // 根据角色ID获取角色信息
    pub async fn get_role(&self, role_id: i64) -> Result<Option<SysRole>, Error> {
        SysRole::select_by_id(pool!(), role_id).await
    }

    //分页查询角色列表
    pub async fn list_roles_by_page(
        &self,
        page: u64,
        size: u64,
        role_name: Option<String>,
    ) -> Result<(Vec<SysRole>, u64), Error> {
        let (roles, total) =
            SysRole::select_role_page_by_name(pool!(), page, size, role_name).await?;
        Ok((roles, total))
    }

    // 获取角色的权限ID列表
    pub async fn get_role_permissions(&self, role_id: i64) -> Result<Vec<i64>, Error> {
        // 检查角色是否存在
        let exists = SysRole::select_by_id(pool!(), role_id).await?.is_some();
        if !exists {
            return Err(Error::from("角色不存在"));
        }
        SysRoleMenu::select_perm_ids_by_role(pool!(), role_id).await
    }
}
