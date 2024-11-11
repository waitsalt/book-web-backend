use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Chapter {
    pub chapter_id: i32,         // 章节编号
    pub chapter_name: String,    // 章节名字
    pub author_id: i32,          // 作者编号
    pub author_name: String,     // 作者名字
    pub roll_id: i32,            // 书籍卷编号 默认为: 0
    pub roll_name: String,       // 书籍卷名 默认为: 正文
    pub book_id: i32,            // 书籍编号
    pub book_name: String,       // 书名
    pub chapter_content: String, // 章节内容
    pub create_time: String,     // 图书创建时间
    pub update_time: String,     // 图书更新时间
}
