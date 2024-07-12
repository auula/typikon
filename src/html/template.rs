use serde::{Deserialize, Serialize};

use crate::{
    book::{self, Settings, SubChapter},
    utils,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Chapter {
    pub title: String,                 // Title of the chapter.
    pub path: String,                  // Path to the chapter.
    pub sub_chapters: Vec<SubChapter>, // List of sub-chapters.
}

// Template struct for HTML template and data to be rendered
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Template {
    pub name: String,
    pub icon: String,
    pub keywords: String,
    pub html_lang: String,
    pub description: String,
    pub chapters: Vec<Chapter>,
    pub highlight: Vec<String>,
    pub custom_js: Vec<String>,
    pub custom_css: Vec<String>,
}

impl Template {
    // Constructor method to create a new Template instance
    pub fn new(settings: &Settings, chapters: &[book::Chapter]) -> Template {
        let mut new_chapters = Vec::new();

        // Format chapters
        chapters.iter().for_each(|chapter| {
            let mut new_sub_chapters = Vec::new();

            // Format sub-chapters
            chapter.sub_chapters.iter().for_each(|sub_chapter| {
                let sub_chapter_path = utils::remove_md_extension(&sub_chapter.path)
                    .replace(' ', "-")
                    .to_lowercase();

                new_sub_chapters.push(SubChapter {
                    title: sub_chapter.title.clone(),
                    path: sub_chapter_path,
                });
            });

            new_chapters.push(Chapter {
                title: chapter.title.clone(),
                path: utils::remove_md_extension(&chapter.path)
                    .replace(' ', "-")
                    .to_lowercase(),
                sub_chapters: new_sub_chapters,
            });
        });

        Self {
            chapters: new_chapters,
            name: settings.title.clone(),
            icon: settings.icon.clone(),
            keywords: settings.keywords.clone(),
            html_lang: settings.language.clone(),
            highlight: settings.highlight.clone(),
            custom_js: settings.custom_js.clone(),
            custom_css: settings.custom_css.clone(),
            description: settings.description.clone(),
        }
    }
}
