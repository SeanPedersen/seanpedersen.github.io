use anyhow::Result;
use rayon::prelude::*;
use serde_json::json;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::Path;
use std::sync::Arc;
use std::time::Instant;
use tera::{Tera, Value};

use crate::page_generation::{extract_headings, format_date, strip_html_tags, Post, PostSummary};

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

pub fn build_post_pages(out_dir: &Path, posts: &Arc<Vec<Post>>) -> Result<()> {
    let start = Instant::now();

    let posts_out_dir = out_dir.join("posts");
    fs::create_dir_all(&posts_out_dir)?;

    generate_all_post_pages(&posts_out_dir, posts)?;

    println!(
        "✓ Generated {} post pages in {:.2}s",
        posts.len(),
        start.elapsed().as_secs_f64()
    );
    Ok(())
}

pub fn get_related_posts(
    all_posts: &[Post],
    current_id: &str,
    tags: &[String],
    limit: usize,
) -> Vec<PostSummary> {
    if tags.is_empty() {
        return Vec::new();
    }

    let first_tag = &tags[0];
    let mut related: Vec<_> = all_posts
        .iter()
        .filter(|p| p.id != current_id && p.tags.contains(first_tag))
        .take(10)
        .map(|p| PostSummary {
            id: p.id.clone(),
            title: p.title.clone(),
            date: p.date.clone(),
            tags: p.tags.clone(),
        })
        .collect();

    if related.len() <= limit {
        return related;
    }

    // Take first 2 and one from middle
    let mut result = related.drain(0..2).collect::<Vec<_>>();
    if !related.is_empty() {
        let idx = (related.len() as f64 * 0.5) as usize;
        result.push(related[idx].clone());
    }
    result
}

pub fn generate_post_page(out_dir: &Path, post: &Post, related: &[PostSummary]) -> Result<()> {
    let mut tera = Tera::new("website/post/**/*")?;
    tera.register_function("inline_css", inline_css_placeholder);

    // Detect if page has code blocks
    let has_code_blocks = post.content_html.contains("<pre");

    let excerpt = strip_html_tags(&post.content_html)
        .chars()
        .take(160)
        .collect::<String>();

    let title_id = post
        .title
        .to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-");

    // Extract headings for TOC
    let headings = extract_headings(&post.content_html);
    let has_toc = !headings.is_empty();

    let keywords = post.tags.join(", ");

    // Prepare related posts data
    let related_data: Vec<serde_json::Value> = related
        .iter()
        .map(|rel| {
            json!({
                "id": rel.id,
                "title": rel.title,
                "formatted_date": format_date(&rel.date),
            })
        })
        .collect();

    let mut context = tera::Context::new();
    context.insert("post_title", &post.title);
    context.insert("post_id", &post.id);
    context.insert("excerpt", &excerpt);
    context.insert("keywords", &keywords);
    context.insert("has_toc", &has_toc);
    context.insert("headings", &headings);
    context.insert("has_code_blocks", &has_code_blocks);
    context.insert("title_id", &title_id);
    context.insert("formatted_date", &format_date(&post.date));
    context.insert("content_html", &post.content_html);
    context.insert("related_posts", &related_data);
    context.insert("post_title_json", &serde_json::to_string(&post.title)?);
    context.insert("title_id_json", &serde_json::to_string(&title_id)?);

    let html = tera.render("post.html", &context)?;

    let mut file = BufWriter::new(File::create(out_dir.join(format!("{}.html", post.id)))?);
    write!(file, "{}", html)?;

    Ok(())
}

pub fn generate_all_post_pages(posts_out_dir: &Path, posts: &Arc<Vec<Post>>) -> Result<()> {
    let post_ids: Vec<String> = posts.iter().map(|p| p.id.clone()).collect();
    let completed = std::sync::atomic::AtomicUsize::new(0);

    post_ids.par_iter().try_for_each(|post_id| -> Result<()> {
        let post = posts.iter().find(|p| &p.id == post_id).unwrap();
        let related = get_related_posts(posts, post_id, &post.tags, 3);
        generate_post_page(posts_out_dir, post, &related)?;
        completed.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        Ok(())
    })?;

    Ok(())
}
