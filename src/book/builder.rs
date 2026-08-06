use crate::book::search::Document;
use crate::html::{Markdown, Template};
use crate::utils::{
    copy_file, invalid_input, markdown_route, path_to_url, site_url, validate_relative_path, Logger,
};
use crate::{TypikonError, TypikonResult};
use std::collections::HashSet;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use tera::{Context, Tera};

use super::settings::ResolvedDirectories;
use super::{root, settings, Root, Settings};

static UNIQUE_PATH_COUNTER: AtomicU64 = AtomicU64::new(0);

#[derive(Debug)]
pub struct Builder {
    pub engine: Tera,
    pub root: Root,
    pub settings: Settings,
    pub directories: ResolvedDirectories,
    pub document_index: i32,
    pub search_data: Vec<Document>,
    template: Template,
}

pub fn new_builder() -> TypikonResult<Builder> {
    let project_root = std::env::current_dir()?;
    new_builder_at(project_root)
}

pub fn new_builder_at(project_root: impl AsRef<Path>) -> TypikonResult<Builder> {
    let project_root = project_root.as_ref().canonicalize()?;
    let root = root::get_root_from_file(project_root.join("root.yml"))?;
    let settings = settings::get_settings_from_file(project_root.join("settings.yml"))?;
    let directories = settings.resolve_directories(&project_root)?;

    validate_book(&root, &directories.input)?;
    validate_output_routes(&root)?;

    let template_path = directories.active_theme.join("index.html");
    let mut engine = Tera::default();

    // Load one explicit template to avoid Windows separators and glob characters in paths.
    engine.add_template_file(&template_path, Some("index.html"))?;
    let template = Template::new(&settings, &root.chapters)?;

    Ok(Builder {
        engine,
        root,
        settings,
        directories,
        document_index: 0,
        search_data: Vec::new(),
        template,
    })
}

impl Builder {
    pub fn generate_books(&mut self) -> TypikonResult<()> {
        self.document_index = 0;
        self.search_data.clear();

        let temporary_output = unique_sibling_path(&self.directories.output, "building")?;
        fs::create_dir_all(&temporary_output)?;

        // Replace the published site only after the temporary build has fully succeeded.
        if let Err(error) = self.generate_into(&temporary_output) {
            let _ = fs::remove_dir_all(&temporary_output);
            return Err(error);
        }

        replace_output_directory(&temporary_output, &self.directories.output)?;
        Ok(())
    }

    fn generate_into(&mut self, output_root: &Path) -> TypikonResult<()> {
        let mut log = Logger::console_log();
        let assets_source = self.directories.active_theme.join("assets");
        let assets_output = output_root.join("assets");
        copy_file(&assets_source, &assets_output)?;
        log.info(format_args!(
            "Building static assets {:?} successful",
            assets_output
        ));

        let index_markdown = self.read_markdown(Path::new(&self.root.path), "index")?;
        self.render_page(
            output_root,
            Path::new("index.html"),
            self.template.name.clone(),
            index_markdown,
        )?;

        // Preserve root.yml order so data.json and logs remain deterministic across platforms.
        for chapter in self.root.chapters.clone() {
            let chapter_route = markdown_route(Path::new(&chapter.path))?;
            let chapter_output = chapter_route.join("index.html");
            let chapter_markdown = self.read_markdown(Path::new(&chapter.path), "chapter")?;
            self.render_page(
                output_root,
                &chapter_output,
                chapter.title.clone(),
                chapter_markdown,
            )?;

            for sub_chapter in chapter.sub_chapters {
                let sub_route = markdown_route(Path::new(&sub_chapter.path))?;
                let mut sub_output = chapter_route.join(sub_route);
                sub_output.set_extension("html");
                let sub_markdown =
                    self.read_markdown(Path::new(&sub_chapter.path), "subchapter")?;
                self.render_page(output_root, &sub_output, sub_chapter.title, sub_markdown)?;
            }
        }

        self.save_data_json(output_root)?;
        Ok(())
    }

    fn read_markdown(&self, relative_path: &Path, label: &str) -> TypikonResult<Markdown> {
        let source = resolve_markdown_file(&self.directories.input, relative_path, label)?;
        Ok(Markdown::new(fs::read_to_string(source)?))
    }

    fn render_page(
        &mut self,
        output_root: &Path,
        relative_output: &Path,
        page_title: String,
        markdown: Markdown,
    ) -> TypikonResult<()> {
        let html_content = markdown.to_html();
        let search_content = markdown.to_plain_text();
        let context = self.template_context(&html_content);
        let rendered = self.engine.render("index.html", &context)?;
        let output_file = output_root.join(relative_output);

        // Create every parent directory so nested subchapter paths work on all platforms.
        if let Some(parent) = output_file.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&output_file, rendered)?;

        let route = path_to_url(relative_output)?;
        self.document_index += 1;
        self.search_data.push(Document::new(
            self.document_index,
            site_url(&self.template.base_url, &route),
            page_title,
            search_content,
        ));

        let mut log = Logger::console_log();
        log.info(format_args!(
            "Building hypertext file {:?} successful",
            output_file
        ));
        Ok(())
    }

    fn template_context(&self, content: &str) -> Context {
        let mut context = Context::new();
        context.insert("title", &self.template.name);
        context.insert("author", &self.template.author);
        context.insert("icon", &self.template.icon);
        context.insert("keywords", &self.template.keywords);
        context.insert("language", &self.template.html_lang);
        context.insert("base_url", &self.template.base_url);
        context.insert("chapters", &self.template.chapters);
        context.insert("custom_js", &self.template.custom_js);
        context.insert("custom_css", &self.template.custom_css);
        context.insert("highlight", &self.template.highlight);
        context.insert("description", &self.template.description);
        context.insert("content", content);
        context
    }

    fn save_data_json(&self, output_root: &Path) -> TypikonResult<()> {
        let json_data = serde_json::to_string_pretty(&self.search_data)?;
        let data_json_path = output_root.join("data.json");
        let mut file = File::create(data_json_path)?;
        file.write_all(json_data.as_bytes())?;
        file.sync_all()?;
        Ok(())
    }
}

fn validate_book(root: &Root, input_root: &Path) -> TypikonResult<()> {
    resolve_markdown_file(input_root, Path::new(&root.path), "index")?;
    for chapter in &root.chapters {
        if chapter.title.trim().is_empty() {
            return Err(invalid_input("Chapter titles cannot be empty").into());
        }
        resolve_markdown_file(input_root, Path::new(&chapter.path), "chapter")?;
        for sub_chapter in &chapter.sub_chapters {
            if sub_chapter.title.trim().is_empty() {
                return Err(invalid_input("Subchapter titles cannot be empty").into());
            }
            resolve_markdown_file(input_root, Path::new(&sub_chapter.path), "subchapter")?;
        }
    }
    Ok(())
}

fn resolve_markdown_file(input_root: &Path, path: &Path, label: &str) -> TypikonResult<PathBuf> {
    let relative = validate_relative_path(path, &format!("{label} path"))?;
    markdown_route(&relative)?;
    let source = input_root.join(relative);
    let canonical = source.canonicalize().map_err(|error| {
        invalid_input(format!(
            "{label} file does not exist {}: {error}",
            source.display()
        ))
    })?;

    // Recheck the canonical prefix to prevent symlinks from reading files outside the input.
    if !canonical.starts_with(input_root) || !canonical.is_file() {
        return Err(invalid_input(format!(
            "{label} must be a file inside the input directory: {}",
            source.display()
        ))
        .into());
    }

    Ok(canonical)
}

fn validate_output_routes(root: &Root) -> TypikonResult<()> {
    let mut routes = HashSet::new();
    routes.insert(PathBuf::from("index.html"));

    for chapter in &root.chapters {
        let chapter_route = markdown_route(Path::new(&chapter.path))?;
        insert_unique_route(&mut routes, chapter_route.join("index.html"))?;

        for sub_chapter in &chapter.sub_chapters {
            let sub_route = markdown_route(Path::new(&sub_chapter.path))?;
            let mut output = chapter_route.join(sub_route);
            output.set_extension("html");
            insert_unique_route(&mut routes, output)?;
        }
    }

    Ok(())
}

fn insert_unique_route(routes: &mut HashSet<PathBuf>, route: PathBuf) -> TypikonResult<()> {
    if !routes.insert(route.clone()) {
        return Err(invalid_input(format!(
            "Multiple chapters generate the same output path: {}",
            route.display()
        ))
        .into());
    }
    Ok(())
}

fn unique_sibling_path(output: &Path, suffix: &str) -> TypikonResult<PathBuf> {
    let parent = output
        .parent()
        .ok_or_else(|| invalid_input("The output directory has no parent"))?;
    let name = output
        .file_name()
        .and_then(|value| value.to_str())
        .ok_or_else(|| invalid_input("The output directory name must be valid UTF-8"))?;

    loop {
        let counter = UNIQUE_PATH_COUNTER.fetch_add(1, Ordering::Relaxed);
        let candidate = parent.join(format!(
            ".{name}.typikon-{suffix}-{}-{counter}",
            std::process::id()
        ));
        if !candidate.exists() {
            return Ok(candidate);
        }
    }
}

fn replace_output_directory(temporary: &Path, output: &Path) -> TypikonResult<()> {
    if !output.exists() {
        fs::rename(temporary, output)?;
        return Ok(());
    }

    let backup = unique_sibling_path(output, "backup")?;
    fs::rename(output, &backup)?;
    if let Err(error) = fs::rename(temporary, output) {
        let restore_result = fs::rename(&backup, output);
        return match restore_result {
            Ok(()) => Err(error.into()),
            Err(restore_error) => Err(format!(
                "Failed to publish the new site: {error}; restoring the old site also failed: {restore_error}"
            )
            .into()),
        };
    }

    fs::remove_dir_all(backup).map_err(|error| -> TypikonError { error.into() })?;
    Ok(())
}
