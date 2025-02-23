use serde::{Deserialize, Serialize};
use rbatis::rbdc::DateTime;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommentVO {
    pub id: i64,
    pub author: String,  // 评论作者名称
    pub content: String, // 评论内容
    pub timestamp: String, // 评论时间
    pub replies: Vec<CommentVO>, // 子评论
}

impl CommentVO {
    // 构建评论树
    pub fn build_tree(comments: Vec<CommentWithUser>) -> Vec<CommentVO> {
        let mut result = Vec::new();
        let mut map = HashMap::new();

        // 按 level 排序,确保父评论在前
        let mut sorted_comments = comments;
        sorted_comments.sort_by_key(|c| c.level);

        // 先将所有评论转换为 VO 并建立映射
        for comment in &sorted_comments {
            let comment_vo = CommentVO {
                id: comment.comment_id.unwrap_or_default(),
                author: comment.username.clone().unwrap_or_default(),
                content: comment.content.clone(),
                timestamp: comment.created_at.clone()
                    .map(|t| t.to_string())
                    .unwrap_or_default(),
                replies: Vec::new(),
            };

            map.insert(comment.comment_id.unwrap_or_default(), comment_vo);
        }

        // 构建评论树
        for comment in sorted_comments {
            let comment_id = comment.comment_id.unwrap_or_default();
            let parent_id = comment.parent_id;

            if parent_id == 0 {
                // 顶级评论
                if let Some(comment) = map.remove(&comment_id) {
                    result.push(comment);
                }
            } else {
                // 获取当前评论
                if let Some(current) = map.remove(&comment_id) {
                    // 根据 path 找到父评论
                    let path_parts: Vec<&str> = comment.path.split('/').collect();
                    if path_parts.len() >= 2 {
                        let parent_id = path_parts[path_parts.len() - 2].parse::<i64>().unwrap_or(0);
                        
                        // 先在 map 中查找父评论
                        if let Some(parent) = map.get_mut(&parent_id) {
                            parent.replies.push(current);
                        } else {
                            // 在结果集中递归查找父评论
                            fn find_parent_and_add_reply(comments: &mut Vec<CommentVO>, parent_id: i64, reply: CommentVO) {
                                for comment in comments.iter_mut() {
                                    if comment.id == parent_id {
                                        comment.replies.push(reply);
                                        return;
                                    }
                                    find_parent_and_add_reply(&mut comment.replies, parent_id, reply.clone());
                                }
                            }
                            find_parent_and_add_reply(&mut result, parent_id, current);
                        }
                    }
                }
            }
        }

        result
    }
}

// 包含用户信息的评论
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommentWithUser {
    // 评论信息
    pub comment_id: Option<i64>,
    pub post_id: i64,
    pub user_id: i64,
    pub content: String,
    pub parent_id: i64,
    pub created_at: Option<DateTime>,
    pub level: i16,
    pub path: String,
    
    // 用户信息
    pub username: Option<String>,
    pub nickname: Option<String>,
    pub avatar: Option<String>,
}