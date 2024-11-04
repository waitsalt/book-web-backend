use serde::Deserialize;
use serde::Serialize;
use tokio::io::AsyncWriteExt;

use crate::util::error::AppError;

#[derive(Debug, Deserialize, Serialize)]
pub struct Book {
    id: String,
    name: String,
    author: String,
    tag: String,
    desc: String,
    chapter: Vec<(String, String)>,
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
    pub chapter: Vec<(String, String)>,
}

impl UploadBook {
    pub async fn save_to_file(&self) -> Result<(), AppError> {
        let file_name = format!("[{}]{}", self.author, self.name);
        let file_path = format!("{}.txt", file_name);
        let mut file = tokio::fs::File::create(&file_path)
            .await
            .map_err(|_| AppError::Other)?;

        let book_info = format!(
            "书名: {}\n作者: {}\n状态: {}\n标签: {}\n简介: \n{}",
            self.name, self.author, self.status, self.tag, self.desc
        );
        file.write_all(book_info.as_bytes())
            .await
            .map_err(|_| AppError::Other)?;
        for (chapter_name, chapter_content) in &self.chapter {
            let chapter = format!("\n\n{}\n{}", chapter_name, chapter_content);
            file.write_all(chapter.as_bytes())
                .await
                .map_err(|_| AppError::Other)?;
        }
        Ok(())
    }
}
