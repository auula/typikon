use tera::{Context, Tera};

use crate::{
    book::search::Document,
    cli,
    html::{self, Hypertext, Markdown, Template},
    utils::{self, Logger},
};

use super::{root, settings, Root, Settings};
use std::{
    collections::HashMap,
    fs::{self, File},
    io::{self, Write},
    path::Path,
};

#[derive(Debug)]
pub struct Builder {
    pub engine: Tera,
    pub root: Root,
    pub settings: Settings,
    pub document_index: i32,
    pub search_data: Vec<Document>,
}

pub fn new_builder() -> Result<Builder, Box<dyn std::error::Error>> {
    let root = root::get_root()?;
    let settings = settings::get_settings()?;
    let engine = Tera::new(
        // create new tera instance for template engine.
        format!(
            "{}/{}/**/*.html",
            &settings.get_theme_path(),
            &settings.get_theme_name()
        )
        .as_str(),
    )?;

    Ok(Builder {
        engine,
        root,
        settings,
        document_index: 0,
        search_data: Vec::new(),
    })
}

impl Builder {
    pub fn generate_books(&mut self) -> io::Result<()> {
        cli::ouput_banner();
        self.create_workspace();
        self.copy_theme_assets();
        self.render_index_html();
        self.render_chapter_html();
        self.render_sub_chapter_html();
        self.save_data_json()?;
        Ok(())
    }

    fn save_data_json(&self) -> std::io::Result<()> {
        let json_data = serde_json::to_string_pretty(&self.search_data)?;
        let out_root_path = &self.settings.get_output_path();
        let data_json_path = Path::new(out_root_path).join("data.json");
        let mut file = File::create(data_json_path)?;
        file.write_all(json_data.as_bytes())?;
        Ok(())
    }

    // copy theme assets to output directory
    fn copy_theme_assets(&self) {
        let mut log = Logger::console_log();
        let from = format!(
            "{}/{}/assets",
            self.settings.get_theme_path(),
            self.settings.get_theme_name()
        );

        let to = format!("{}/assets", self.settings.get_output_path());

        // copy theme assets
        match utils::copy_file(Path::new(&from), Path::new(&to)) {
            Ok(_) => log.info(format_args!("Building static assets {:?} successful", &to)),
            Err(err) => log.error(format_args!("Failed to build static assets : {:?}", err)),
        }
    }

    fn create_workspace(&self) {
        let mut log = Logger::console_log();
        let base_path = Path::new(self.settings.get_output_path());

        if base_path.exists() {
            let _ = fs::remove_dir_all(base_path);
        }

        let _ = fs::create_dir(base_path).map_err(|err| {
            log.error(format_args!(
                "Failed to create working directory : {:?}",
                err
            ));
            return;
        });

        log.info(format_args!(
            "New create working directory {:?} successful",
            base_path
        ));
    }

    fn render_index_html(&mut self) {
        let mut log = Logger::console_log(); // initialize console logger.
        let template = Template::new(&self.settings, &self.root.get_chapters());

        let base_path = Path::new(self.settings.get_input_path());
        let index_file = base_path.join(&self.root.path);

        // read markdown content from file
        match fs::read_to_string(&index_file) {
            Ok(markdown_content) => {
                // create Hypertext instance
                let hypertext = Hypertext::new(
                    &template.name,
                    &index_file.display().to_string(),
                    Markdown::new(markdown_content),
                );

                let mut context = Context::new();
                context.insert("title", &template.name);
                context.insert("icon", &template.icon);
                context.insert("keywords", &template.keywords);
                context.insert("language", &template.html_lang);
                context.insert("chapters", &template.chapters);
                context.insert("custom_js", &template.custom_js);
                context.insert("custom_css", &template.custom_css);
                context.insert("highlight", &template.highlight);
                context.insert("description", &template.description);
                context.insert("content", &hypertext.to_html());

                // render template
                let rendered = self.engine.render("index.html", &context).unwrap();

                // get the root inputs directory path.
                let out_base_path = Path::new(self.settings.get_output_path());

                let index_chapter_file = out_base_path.join("index.html");

                // push full text search data
                self.document_index += 1;
                self.search_data.push(Document::new(
                    self.document_index,
                    "/index.html".to_string(),
                    template.name.clone(),
                    hypertext.to_html(),
                ));

                // write to file /chapter2/chapter_1.1.1.html
                match fs::write(index_chapter_file.display().to_string(), rendered) {
                    Ok(_) => log.info(format_args!(
                        "Building hypertext file {:?} successful",
                        index_chapter_file
                    )),
                    Err(err) => log.error(format_args!("Failed to data write file: {:?}", err)),
                }
            }
            Err(_) => log.error(format_args!("file not found {:?}", index_file)),
        }
    }

    fn render_sub_chapter_html(&mut self) {
        let mut log = Logger::console_log();
        let template = Template::new(&self.settings, &self.root.get_chapters());

        let root = self.settings.get_output_path();
        let base_path = Path::new(&root);

        let sub_chapters_html = self.sub_chapters_html();

        // iterate over sub-chapters and their HTML content
        for (chapter_title, html_chapters) in sub_chapters_html.iter() {
            for html_content in html_chapters.iter() {
                // generate full file path
                let chapter_dir = base_path.join(&chapter_title.replace(' ', "-").to_lowercase());

                // create context and add data
                let mut context = Context::new();
                context.insert("title", &template.name);
                context.insert("icon", &template.icon);
                context.insert("keywords", &template.keywords);
                context.insert("language", &template.html_lang);
                context.insert("chapters", &template.chapters);
                context.insert("custom_js", &template.custom_js);
                context.insert("custom_css", &template.custom_css);
                context.insert("highlight", &template.highlight);
                context.insert("description", &template.description);
                context.insert("content", &html_content.to_html());

                // render template
                let rendered = self.engine.render("index.html", &context).unwrap();

                // write to file /chapter2/chapter_1.1.1.md => /chapter2/chapter_1.1.1
                let sub_chapter_file = chapter_dir.join(
                    utils::remove_md_extension(&html_content.path)
                        .replace(' ', "-")
                        .to_lowercase(),
                );

                // push full text search data
                self.document_index += 1;
                self.search_data.push(Document::new(
                    self.document_index,
                    format!(
                        "/{}/{}.html",
                        chapter_title.replace(' ', "-").to_lowercase(),
                        utils::remove_md_extension(&html_content.path)
                            .replace(' ', "-")
                            .to_lowercase()
                    ),
                    html_content.title.to_string(),
                    html_content.to_html(),
                ));

                // write to file /chapter2/chapter_1.1.1.html
                match fs::write(
                    format!("{}.html", &sub_chapter_file.display().to_string()),
                    &rendered,
                ) {
                    Ok(_) => log.info(format_args!(
                        "Building hypertext file {:?} successful",
                        sub_chapter_file
                    )),
                    Err(err) => log.error(format_args!("Failed to write file: {:?}", err)),
                }
            }
        }
    }

    fn render_chapter_html(&mut self) {
        let mut log = Logger::console_log();
        let template = Template::new(&self.settings, &self.root.get_chapters());
        let root = self.settings.get_output_path();
        let base_path = Path::new(&root);

        let chapters_html: HashMap<String, Hypertext> = self.chapters_html();

        // iterate over chapters and their HTML content
        for (chapter_title, html_content) in chapters_html.iter() {
            let chapter_dir = base_path.join(&chapter_title.replace(' ', "-").to_lowercase());
            let chapter_file = chapter_dir.join("index.html");

            // create directory
            match fs::create_dir_all(&chapter_dir) {
                Ok(_) => log.info(format_args!("Folder created {:?} successful", chapter_dir)),
                Err(err) => log.error(format_args!("Failed to create directory: {:?}", err)),
            }

            let mut context = Context::new();
            context.insert("title", &template.name);
            context.insert("icon", &template.icon);
            context.insert("keywords", &template.keywords);
            context.insert("language", &template.html_lang);
            context.insert("chapters", &template.chapters);
            context.insert("custom_js", &template.custom_js);
            context.insert("custom_css", &template.custom_css);
            context.insert("highlight", &template.highlight);
            context.insert("description", &template.description);
            context.insert("content", &html_content.to_html());

            // push full text search data
            self.document_index += 1;
            self.search_data.push(Document::new(
                self.document_index,
                format!(
                    "/{}/index.html",
                    chapter_title.replace(' ', "-").to_lowercase(),
                ),
                html_content.title.to_string(),
                html_content.to_html(),
            ));

            // render template
            let rendered = self.engine.render("index.html", &context).unwrap();

            // write to file
            match fs::write(&chapter_file, &rendered) {
                Ok(_) => log.info(format_args!(
                    "Building hypertext file {:?} successful",
                    chapter_file
                )),
                Err(err) => log.error(format_args!("Failed to data write file: {:?}", err)),
            }
        }
    }

    // HTML content for chapters and return as HashMap<String, html::Hypertext>
    fn chapters_html(&self) -> HashMap<String, html::Hypertext> {
        let mut result: HashMap<String, html::Hypertext> = HashMap::new();
        let mut log = Logger::console_log();

        // get chapters from root if available
        for chapter in self.root.get_chapters() {
            let chapter_index_path = Path::new(&chapter.path);

            let chapter_path = format!(
                "{}/{}",
                self.settings.get_input_path(),
                &chapter_index_path.display().to_string()
            );

            // read markdown content from file
            match fs::read_to_string(&chapter_path) {
                Ok(markdown_content) => {
                    // create Hypertext instance
                    let hypertext = Hypertext::new(
                        &chapter.title,
                        &chapter_path,
                        Markdown::new(&markdown_content),
                    );

                    // insert into result HashMap
                    result.insert(utils::remove_md_extension(chapter.path.as_str()), hypertext);

                    log.info(format_args!(
                        "Loading markdown file {:?} successful",
                        chapter_path
                    ));
                }
                Err(_) => log.error(format_args!("file not found {:?}", &chapter_path)),
            }
        }

        result
    }

    // HTML content for sub-chapters and return as HashMap<String, Vec<html::Hypertext>>
    fn sub_chapters_html(&self) -> HashMap<String, Vec<html::Hypertext>> {
        let mut result: HashMap<String, Vec<html::Hypertext>> = HashMap::new();
        let mut log = Logger::console_log();

        // iterate over chapters
        for chapter in self.root.get_chapters() {
            let mut chapter_hypertexts: Vec<html::Hypertext> = Vec::new();

            for sub_chapter in &chapter.sub_chapters {
                let sub_chapter_path =
                    format!("{}/{}", self.settings.get_input_path(), &sub_chapter.path);

                // read markdown content from file
                match fs::read_to_string(&sub_chapter_path) {
                    Ok(markdown_content) => {
                        // create Hypertext instance
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

            // insert sub-chapters into result HashMap
            result.insert(
                utils::remove_md_extension(chapter.path.as_str()),
                chapter_hypertexts,
            );
        }

        result
    }
}
