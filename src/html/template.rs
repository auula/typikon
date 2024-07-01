use super::{Hypertext, Markdown};

// 模版 html 和需要被渲染的数据内容
#[derive(Debug)]
pub struct Template {
    // 模版名称用于区分多个文件的模版
    pub name: String,
    // 模版路径，模版存储多个文件层次结构中
    pub path: String,
    // 模版文件的内容
    pub content: String,
    // 要插入渲染好的 HTML 数据，这个是通过 MD 转换好的 HTML 数据段
    pub hypertext: Hypertext,
    // 模版被渲染之后元数据，自定义 css 和 头部信息
    pub data_ctx: tera::Context,
}

impl Template {
    pub fn new(_tmpl: String) -> Template {
        unimplemented!()
    }

    pub fn empty() -> Template {
        Template {
            name: "index.html".to_string(),
            path: "index.html".to_string(),
            content: "".to_string(),
            hypertext: Hypertext::new("index.html", Markdown::new("")),
            data_ctx: tera::Context::new(),
        }
    }
}

// 渲染模版引擎函数
pub fn render_to_html(_tmpl: Vec<Template>) {
    unimplemented!()
}

pub fn get_templates() -> Option<Vec<Template>> {
    Some(vec![])
}
