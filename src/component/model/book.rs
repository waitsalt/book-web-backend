use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
pub struct Book {
    name: String,
    author: String,
    tag: String,
    desc: String,
    chapter: Vec<(String, String)>,
    uploader: String,
    checker: String,
}
