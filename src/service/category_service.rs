use crate::domain::dto::category_dto::{CreateCategoryDTO, UpdateCategoryDTO};
use crate::domain::entity::article_categories::ArticleCategories;
use crate::pool;
use rbatis::rbdc::DateTime;
use rbatis::Error;
use rbatis::rbatis_codegen::ops::AsProxy;

#[derive(Debug, Clone)]
pub struct CategoryService;

impl CategoryService {
    // 创建分类
    pub async fn create_category(&self, dto: CreateCategoryDTO) -> Result<i64, Error> {
        let category = ArticleCategories {
            id: None,
            name: Some(dto.name),
            icon: dto.icon_class,
            count: Some(0),
            created_time: Some(DateTime::now()),
        };

        // 检查分类名是否已存在
        let exists = ArticleCategories::select_by_column(pool!(), "name", &category.name)
            .await?
            .len() > 0;
        if exists {
            return Err(Error::from("分类名已存在"));
        }

        // 插入分类
        let result = ArticleCategories::insert(pool!(), &category).await?;
        Ok(result.last_insert_id.i64())
    }

    // 更新分类
    pub async fn update_category(&self, dto: UpdateCategoryDTO) -> Result<(), Error> {
        // 检查分类是否存在
        let mut category = ArticleCategories::select_by_id(pool!(), dto.id)
            .await?
            .ok_or_else(|| Error::from("分类不存在"))?;

        // 检查新名称是否与其他分类重复
        let exists = ArticleCategories::select_by_column(pool!(), "name", &dto.name)
            .await?
            .iter()
            .any(|c| c.id != Some(dto.id));
        if exists {
            return Err(Error::from("分类名已存在"));
        }

        // 更新分类信息
        category.name = Some(dto.name);
        category.icon = dto.icon_class;

        ArticleCategories::update_by_column(pool!(), &category, "id").await?;
        Ok(())
    }

    // 删除分类
    pub async fn delete_category(&self, id: i64) -> Result<(), Error> {
        // 检查分类是否存在
        let exists = ArticleCategories::select_by_id(pool!(), id)
            .await?
            .is_some();
        if !exists {
            return Err(Error::from("分类不存在"));
        }

        // TODO: 检查分类下是否有文章

        ArticleCategories::delete_by_column(pool!(), "id", id).await?;
        Ok(())
    }

    // 获取分类列表
    pub async fn get_categories(&self) -> Result<Vec<ArticleCategories>, Error> {
        ArticleCategories::select_all(pool!()).await
    }

    // 获取单个分类
    pub async fn get_category(&self, id: i64) -> Result<Option<ArticleCategories>, Error> {
        ArticleCategories::select_by_id(pool!(), id).await
    }
} 