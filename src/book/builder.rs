use std::io;

use tera::Tera;

use crate::{
    book,
    html::{
        self,
        template::{self, Template},
    },
};

use super::{get_settings, Root, Settings};

#[derive(Debug)]
pub struct Builder {
    root: Root,
    pub templates: Template,
    pub engine: Tera,
    settings: Settings,
}

impl Builder {
    // 渲染整本书
    pub fn render(&self) -> io::Result<()> {
        println!("Builder.render() {:?}", self);
        unimplemented!()
    }

    pub fn get_hypertext() -> Vec<html::Hypertext> {
        // 1. 通过 root 目录先创建目录
        // 2. 通过 root 目录读取 md 内容
        // 3. 通过 md 内容构建 Hypertext
        // 4. 通过 Hypertext 落盘
        unimplemented!()
    }

    pub fn get_chapter(&self) -> Option<Vec<book::Chapter>> {
        if self.root.root.chapters.is_empty() {
            None
        } else {
            Some(self.root.root.chapters.clone())
        }
    }

    pub fn get_sub_chapter(&self) -> Option<Vec<book::SubChapter>> {
        if self.root.root.chapters.is_empty() {
            None
        } else {
            Some(
                self.root
                    .root
                    .chapters
                    .iter()
                    .flat_map(|chapter| chapter.sub_chapters.clone())
                    .collect(),
            )
        }
    }

    // 先把目录创建好
    pub fn create_directory() -> io::Result<()> {
        unimplemented!()
    }


}

pub fn new_builder() -> Result<Builder, Box<dyn std::error::Error>> {
    let root = book::get_root()?;
    let templates = html::Template::empty();
    let settings = book::get_settings()?;
    let engine = Tera::new(format!("theme/{}/**/*.html", &settings.settings.theme).as_str())?;
    // 返回 Builder 实例
    Ok(Builder {
        root,
        templates,
        engine,
        settings,
    })
}
