use crate::component::database::book::{
    check_book_unique, fuzzy_search_book_by_name_and_author, update_book_info_db,
};
use crate::component::model::book::{Book, Chapter, DescBook, SearchBook};
use crate::util::config::CONFIG;
use crate::{
    component::model::book::InfoBook,
    util::error::{AppError, BookError},
};
use axum::http::header;
use axum::response::IntoResponse;
use axum::Json;
use axum::{
    extract::{Multipart, Path},
    routing::{get, post},
    Router,
};
use tokio::io::AsyncWriteExt;

pub async fn init() -> Router {
    Router::new()
        .route("/", get(book))
        .route("/upload", post(upload_book))
        .route("/:book_id", get(read_book).put(edit_book))
        .route("/:book_id/:chapter_id", get(read_chapter).put(edit_chapter))
        .route("/download/:book_id", get(download_book))
        .route("/search", post(search_book))
        .route("/test/upload/book", post(test_upload_book))
}

async fn test_upload_book(Json(book): Json<Book>) -> Result<(), AppError> {
    let res = update_book_info_db(&book).await?;
    Ok(res)
}

/*
curl http://127.0.0.1:8000/book
*/
pub async fn book() -> &'static str {
    "book"
}

async fn upload_book(mut multipart: Multipart) -> Result<&'static str, AppError> {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let data = field.bytes().await.unwrap();
        let upload_book: InfoBook = serde_json::from_slice(&data)
            .map_err(|_| AppError::BookError(BookError::UploadFileFormatError))?;

        // 检查上传文件的唯一性
        if check_book_unique(&upload_book.name, &upload_book.author).await? {
            return Err(AppError::BookError(BookError::BookExist));
        }

        // 将 json 文件按照规定的格式转为 txt，并保存
        let file_name = format!("[{}]{}", upload_book.author, upload_book.name);
        let book_id = nanoid::nanoid!(10);
        let file_dir_path = format!("{}/book/{}", CONFIG.data.path, book_id);
        let file_path = format!("{}/book/{}/{}.txt", CONFIG.data.path, book_id, file_name);
        tokio::fs::create_dir(&file_dir_path).await.unwrap();
        let mut file = tokio::fs::File::create(&file_path).await.unwrap();
        let book_info = format!(
            "书名: {}\n作者: {}\n状态: {}\n标签: {}\n简介: \n{}",
            upload_book.name,
            upload_book.author,
            upload_book.status,
            upload_book.tag,
            upload_book.desc
        );
        let mut chapters: Vec<(String, String)> = Vec::new();
        file.write_all(book_info.as_bytes()).await.unwrap();
        for (chapter_name, chapter_content) in &upload_book.chapter {
            let chapter = format!("\n\n{}\n{}", chapter_name, chapter_content);
            file.write_all(chapter.as_bytes()).await.unwrap();

            let chapter_id = nanoid::nanoid!(5);
            chapters.push((chapter_id.clone(), chapter_name.clone()));
            let chapter_path = format!("{}/book/{}/{}.txt", CONFIG.data.path, book_id, chapter_id);
            let _ = tokio::fs::File::create(&chapter_path).await.unwrap();
            tokio::fs::write(
                &chapter_path,
                format!("{}\n{}", chapter_name, chapter_content).as_bytes(),
            )
            .await
            .unwrap();
        }

        // 生成 book_info.json 文件
        // 并保存
        let mut book_info = upload_book.clone();
        book_info.chapter = chapters;
        let book_info_string = serde_json::to_string(&book_info).unwrap();
        let book_info_file_path = format!("{}/book/{}/book_info.json", CONFIG.data.path, book_id);
        let _ = tokio::fs::File::create(&book_info_file_path).await.unwrap();
        tokio::fs::write(&book_info_file_path, book_info_string.as_bytes())
            .await
            .unwrap();

        // 将 book_info 中的数据上传到数据库中
        let book_info = Book::from_info_book(&book_info, &book_id).await;
        update_book_info_db(&book_info).await?;
        return Ok("success");
    }
    Err(AppError::BookError(BookError::NoUploadFile))
}

async fn read_book(Path(book_id): Path<String>) -> Result<Json<InfoBook>, AppError> {
    let book_info_path = format!("{}/book/{}/book_info.json", CONFIG.data.path, book_id);
    let book_info_string = tokio::fs::read_to_string(&book_info_path).await.unwrap();
    let book_info: InfoBook = serde_json::from_str(&book_info_string).unwrap();
    Ok(Json(book_info))
}

async fn edit_book(Path(book_id): Path<String>, Json(book): Json<Book>) -> Result<(), AppError> {
    let book_info_path = format!("{}/book/{}/book_info.json", CONFIG.data.path, book_id);
    let book_info_string = serde_json::to_string(&book).unwrap();
    tokio::fs::write(book_info_path, book_info_string.as_bytes())
        .await
        .unwrap();
    Ok(())
}

async fn read_chapter(
    Path((book_id, chapter_id)): Path<(String, String)>,
) -> Result<String, AppError> {
    let chapter_path = format!("{}/book/{}/{}.txt", CONFIG.data.path, book_id, chapter_id);
    let chapter_content = tokio::fs::read_to_string(&chapter_path).await.unwrap();
    Ok(chapter_content)
}

async fn edit_chapter(
    Path((book_id, chapter_id)): Path<(String, String)>,
    Json(chapter): Json<Chapter>,
) {
    let book_info_path = format!("{}/book/{}/book_info.json", CONFIG.data.path, book_id);
    let book_info_string = tokio::fs::read_to_string(&book_info_path).await.unwrap();
    let mut book_info_json: Book = serde_json::from_str(&book_info_string).unwrap();
    let chapter_id = chapter_id.parse::<usize>().unwrap();
    if book_info_json.chapter[chapter_id] != chapter.name {
        book_info_json.chapter[chapter_id] = chapter.name;
        let book_info_string = serde_json::to_string(&book_info_json).unwrap();
        tokio::fs::write(book_info_path, book_info_string.as_bytes())
            .await
            .unwrap();
    }
    let chapter_path = format!("{}/book/{}/{}.txt", CONFIG.data.path, book_id, chapter_id);
    tokio::fs::write(chapter_path, chapter.content.as_bytes())
        .await
        .unwrap();
}

async fn download_book(Path(book_id): Path<String>) -> Result<impl IntoResponse, AppError> {
    let book_info_path = format!("{}/book/{}/book_info.json", CONFIG.data.path, book_id);
    let book_info_string = tokio::fs::read_to_string(&book_info_path).await.unwrap();
    let book_info_json: InfoBook = serde_json::from_str(&book_info_string).unwrap();
    let book_file_name = format!("[{}]{}.txt", book_info_json.author, book_info_json.name);
    let book_file_path = format!("{}/book/{}/{}", CONFIG.data.path, book_id, book_file_name);
    let headers = [
        (
            header::CONTENT_TYPE,
            "text/plain; charset=utf-8".to_string(),
        ),
        (
            header::CONTENT_DISPOSITION,
            format!("attachment; filename=\"{}\"", book_file_name),
        ),
    ];
    let book_file = tokio::fs::File::open(&book_file_path).await.unwrap();
    let book_stream = tokio_util::io::ReaderStream::new(book_file);
    let body = axum::body::Body::from_stream(book_stream);

    Ok((headers, body))
}

async fn search_book(Json(search_book): Json<SearchBook>) -> Result<Json<Vec<DescBook>>, AppError> {
    let res = fuzzy_search_book_by_name_and_author(&search_book.name, &search_book.author).await?;
    Ok(res)
}
