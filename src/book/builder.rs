use std::{collections::HashMap, fs, io, path::Path};

use tera::Tera;

use crate::{
    book,
    html::{self, template::Template, Hypertext, Markdown},
    utils::Logger,
};

use super::{Root, Settings};

#[derive(Debug)]
pub struct Builder {
    root: Root,
    pub engine: Tera,
    settings: Settings,
    pub templates: Vec<Template>,
}

impl Builder {
    // 渲染整本书
    pub fn render(&mut self) -> io::Result<()> {
        self.create_directory()?;
        self.get_chapters_hypertext();
        self.copy_theme_assets()?;
        Ok(())
    }

    pub fn get_chapters_hypertext(&mut self) -> HashMap<String, Vec<html::Hypertext>> {
        let mut result: HashMap<String, Vec<html::Hypertext>> = HashMap::new();
        let mut log = Logger::console_log();

        if let Some(chapters) = self.get_chapter() {
            // 开始遍历所有文章，找出同一子篇章的
            for chapter in chapters {
                let base_chapter = Path::new(&chapter.title);
                let mut chapter_hypertexts: Vec<html::Hypertext> = Vec::new();

                // 遍历子文章目录，并且将所有的子文章放到同父类中
                for sub_chapter in &chapter.sub_chapters {
                    let sub_chapter_path = &sub_chapter.path;

                    // 开始读取 markdown 文件的内容
                    match fs::read_to_string(sub_chapter_path) {
                        Ok(markdown_content) => {
                            let hypertext =
                                Hypertext::new(sub_chapter_path, Markdown::new(&markdown_content));
                            chapter_hypertexts.push(hypertext);

                            log.info(format_args!(
                                "Loading markdown file {:?} successful",
                                sub_chapter_path
                            ));
                        }
                        Err(err) => {
                            log.error(format_args!("Loading markdown file fail : {:?}", err))
                        }
                    }
                }

                // 转换小写名字
                let chapter_key = base_chapter
                    .file_name()
                    .and_then(|os_str| os_str.to_str())
                    .map_or_else(|| chapter.title.clone(), |s| s.to_string());

                result.insert(chapter_key.to_lowercase(), chapter_hypertexts);
            }
        }

        result
    }

    fn get_chapter(&self) -> Option<Vec<book::Chapter>> {
        if self.root.root.chapters.is_empty() {
            None
        } else {
            Some(self.root.root.chapters.clone())
        }
    }

    fn copy_theme_assets(&self) -> std::io::Result<()> {
        let mut log = Logger::console_log();
        let from = format!(
            "{}/{}/assets",
            self.settings.settings.directory.theme, self.settings.settings.theme
        );
        let to = format!("{}/assets", self.settings.settings.directory.output);

        copy_dir_recursive(Path::new(&from), Path::new(&to))?;

        log.info(format_args!(
            "Building theme assets directory {:?} successful",
            &to
        ));

        Ok(())
    }

    // 先把目录创建好
    pub fn create_directory(&mut self) -> io::Result<()> {
        let base_path = Path::new(&self.settings.settings.directory.output);
        let mut log = Logger::console_log();
        // 如果文件存在就删除并重建
        if base_path.exists() {
            fs::remove_dir_all(base_path)?;
            log.info(format_args!(
                "Clean up output diretory {:?} successful",
                base_path
            ));
        }

        fs::create_dir(base_path)?;
        log.info(format_args!(
            "New create output diretory {:?} successful",
            base_path
        ));

        // 创建存放静态 html 文件的二级目录
        self.root.root.chapters.iter().try_for_each(|chapter| {
            fs::create_dir(base_path.join(&chapter.title.replace(" ", "_").to_lowercase()))
        })
    }
}

pub fn new_builder() -> Result<Builder, Box<dyn std::error::Error>> {
    let root = book::get_root()?;
    let templates = html::get_templates().unwrap();
    let settings = book::get_settings()?;
    let engine = Tera::new(
        format!(
            "{}/{}/**/*.html",
            &settings.settings.directory.theme, &settings.settings.theme
        )
        .as_str(),
    )?;
    // 返回 Builder 实例
    Ok(Builder {
        root,
        templates,
        engine,
        settings,
    })
}

fn copy_dir_recursive(from: &Path, to: &Path) -> io::Result<()> {
    // 创建目标目录如果它不存在
    if !to.exists() {
        fs::create_dir_all(to)?;
    }

    // 遍历源目录
    for entry in fs::read_dir(from)? {
        let entry = entry?;
        let path = entry.path();
        let relative_path = path.strip_prefix(from).unwrap();
        let target_path = to.join(relative_path);

        if path.is_dir() {
            // 递归地复制子目录
            copy_dir_recursive(&path, &target_path)?;
        } else if path.is_file() {
            // 复制文件
            fs::copy(&path, &target_path)?;
        }
    }

    Ok(())
}
