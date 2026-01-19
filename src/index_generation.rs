use anyhow::Result;
use chrono::Datelike;
use serde_json::json;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use tera::Tera;

use crate::page_generation::{read_inline_css, PostSummary};

pub fn generate_index_page(out_dir: &Path, posts: &[PostSummary], tags: &[String]) -> Result<()> {
    let tera = Tera::new("website/html-templates/**/*")?;

    let css = read_inline_css()?;
    let year = chrono::Local::now().year();

    // Prepare posts data with tags_json
    let posts_data: Vec<serde_json::Value> = posts
        .iter()
        .map(|post| {
            json!({
                "id": post.id,
                "title": post.title,
                "tags": post.tags,
                "tags_json": serde_json::to_string(&post.tags).unwrap_or_default(),
            })
        })
        .collect();

    let mut context = tera::Context::new();
    context.insert("css", &css);
    context.insert("tags", tags);
    context.insert("posts", &posts_data);
    context.insert("year", &year);

    let html = tera.render("index.html", &context)?;

    let mut file = BufWriter::new(File::create(out_dir.join("index.html"))?);
    write!(file, "{}", html)?;

    Ok(())
}
