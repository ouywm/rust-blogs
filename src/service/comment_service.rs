use crate::domain::dto::comment_dto::{CreateCommentDTO, UpdateCommentDTO, CommentQueryDTO};
use crate::domain::entity::blog_comment::BlogComment;
use crate::pool;
use rbatis::rbdc::DateTime;
use rbatis::Error;
use rbatis::rbatis_codegen::ops::AsProxy;
use crate::domain::vo::comment_vo::{CommentVO, CommentWithUser};

#[derive(Debug, Clone)]
pub struct CommentService;

impl CommentService {
    // 创建评论
    pub async fn create_comment(&self, user_id: i64, dto: CreateCommentDTO) -> Result<i64, Error> {
        // 获取评论层级和路径
        let (level, path) = if let Some(parent_id) = dto.parent_id {
            let parent = BlogComment::select_by_id(pool!(), parent_id)
                .await?
                .ok_or_else(|| Error::from("父评论不存在"))?;

            // 直接在父评论层级基础上加1
            (parent.level + 1, format!("{}{}/", parent.path, parent_id))
        } else {
            (1, String::from("0/"))
        };

        let comment = BlogComment {
            comment_id: None,
            post_id: dto.post_id,
            user_id,
            content: dto.content,
            parent_id: dto.parent_id.unwrap_or(0),
            created_at: Some(DateTime::now()),
            level,
            path,
        };

        let result = BlogComment::insert(pool!(), &comment).await?;
        Ok(result.last_insert_id.i64())
    }

    // 更新评论
    pub async fn update_comment(&self, user_id: i64, dto: UpdateCommentDTO) -> Result<(), Error> {
        let mut comment = BlogComment::select_by_id(pool!(), dto.comment_id)
            .await?
            .ok_or_else(|| Error::from("评论不存在"))?;

        // 验证是否是评论作者
        if comment.user_id != user_id {
            return Err(Error::from("无权修改他人评论"));
        }

        comment.content = dto.content;
        BlogComment::update_by_column(pool!(), &comment, "comment_id").await?;
        Ok(())
    }

    // 删除评论
    pub async fn delete_comment(&self, comment_id: i64) -> Result<(), Error> {
        // 检查评论是否存在
        let exists = BlogComment::select_by_id(pool!(), comment_id)
            .await?
            .is_some();
        if !exists {
            return Err(Error::from("评论不存在"));
        }

        // 删除评论
        BlogComment::delete_comment(pool!(), comment_id).await
    }

    // 获取文章评论列表
    pub async fn get_comments(&self, query: CommentQueryDTO) -> Result<(Vec<BlogComment>, u64), Error> {
        BlogComment::select_page(
            pool!(),
            query.page.unwrap_or(1),
            query.size.unwrap_or(10),
            query.post_id,
        ).await
    }

    // 获取文章评论树
    pub async fn get_comment_tree(&self, post_id: i64) -> Result<Vec<CommentVO>, Error> {
        let comments = BlogComment::select_comment_tree(pool!(), post_id).await?;
        Ok(CommentVO::build_tree(comments))
    }
}