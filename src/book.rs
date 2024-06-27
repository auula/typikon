use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Book {
    title: String,
    chapters: Vec<Chapter>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Chapter {
    title: String,
    path: String,
    url: String
    sub_chapters: Vec<SubChapter>,
}

#[derive(Debug, Serialize, Deserialize)]
struct SubChapter {
    title: String,
    path: String,
    url: String
}