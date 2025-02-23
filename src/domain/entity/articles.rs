use chrono::Utc;
use rbatis::rbdc::DateTime;
use rbatis::{crud, impl_select, Error, RBatis};
use rbs::Value;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Articles {
    pub id: Option<i64>,                // 文章ID
    pub category_id: Option<i64>,       // 分类ID
    pub title: Option<String>,          // 文章标题
    pub html_content: Option<String>,   // HTML内容
    pub home_img: Option<String>,       // 封面图路径
    pub brief: Option<String>,          // 文章简介
    pub view_count: Option<i32>,        // 浏览量
    pub created_time: Option<DateTime>, // 创建时间
    pub updated_time: Option<DateTime>, // 更新时间
    /// 是否可见（1=可见, 0=不可见）
    pub is_visible: Option<i64>,

    /// 是否置顶（1=置顶, 0=不置顶）
    pub is_top: Option<i64>,
}

// 添加 ArticleVO 结构体，用于返回给前端的数据
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ArticleVO {
    pub id: Option<i64>,
    pub category_id: Option<i64>,
    pub title: Option<String>,
    pub html_content: Option<String>,
    pub home_img: Option<String>,
    pub brief: Option<String>,
    pub view_count: Option<i32>,
    pub created_time: Option<DateTime>,
    pub updated_time: Option<DateTime>,
    pub type_name: Option<String>, // 额外添加分类名称字段
}

// 用于计数的结构体
#[derive(Debug, Serialize, Deserialize)]
struct Count {
    count: u64,
}

// 基础 CRUD 实现
crud!(Articles {}, "articles");
impl_select!(Articles {
    select_by_id(id: i64) -> Option => "`where id = #{id}`"
});

impl Articles {
    // 分页查询文章列表
    pub async fn select_page(
        rb: &RBatis,
        page: u64,
        size: u64,
        category_id: Option<i64>,
        title: Option<String>,
        year: Option<String>,
        only_visible: bool,
    ) -> Result<(Vec<ArticleVO>, u64), Error> {
        let mut sql = String::from(
            "SELECT a.*, c.name as type_name 
             FROM articles a 
             LEFT JOIN article_categories c ON a.category_id = c.id 
             WHERE 1=1"
        );
        let mut params: Vec<Value> = Vec::new();

        // 添加查询条件
        if let Some(cid) = category_id {
            sql.push_str(" AND a.category_id = ?");
            params.push(rbs::to_value!(cid));
        }

        if let Some(t) = title {
            sql.push_str(" AND a.title LIKE ?");
            params.push(rbs::to_value!(format!("%{}%", t)));
        }

        if let Some(y) = year {
            if y != "All" {
                sql.push_str(" AND YEAR(a.created_time) = ?");
                params.push(rbs::to_value!(y));
            }
        }

        // 只查询可见的文章
        if only_visible {
            sql.push_str(" AND a.is_visible = 1");
        }

        // 排序规则: 先按置顶降序,再按更新时间降序
        sql.push_str(" ORDER BY a.is_top DESC, a.updated_time DESC, a.created_time DESC");

        // 添加分页
        let offset = (page - 1) * size;
        sql.push_str(" LIMIT ? OFFSET ?");
        params.push(rbs::to_value!(size));
        params.push(rbs::to_value!(offset));

        // 执行查询
        let articles: Vec<ArticleVO> = rb.query_decode(&sql, params.clone()).await?;

        // 获取总数
        let count_sql = format!(
            "SELECT COUNT(*) as count FROM articles a WHERE 1=1{}",
            if only_visible { " AND is_visible = 1" } else { "" }
        );
        let counts: Vec<Count> = rb.query_decode(&count_sql, params).await?;
        let total = counts.first().map(|c| c.count).unwrap_or(0);

        Ok((articles, total))
    }
}
