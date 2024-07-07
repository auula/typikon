use super::{Root, Settings}; // Importing structs Root and Settings from the parent module.
use crate::{
    // Importing modules from the crate.
    book,
    cli,
    html::{self, Hypertext, Markdown, Template}, // Importing types Hypertext, Markdown, and Template from html module.
    utils::{self, Logger},                       // Importing utils module and Logger type from it.
};
use std::{collections::HashMap, fs, path::Path}; // Importing standard library modules.
use tera::{Context, Tera}; // Importing Context and Tera types from tera crate.

#[derive(Debug)]
pub struct Builder {
    root: Root,         // Root struct containing book data.
    engine: Tera,       // Tera template engine instance for rendering HTML.
    settings: Settings, // Settings struct containing project settings.
}

impl Builder {
    // Render the entire book
    pub fn render(&mut self) -> anyhow::Result<()> {
        cli::print_banner(); // Printing CLI banner.
        self.create_directory(); // Create output directory.
        self.copy_theme_assets(); // Copy theme assets.
        self.render_index_html();
        self.render_chapter_html(); // Render HTML for chapters.
        self.render_sub_chapter_html(); // Render HTML for sub-chapters.
        Ok(())
    }

    fn render_index_html(&mut self) {
        let mut log = Logger::console_log(); // Initialize console logger.
        let template = Template::new(&self.settings.settings.about, &self.root.root.chapters); // Create a new Template instance.

        let root = self.settings.settings.directory.source.clone();
        let base_path = Path::new(&root);
        let index_file = base_path.join(&self.root.root.index);

        // Read markdown content from file
        match fs::read_to_string(&index_file) {
            Ok(markdown_content) => {
                // Create Hypertext instance
                let hypertext = Hypertext::new(
                    "Index File",
                    &index_file.display().to_string(),
                    Markdown::new(markdown_content),
                );

                // Create context and add data
                let mut context = Context::new();
                context.insert("title", &template.title);
                context.insert("language", &template.language);
                context.insert("keywords", &template.keywords);
                context.insert("description", &template.description);
                context.insert("chapters", &template.chapters);
                context.insert("content", &hypertext.to_html());
                context.insert("custom_js", &self.settings.settings.custom_js);
                context.insert("custom_css", &self.settings.settings.custom_css);

                // Render template
                let rendered = self.engine.render("index.html", &context).unwrap();

                let out_root = self.settings.settings.directory.output.clone(); // Get the root inputs directory path.
                let out_base_path = Path::new(&out_root);

                let index_chapter_file = out_base_path.join("index.html");

                // Write to file /chapter2/chapter_1.1.1.html
                match fs::write(index_chapter_file.display().to_string(), rendered) {
                    Ok(_) => log.info(format_args!(
                        "Data written to file {:?} successful",
                        index_chapter_file
                    )),
                    Err(err) => log.error(format_args!("Failed to write file: {:?}", err)),
                }
            }
            Err(_) => log.error(format_args!("File not found {:?}", index_file)),
        }
    }

    // Render HTML for sub-chapters
    fn render_sub_chapter_html(&mut self) {
        let mut log = Logger::console_log(); // Initialize console logger.
        let template = Template::new(&self.settings.settings.about, &self.root.root.chapters); // Create a new Template instance.

        let root = self.settings.settings.directory.output.clone(); // Get the root output directory path.
        let base_path = Path::new(&root); // Create a Path instance for the root directory.

        let sub_chapters_html = self.sub_chapters_html(); // Get HTML content for sub-chapters.

        // Iterate over sub-chapters and their HTML content
        for (chapter_title, html_chapters) in sub_chapters_html.iter() {
            for html_content in html_chapters.iter() {
                // Generate full file path
                let chapter_dir = base_path.join(&chapter_title.replace(' ', "-").to_lowercase());

                // Create context and add data
                let mut context = Context::new();
                context.insert("title", &template.title);
                context.insert("language", &template.language);
                context.insert("keywords", &template.keywords);
                context.insert("description", &template.description);
                context.insert("chapters", &template.chapters);
                context.insert("content", &html_content.to_html());
                context.insert("custom_js", &self.settings.settings.custom_js);
                context.insert("custom_css", &self.settings.settings.custom_css);

                // Render template
                let rendered = self.engine.render("index.html", &context).unwrap();

                // Write to file /chapter2/chapter_1.1.1.md => /chapter2/chapter_1.1.1
                let sub_chapter_file = chapter_dir.join(
                    utils::remove_md_extension(&html_content.path)
                        .replace(' ', "-")
                        .to_lowercase(),
                );

                // Write to file /chapter2/chapter_1.1.1.html
                match fs::write(
                    format!("{}.html", &sub_chapter_file.display().to_string()),
                    &rendered,
                ) {
                    Ok(_) => log.info(format_args!(
                        "Data written to file {:?} successful",
                        sub_chapter_file
                    )),
                    Err(err) => log.error(format_args!("Failed to write file: {:?}", err)),
                }
            }
        }
    }

    // Render HTML for chapters
    fn render_chapter_html(&mut self) {
        let mut log = Logger::console_log(); // Initialize console logger.
        let template = Template::new(&self.settings.settings.about, &self.root.root.chapters); // Create a new Template instance.
        let root = self.settings.settings.directory.output.clone(); // Get the root output directory path.
        let base_path = Path::new(&root); // Create a Path instance for the root directory.

        let chapters_html: HashMap<String, Hypertext> = self.chapters_html(); // Get HTML content for chapters.

        // Iterate over chapters and their HTML content
        for (chapter_title, html_content) in chapters_html.iter() {
            let chapter_dir = base_path.join(&chapter_title.replace(' ', "-").to_lowercase());
            let chapter_file = chapter_dir.join("index.html");

            // Create directory
            match fs::create_dir_all(&chapter_dir) {
                Ok(_) => log.info(format_args!("Folder created {:?} successful", chapter_dir)),
                Err(err) => log.error(format_args!("Failed to create directory: {:?}", err)),
            }

            // Create context and add data
            let mut context = Context::new();
            context.insert("title", &template.title);
            context.insert("language", &template.language);
            context.insert("keywords", &template.keywords);
            context.insert("description", &template.description);
            context.insert("chapters", &template.chapters);
            context.insert("content", &html_content.to_html());
            context.insert("custom_js", &self.settings.settings.custom_js);
            context.insert("custom_css", &self.settings.settings.custom_css);

            // Render template
            let rendered = self.engine.render("index.html", &context).unwrap();

            // Write to file
            match fs::write(&chapter_file, &rendered) {
                Ok(_) => log.info(format_args!(
                    "Data written to file {:?} successful",
                    chapter_file
                )),
                Err(err) => log.error(format_args!("Failed to write file: {:?}", err)),
            }
        }
    }

    // Get chapters and return as Option<Vec<book::Chapter>>
    fn get_chapter(&self) -> Option<Vec<book::Chapter>> {
        if self.root.root.chapters.is_empty() {
            None
        } else {
            Some(self.root.root.chapters.clone())
        }
    }

    // Copy theme assets to output directory
    fn copy_theme_assets(&self) {
        let mut log = Logger::console_log(); // Initialize console logger.
        let from = format!(
            "{}/{}/assets",
            self.settings.settings.directory.theme, self.settings.settings.theme
        );
        let to = format!("{}/assets", self.settings.settings.directory.output);

        // Copy theme assets
        match copy_dir_recursive(Path::new(&from), Path::new(&to)) {
            Ok(_) => log.info(format_args!("Building static assets {:?} successful", &to)),
            Err(err) => log.error(format_args!("Building static assets failed: {:?}", err)),
        }
    }

    // Create output directory, clean if it exists
    fn create_directory(&mut self) {
        let base_path = Path::new(&self.settings.settings.directory.output); // Get output directory path.
        let mut log = Logger::console_log(); // Initialize console logger.

        // If directory exists, remove and recreate
        if base_path.exists() {
            match fs::remove_dir_all(base_path) {
                Ok(_) => log.info(format_args!(
                    "Clean up output directory {:?} successful",
                    base_path
                )),
                Err(err) => log.error(format_args!("Clean up output directory failed: {:?}", err)),
            };
        }

        // Create new directory
        match fs::create_dir(base_path) {
            Ok(_) => log.info(format_args!(
                "New create output directory {:?} successful",
                base_path
            )),
            Err(err) => log.error(format_args!("Create output directory failed: {:?}", err)),
        }
    }

    // Get HTML content for chapters and return as HashMap<String, html::Hypertext>
    fn chapters_html(&mut self) -> HashMap<String, html::Hypertext> {
        let mut result: HashMap<String, html::Hypertext> = HashMap::new(); // Initialize result HashMap.
        let mut log = Logger::console_log(); // Initialize console logger.

        // Get chapters from root if available
        if let Some(chapters) = self.get_chapter() {
            // Iterate over chapters
            for chapter in chapters {
                let chapter_index_path = Path::new(&chapter.index); // Get chapter index path.

                let chapter_path = format!(
                    "{}/{}",
                    self.settings.settings.directory.source,
                    &chapter_index_path.display().to_string()
                );

                // Read markdown content from file
                match fs::read_to_string(&chapter_path) {
                    Ok(markdown_content) => {
                        // Create Hypertext instance
                        let hypertext = Hypertext::new(
                            &chapter.title,
                            &chapter_path,
                            Markdown::new(&markdown_content),
                        );

                        // Insert into result HashMap
                        result.insert(
                            utils::remove_md_extension(chapter.index.as_str()),
                            hypertext,
                        );

                        log.info(format_args!(
                            "Loading markdown file {:?} successful",
                            chapter_path
                        ));
                    }
                    Err(_) => continue,
                }
            }
        }

        result // Return HashMap with HTML content for chapters.
    }

    // Get HTML content for sub-chapters and return as HashMap<String, Vec<html::Hypertext>>
    fn sub_chapters_html(&mut self) -> HashMap<String, Vec<html::Hypertext>> {
        let mut result: HashMap<String, Vec<html::Hypertext>> = HashMap::new(); // Initialize result HashMap.
        let mut log = Logger::console_log(); // Initialize console logger.

        // Get chapters from root if available
        if let Some(chapters) = self.get_chapter() {
            // Iterate over chapters
            for chapter in chapters {
                let mut chapter_hypertexts: Vec<html::Hypertext> = Vec::new(); // Initialize Vec for sub-chapters

                // Iterate over sub-chapters and add to Vec
                for sub_chapter in &chapter.sub_chapters {
                    let sub_chapter_path = format!(
                        "{}/{}",
                        self.settings.settings.directory.source, &sub_chapter.path
                    );

                    // Read markdown content from file
                    match fs::read_to_string(&sub_chapter_path) {
                        Ok(markdown_content) => {
                            // Create Hypertext instance
                            let hypertext = Hypertext::new(
                                &sub_chapter.title,
                                &sub_chapter.path,
                                Markdown::new(&markdown_content),
                            );
                            chapter_hypertexts.push(hypertext);

                            log.info(format_args!(
                                "Loading markdown file {:?} successful",
                                sub_chapter_path
                            ));
                        }
                        Err(_) => log.error(format_args!("File not found {:?}", sub_chapter_path)),
                    }
                }

                // Insert sub-chapters into result HashMap
                result.insert(
                    utils::remove_md_extension(chapter.index.as_str()),
                    chapter_hypertexts,
                );
            }
        }

        result // Return HashMap with HTML content for sub-chapters.
    }
}

// Create a new Builder instance and return Result<Builder, Box<dyn std::error::Error>>
pub fn new_builder() -> anyhow::Result<Builder> {
    let root = book::get_root()?; // Get root data from book module.
    let settings = book::get_settings()?; // Get settings from book module.
    let engine = Tera::new(
        // Create new Tera instance for template engine.
        format!(
            "{}/{}/**/*.html",
            &settings.settings.directory.theme, &settings.settings.theme
        )
        .as_str(),
    )?;

    // Return Builder instance
    Ok(Builder {
        root,
        engine,
        settings,
    })
}

// Recursive function to copy directory contents
fn copy_dir_recursive(from: &Path, to: &Path) -> anyhow::Result<()> {
    // Create target directory if it doesn't exist
    if !to.exists() {
        fs::create_dir_all(to)?;
    }

    // Iterate over entries in source directory
    for entry in fs::read_dir(from)? {
        let entry = entry?;
        let path = entry.path();
        let relative_path = path.strip_prefix(from).unwrap();
        let target_path = to.join(relative_path);

        if path.is_dir() {
            // Recursively copy sub-directory
            copy_dir_recursive(&path, &target_path)?;
        } else if path.is_file() {
            // Copy file
            fs::copy(&path, &target_path)?;
        }
    }

    Ok(()) // Return Ok when copying is complete.
}
