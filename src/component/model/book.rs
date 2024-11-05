use crate::component::database::book::update_book_info_db;
use crate::util::config::CONFIG;
use crate::util::error::AppError;
use serde::Deserialize;
use serde::Serialize;
use tokio::io::AsyncWriteExt;

#[derive(Debug, Deserialize, Serialize)]
pub struct Book {
    pub id: String,
    pub name: String,
    pub author: String,
    pub status: String,
    pub tag: String,
    pub desc: String,
    pub chapter: Vec<String>, // 存章节名
                              // uploader: String,
                              // checker: String,
                              // upload_time: String,
                              // edit_time: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UploadBook {
    pub name: String,
    pub author: String,
    pub status: String,
    pub tag: String,
    pub desc: String,
    pub chapter: Vec<(String, String)>, // 章节名 章节内容
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct DescBook {
    pub id: String,
    pub name: String,
    pub author: String,
    pub status: String,
    pub tag: String,
    pub desc: String,
    // uploader: String,
    // checker: String,
    // upload_time: String,
    // edit_time: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchBook {
    pub name: String,
    pub author: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Chapter {
    pub name: String,
    pub content: String,
}

impl Book {
    pub async fn from_uplaod_book(upload_book: &UploadBook, book_id: &String) -> Self {
        let mut chapter: Vec<String> = Vec::new();
        for (name, content) in upload_book.chapter.clone() {
            chapter.push(name);
        }
        Self {
            id: book_id.clone(),
            name: upload_book.name.clone(),
            author: upload_book.author.clone(),
            status: upload_book.status.clone(),
            tag: upload_book.tag.clone(),
            desc: upload_book.desc.clone(),
            chapter: chapter,
        }
    }
}
