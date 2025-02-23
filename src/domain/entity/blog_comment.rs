use rbatis::crud;
use rbatis::rbdc::DateTime;
use serde::{Serialize, Deserialize};
use rbatis::Error;
use rbatis::RBatis;
use crate::domain::vo::comment_vo::CommentWithUser;

#[derive(Debug, Serialize, Deserialize)]
struct Count {
    count: u64
}

/// 博客评论实体
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BlogComment {
    /// 评论ID (primary key, auto increment)
    pub comment_id: Option<i64>,

    /// 关联文章ID（逻辑外键->blog_post.post_id）
    pub post_id: i64,

    /// 评论用户ID（逻辑外键->sys_user.user_id）
    pub user_id: i64,

    /// 评论内容 (max length: 65535)
    pub content: String,

    /// 父评论ID（0表示顶级评论）
    pub parent_id: i64,

    /// 创建时间 (default: CURRENT_TIMESTAMP)
    pub created_at: Option<DateTime>,

    /// 评论层级（从1开始）
    pub level: i16,

    /// 层级路径（格式：0/12/34/）
    pub path: String,
}

// 生成基础CRUD操作
crud!(BlogComment {},"blog_comments");

impl BlogComment {
    // 根据ID查询评论
    pub async fn select_by_id(rb: &RBatis, id: i64) -> Result<Option<BlogComment>, Error> {
        rb.query_decode::<Option<BlogComment>>(
            "SELECT * FROM blog_comments WHERE comment_id = ?",
            vec![rbs::to_value!(id)]
        ).await
    }

    // 分页查询评论
    pub async fn select_page(
        rb: &RBatis,
        page: u64,
        size: u64,
        post_id: i64,
    ) -> Result<(Vec<BlogComment>, u64), Error> {
        let offset = (page - 1) * size;
        
        // 查询评论列表
        let comments: Vec<BlogComment> = rb.query_decode::<Vec<BlogComment>>(
            "SELECT * FROM blog_comments WHERE post_id = ? ORDER BY created_at DESC LIMIT ? OFFSET ?",
            vec![
                rbs::to_value!(post_id),
                rbs::to_value!(size),
                rbs::to_value!(offset),
            ],
        ).await?;

        // 查询总数
        let count: Option<Count> = rb.query_decode::<Option<Count>>(
            "SELECT COUNT(*) as count FROM blog_comments WHERE post_id = ?",
            vec![rbs::to_value!(post_id)],
        ).await?;

        let total = count.map(|c| c.count).unwrap_or(0);

        Ok((comments, total))
    }

    // 根据路径删除评论及其子评论
    pub async fn delete_by_path(rb: &RBatis, path: &str) -> Result<(), Error> {
        rb.exec(
            "DELETE FROM blog_comments WHERE path LIKE ?",
            vec![rbs::to_value!(path)],
        ).await?;
        Ok(())
    }

    // 获取文章的评论树
    pub async fn select_comment_tree(rb: &RBatis, post_id: i64) -> Result<Vec<CommentWithUser>, Error> {
        rb.query_decode(
            r#"
            SELECT 
                c.*,
                u.username,
                u.nickname,
                u.avatar
            FROM blog_comments c
            LEFT JOIN sys_user u ON c.user_id = u.user_id
            WHERE c.post_id = ?
            ORDER BY c.path ASC, c.created_at ASC
            "#,
            vec![rbs::to_value!(post_id)]
        ).await
    }

    // 检查是否有子评论
    pub async fn has_replies(rb: &RBatis, comment_id: i64) -> Result<bool, Error> {
        let sql = "SELECT COUNT(*) as count FROM blog_comments WHERE parent_id = ?";
        let count: Vec<Count> = rb.query_decode(sql, vec![rbs::to_value!(comment_id)]).await?;
        Ok(count.first().map(|c| c.count > 0).unwrap_or(false))
    }

    // 删除评论
    pub async fn delete_comment(rb: &RBatis, comment_id: i64) -> Result<(), Error> {
        // 检查是否有子评论
        if BlogComment::has_replies(rb, comment_id).await? {
            return Err(Error::from("该评论下有回复，不能删除"));
        }

        // 删除评论
        BlogComment::delete_by_column(rb, "comment_id", comment_id).await?;
        Ok(())
    }
}
