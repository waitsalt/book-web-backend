use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
pub struct Book {
    pub id: String,
    pub name: String,
    pub author: String,
    pub status: String,
    pub tag: String,
    pub desc: String,
    pub chapter: Vec<String>, // 存章节 id
                              // uploader: String,
                              // checker: String,
                              // upload_time: String,
                              // edit_time: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct InfoBook {
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
    pub async fn from_info_book(info_book: &InfoBook, book_id: &String) -> Self {
        let mut chapter: Vec<String> = Vec::new();
        for (id, _) in info_book.chapter.clone() {
            chapter.push(id);
        }
        Self {
            id: book_id.clone(),
            name: info_book.name.clone(),
            author: info_book.author.clone(),
            status: info_book.status.clone(),
            tag: info_book.tag.clone(),
            desc: info_book.desc.clone(),
            chapter: chapter,
        }
    }
}
