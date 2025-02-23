use rbatis::rbdc::DateTime;
use rbatis::{crud, impl_select};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ArticleCategories {
    pub id: Option<i64>,                // 分类ID
    pub name: Option<String>,           // 分类名称
    pub icon: Option<String>,           // 分类图标路径
    pub count: Option<i32>,             // 文章数量统计
    pub created_time: Option<DateTime>, // 创建时间
}

// 基础 CRUD 实现
crud!(ArticleCategories {}, "article_categories");
impl_select!(ArticleCategories {
    select_by_id(id: i64) -> Option => "`where id = #{id}`"
});

impl ArticleCategories {}
