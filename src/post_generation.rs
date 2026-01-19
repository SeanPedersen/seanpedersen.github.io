use anyhow::Result;
use rayon::prelude::*;
use serde_json::json;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::Path;
use std::sync::Arc;
use tera::Tera;

use crate::page_generation::{
    extract_headings, format_date, generate_toc_html, read_inline_css, strip_html_tags, Post,
    PostSummary,
};

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
    let tera = Tera::new("website/html-templates/**/*")?;

    let css = read_inline_css()?;
    let prism_css = fs::read_to_string("website/styles/prism-tomorrow.css")?;

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

    // Extract headings and generate TOC HTML
    let headings = extract_headings(&post.content_html);
    let has_toc = !headings.is_empty();
    let toc_html = if has_toc {
        generate_toc_html(&headings, &post.title, &title_id)
    } else {
        String::new()
    };

    // Detect if page has code blocks
    let has_code_blocks = post.content_html.contains("<pre");

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
    context.insert("css", &css);
    context.insert("prism_css", &prism_css);
    context.insert("has_toc", &has_toc);
    context.insert("toc_html", &toc_html);
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

pub fn generate_all_post_pages(
    posts_out_dir: &Path,
    posts: &Arc<Vec<Post>>,
    total_posts: usize,
) -> Result<()> {
    let post_ids: Vec<String> = posts.iter().map(|p| p.id.clone()).collect();
    let completed = std::sync::atomic::AtomicUsize::new(0);

    post_ids.par_iter().try_for_each(|post_id| -> Result<()> {
        let post = posts.iter().find(|p| &p.id == post_id).unwrap();
        let related = get_related_posts(posts, post_id, &post.tags, 3);
        generate_post_page(posts_out_dir, post, &related)?;

        let done = completed.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;
        if done % 10 == 0 || done == total_posts {
            println!("  [{}/{}] posts completed...", done, total_posts);
        }
        Ok(())
    })?;

    Ok(())
}
