use crate::book;

use super::Root;

#[derive(Debug)]
pub struct Builder {
    root: Root,
    out_dir: String,
}

impl Builder {
    // 渲染整本书
    pub fn render(&self) -> Result<(), std::io::Error> {
        println!("Builder.render() {:?}", self);
        Ok(())
    }

    pub fn set_out_dir(&mut self, out_dir_path: &str) {
        self.out_dir = out_dir_path.to_string();
    }
}

pub fn new_builder() -> Result<Builder, Box<dyn std::error::Error>> {
    // 获取书籍根配置
    let root = book::get_root()?;

    // 返回 Builder 实例
    Ok(Builder {
        root,
        out_dir: "./docs/".to_string(),
    })
}
