use std::{
    fs::File,
    io::{self, Write}, path::Path,
};

use serde::{Deserialize, Serialize};

use super::Markdown;

// Define the HTML type
// 部分 Markdown 已经转义好的内容
#[derive(Debug, Serialize, Deserialize)]
pub struct Hypertext {
    path: String,
    markdown: Markdown,
}

impl Hypertext {

    pub fn new(path: &str, markdown: Markdown) -> Hypertext {
        Hypertext {
            path: path.to_string(),
            markdown: markdown,
        }
    }

    pub fn get_file_path(&self) -> &Path {
        Path::new(&self.path)
    }

    pub fn to_disk_html(&self) -> io::Result<()> {
        // 打开文件，如果不存在则创建
        let mut file = File::create(self.path.clone())?;
        // 写入 HTML 内容
        file.write_all(&self.markdown.to_html().as_bytes())?;
        // 返回成功的 Result
        Ok(())
    }
}
