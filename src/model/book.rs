use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Book {
    pub book_id: i32,             // 图书唯一编号
    pub book_name: String,        // 书名
    pub author_name: String,      // 作者名字
    pub user_id: i32,             // 上传者编号
    pub user_name: String,        // 上传者名字
    pub book_picture_url: String, // 图书封面链接
    pub book_class: String,       // 图书类别
    pub create_time: String,      // 图书创建时间
    pub update_time: String,      // 图书更新时间
    pub book_status: String,      // 图书状态: 0: 连载 1: 完结 2: 太监
    pub book_tags: String,        // 图书标签
    pub book_desc: String,        // 图书描述
    pub lastest_chapter: String,  // 最新章节
                                  // pub score: String,            // 评分
                                  // pub collect: i32,             // 收藏数
}
