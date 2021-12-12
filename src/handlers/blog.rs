use std::collections::HashMap;

use askama::Template;
use axum::{extract::Path, response::IntoResponse};

use crate::parsers::{
    markdown::{markdown_parser, meta_parser},
    meta::Meta,
};

use super::HtmlTemplate;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    markdown: String,
    meta: Meta,
}

pub async fn root() -> impl IntoResponse {
    let mark = markdown_parser("test");
    let meta = meta_parser("test");
    let template = IndexTemplate {
        markdown: mark,
        meta,
    };
    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "index.html")]
struct BlogTemplate {
    markdown: String,
    meta: Meta,
}

pub async fn blog(Path(params): Path<HashMap<String, String>>) -> impl IntoResponse {
    let blog = params.get("blog").unwrap();

    let mark = markdown_parser(blog);
    let meta = meta_parser(blog);
    let template = BlogTemplate {
        markdown: mark,
        meta,
    };
    HtmlTemplate(template)
}