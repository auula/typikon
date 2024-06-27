use std::{
    io::{self},
    path::Path,
};

use super::{Hypertext, Markdown};

pub fn render_to_html(markdwon: Markdown, path: &Path) -> Result<Hypertext, io::Error> {
    let _ = path;
    let _ = markdwon;
    unimplemented!()
}
