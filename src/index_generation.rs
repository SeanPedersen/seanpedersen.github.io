use anyhow::Result;
use chrono::Datelike;
use serde_json::json;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::sync::Arc;
use std::time::Instant;
use tera::{Tera, Value};

use crate::page_generation::{extract_all_tags, Post, PostSummary};

/// Tera function that outputs a placeholder for CSS inlining.
/// Usage in template: {{ inline_css(path="/styles/global.css") }}
/// The placeholder is replaced with actual CSS content during optimization.
fn inline_css_placeholder(args: &HashMap<String, Value>) -> tera::Result<Value> {
    let path = args
        .get("path")
        .and_then(|v| v.as_str())
        .ok_or_else(|| tera::Error::msg("inline_css requires a 'path' argument"))?;
    Ok(Value::String(format!("<!-- INLINE_CSS:{} -->", path)))
}

pub fn build_index_page(out_dir: &Path, posts: &Arc<Vec<Post>>) -> Result<()> {
    let start = Instant::now();

    let post_summaries: Vec<PostSummary> = posts
        .iter()
        .map(|p| PostSummary {
            id: p.id.clone(),
            title: p.title.clone(),
            date: p.date.clone(),
            tags: p.tags.clone(),
        })
        .collect();

    let all_tags = extract_all_tags(posts);
    generate_index_page(out_dir, &post_summaries, &all_tags)?;

    println!(
        "✓ Generated index.html in {:.2}s",
        start.elapsed().as_secs_f64()
    );
    Ok(())
}

pub fn generate_index_page(out_dir: &Path, posts: &[PostSummary], tags: &[String]) -> Result<()> {
    let mut tera = Tera::new("website/html-templates/**/*")?;
    tera.register_function("inline_css", inline_css_placeholder);

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
    context.insert("tags", tags);
    context.insert("posts", &posts_data);
    context.insert("year", &year);

    let html = tera.render("index.html", &context)?;

    let mut file = BufWriter::new(File::create(out_dir.join("index.html"))?);
    write!(file, "{}", html)?;

    Ok(())
}
