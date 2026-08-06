use crate::book::{self, Settings, SubChapter};
use crate::utils::{markdown_route, path_to_url, site_url};
use serde::{Deserialize, Serialize};
use std::io;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Chapter {
    pub title: String,
    pub path: String,
    pub sub_chapters: Vec<SubChapter>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Template {
    pub name: String,
    pub author: String,
    pub icon: String,
    pub keywords: String,
    pub html_lang: String,
    pub description: String,
    pub base_url: String,
    pub chapters: Vec<Chapter>,
    pub highlight: Vec<String>,
    pub custom_js: Vec<String>,
    pub custom_css: Vec<String>,
}

impl Template {
    pub fn new(settings: &Settings, chapters: &[book::Chapter]) -> io::Result<Template> {
        let base_url = settings.normalized_base_url()?;
        let mut template_chapters = Vec::with_capacity(chapters.len());

        for chapter in chapters {
            let chapter_route = markdown_route(Path::new(&chapter.path))?;
            let mut sub_chapters = Vec::with_capacity(chapter.sub_chapters.len());

            for sub_chapter in &chapter.sub_chapters {
                let sub_chapter_route = markdown_route(Path::new(&sub_chapter.path))?;
                sub_chapters.push(SubChapter {
                    title: sub_chapter.title.clone(),
                    path: path_to_url(&sub_chapter_route)?,
                });
            }

            template_chapters.push(Chapter {
                title: chapter.title.clone(),
                path: path_to_url(&chapter_route)?,
                sub_chapters,
            });
        }

        // Prefix configured local assets with base_url while preserving external URLs.
        let icon = site_url(&base_url, &settings.icon);
        let custom_js = settings
            .custom_js
            .iter()
            .map(|path| site_url(&base_url, path))
            .collect();
        let custom_css = settings
            .custom_css
            .iter()
            .map(|path| site_url(&base_url, path))
            .collect();

        Ok(Self {
            chapters: template_chapters,
            name: settings.title.clone(),
            author: settings.author.clone(),
            icon,
            keywords: settings.keywords.clone(),
            html_lang: settings.language.clone(),
            highlight: settings.highlight.clone(),
            custom_js,
            custom_css,
            description: settings.description.clone(),
            base_url,
        })
    }
}
