use serde::{Deserialize, Serialize}; // Importing serde attributes.

use crate::{
    // Importing modules from the crate.
    book::{self, About, SubChapter}, // Importing structs About and SubChapter from book module.
    utils,                           // Importing utils module.
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
    pub title: String,          // Title of the template.
    pub keywords: String,       // Keywords for SEO.
    pub description: String,    // Description for SEO.
    pub chapters: Vec<Chapter>, // List of chapters.
    pub language: String,
}

impl Template {
    // Constructor method to create a new Template instance
    pub fn new(about: &About, chapters: &[book::Chapter]) -> Template {
        let mut new_chapters: Vec<Chapter> = Vec::new(); // Initialize empty vector for chapters.

        // Format chapters
        chapters.iter().for_each(|chapter| {
            let mut new_sub_chapters = Vec::new(); // Initialize empty vector for sub-chapters.

            // Format sub-chapters
            chapter.sub_chapters.iter().for_each(|sub_chapter| {
                let sub_chapter_path = utils::remove_md_extension(&sub_chapter.path) // Remove Markdown extension and format path.
                    .replace(' ', "-") // Replace spaces with hyphens.
                    .to_lowercase(); // Convert to lowercase.

                new_sub_chapters.push(SubChapter {
                    title: sub_chapter.title.clone(), // Clone title of sub-chapter.
                    path: sub_chapter_path,           // Assign formatted path.
                });
            });

            // Format main chapter
            let chapter = Chapter {
                title: chapter.title.clone(), // Clone title of chapter.
                path: utils::remove_md_extension(&chapter.index) // Remove Markdown extension and format path.
                    .replace(' ', "-") // Replace spaces with hyphens.
                    .to_lowercase(), // Convert to lowercase.
                sub_chapters: new_sub_chapters, // Assign formatted sub-chapters.
            };

            new_chapters.push(chapter); // Push formatted chapter to new_chapters vector.
        });

        Self {
            chapters: new_chapters,                 // Assign formatted chapters.
            title: about.title.clone(),             // Clone title of about section.
            keywords: about.keywords.clone(),       // Clone keywords for SEO.
            language: about.language.clone(),       // Initialize content as empty string.
            description: about.description.clone(), // Clone description for SEO.
        }
    }
}
