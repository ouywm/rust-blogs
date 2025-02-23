use crate::domain::dto::article_dto::{ArticleQueryDTO, CreateArticleDTO, UpdateArticleDTO};
use crate::domain::entity::articles::{ArticleVO, Articles};
use crate::pool;
use rbatis::rbatis_codegen::ops::AsProxy;
use rbatis::rbdc::DateTime;
use rbatis::Error;

#[derive(Debug, Clone)]
pub struct ArticleService;

impl ArticleService {
    // 创建文章
    pub async fn create_article(&self, dto: CreateArticleDTO) -> Result<i64, Error> {
        let article = Articles {
            id: None,
            category_id: Some(dto.category_id),
            title: Some(dto.title),
            html_content: Some(dto.html_content),
            home_img: dto.home_img,
            brief: dto.brief,
            view_count: Some(0),
            created_time: Some(DateTime::now()),
            updated_time: Some(DateTime::now()),
            is_visible: Some(if dto.is_visible.unwrap_or(true) { 1 } else { 0 }),
            is_top: Some(if dto.is_top.unwrap_or(false) { 1 } else { 0 }),
        };

        let result = Articles::insert(pool!(), &article).await?;
        Ok(result.last_insert_id.i64())
    }

    // 更新文章
    pub async fn update_article(&self, dto: UpdateArticleDTO) -> Result<(), Error> {
        let mut article = Articles::select_by_id(pool!(), dto.id)
            .await?
            .ok_or_else(|| Error::from("文章不存在"))?;

        article.category_id = Some(dto.category_id);
        article.title = Some(dto.title);
        article.html_content = Some(dto.html_content);
        article.home_img = dto.home_img;
        article.brief = dto.brief;
        article.updated_time = Some(DateTime::now());
        
        // 更新可见性和置顶状态
        if let Some(is_visible) = dto.is_visible {
            article.is_visible = Some(if is_visible { 1 } else { 0 });
        }
        if let Some(is_top) = dto.is_top {
            article.is_top = Some(if is_top { 1 } else { 0 });
        }

        Articles::update_by_column(pool!(), &article, "id").await?;
        Ok(())
    }

    // 删除文章
    pub async fn delete_article(&self, id: i64) -> Result<(), Error> {
        let exists = Articles::select_by_id(pool!(), id).await?.is_some();
        if !exists {
            return Err(Error::from("文章不存在"));
        }

        Articles::delete_by_column(pool!(), "id", id).await?;
        Ok(())
    }

    // 获取文章列表(带分页和条件查询)
    pub async fn get_articles(
        &self,
        query: ArticleQueryDTO,
    ) -> Result<(Vec<ArticleVO>, u64), Error> {
        Articles::select_page(
            pool!(),
            query.page.unwrap_or(1),
            query.size.unwrap_or(10),
            query.category_id,
            query.title,
            query.year,
            true,
        )
        .await
    }

    // 获取单个文章
    pub async fn get_article(&self, id: i64) -> Result<Option<Articles>, Error> {
        Articles::select_by_id(pool!(), id).await
    }

    // 增加浏览量
    pub async fn increment_view_count(&self, id: i64) -> Result<(), Error> {
        let mut article = Articles::select_by_id(pool!(), id)
            .await?
            .ok_or_else(|| Error::from("文章不存在"))?;

        article.view_count = Some(article.view_count.unwrap_or(0) + 1);
        Articles::update_by_column(pool!(), &article, "id").await?;
        Ok(())
    }
}
