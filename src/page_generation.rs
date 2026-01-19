use anyhow::{Context, Result};
use chrono::{Datelike, NaiveDate};
use once_cell::sync::Lazy;
use pulldown_cmark::{html, CodeBlockKind, Event, Options, Parser, Tag, TagEnd};
use pulldown_cmark_escape::escape_html;
use rayon::prelude::*;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashSet;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::Path;
use std::sync::Arc;
use syntect::html::{ClassStyle, ClassedHTMLGenerator};
use syntect::parsing::{SyntaxSet, SyntaxSetBuilder};
use tera::Tera;

// Load custom syntaxes once at startup
static CUSTOM_SYNTAXES: Lazy<Option<SyntaxSet>> = Lazy::new(|| {
    if Path::new("syntaxes").exists() {
        let mut builder = SyntaxSetBuilder::new();
        match builder.add_from_folder("syntaxes", true) {
            Ok(_) => Some(builder.build()),
            Err(e) => {
                eprintln!("Warning: Failed to load custom syntaxes: {}", e);
                None
            }
        }
    } else {
        None
    }
});

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostMetadata {
    pub date: String,
}

#[derive(Debug, Clone)]
pub struct Post {
    pub id: String,
    pub title: String,
    pub date: String,
    pub tags: Vec<String>,
    pub content_html: String,
}

#[derive(Debug, Clone)]
pub struct PostSummary {
    pub id: String,
    pub title: String,
    pub date: String,
    pub tags: Vec<String>,
}

pub fn read_all_posts(posts_dir: &Path) -> Result<Vec<Post>> {
    let entries: Vec<_> = fs::read_dir(posts_dir)?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("md"))
        .collect();

    let posts: Vec<Post> = entries
        .par_iter()
        .filter_map(|entry| read_post(&entry.path()).ok().flatten())
        .collect();

    Ok(posts)
}

fn read_post(path: &Path) -> Result<Option<Post>> {
    let content = fs::read_to_string(path)?;
    let id = path
        .file_stem()
        .and_then(|s| s.to_str())
        .context("Invalid filename")?
        .to_string();

    // Parse frontmatter
    let (metadata, markdown) = parse_frontmatter(&content)?;

    // Extract title from first H1
    let title = extract_title(&markdown);
    if title.is_empty() {
        return Ok(None);
    }

    // Extract tags from last line
    let tags = extract_tags(&content);

    // Remove first H1 from content
    let markdown_without_title = remove_first_h1(&markdown);

    // Convert markdown to HTML
    let content_html = markdown_to_html(&markdown_without_title, &tags);

    Ok(Some(Post {
        id,
        title,
        date: metadata.date,
        tags,
        content_html,
    }))
}

fn parse_frontmatter(content: &str) -> Result<(PostMetadata, String)> {
    let re = Regex::new(r"(?s)^---\s*\n(.*?)\n---\s*\n(.*)$").unwrap();

    if let Some(caps) = re.captures(content) {
        let yaml = caps.get(1).unwrap().as_str();
        let markdown = caps.get(2).unwrap().as_str();
        let metadata: PostMetadata = serde_yaml::from_str(yaml)?;
        Ok((metadata, markdown.to_string()))
    } else {
        anyhow::bail!("No frontmatter found");
    }
}

fn extract_title(markdown: &str) -> String {
    for line in markdown.lines() {
        if let Some(title) = line.strip_prefix("# ") {
            return title.trim().to_string();
        }
    }
    String::new()
}

fn extract_tags(content: &str) -> Vec<String> {
    let lines: Vec<&str> = content.lines().filter(|l| !l.trim().is_empty()).collect();
    if let Some(last_line) = lines.last() {
        let re = Regex::new(r"#([a-zA-Z0-9_-]+)").unwrap();
        return re
            .captures_iter(last_line)
            .map(|cap| cap[1].to_string())
            .collect();
    }
    Vec::new()
}

fn remove_first_h1(markdown: &str) -> String {
    let lines: Vec<&str> = markdown.lines().collect();
    let mut found_h1 = false;

    lines
        .into_iter()
        .filter(|line| {
            if !found_h1 && line.starts_with("# ") {
                found_h1 = true;
                false
            } else {
                true
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn find_syntax<'a>(syntax_set: &'a SyntaxSet, lang: &str) -> Option<&'a syntect::parsing::SyntaxReference> {
    // First try custom syntaxes
    if let Some(custom_set) = &*CUSTOM_SYNTAXES {
        if let Some(syntax) = custom_set.find_syntax_by_token(lang) {
            return Some(syntax);
        }
    }

    // Fall back to defaults
    syntax_set.find_syntax_by_token(lang)
}

fn markdown_to_html(markdown: &str, tags: &[String]) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(markdown, options);

    // Load syntax highlighting assets
    let syntax_set = SyntaxSet::load_defaults_newlines();

    let mut html_output = String::new();
    let mut in_code_block = false;
    let mut code_block_lang: Option<String> = None;
    let mut code_block_content = String::new();
    let mut in_table_head = false;
    let mut in_table = false;
    let mut table_body_started = false;

    for event in parser {
        match event {
            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(lang))) => {
                in_code_block = true;
                code_block_lang = if lang.is_empty() {
                    None
                } else {
                    Some(lang.to_string())
                };
                code_block_content.clear();
            }
            Event::End(TagEnd::CodeBlock) => {
                if in_code_block {
                    if let Some(lang) = &code_block_lang {
                        // Use syntect for syntax highlighting with CSS classes
                        // Try to find syntax in custom syntaxes first, then defaults
                        if let Some(syntax) = find_syntax(&syntax_set, lang) {
                            // Determine which syntax set to use for the generator
                            let (syntax_ref, syntax_set_ref) = if let Some(custom_set) = &*CUSTOM_SYNTAXES {
                                if let Some(custom_syntax) = custom_set.find_syntax_by_token(lang) {
                                    (custom_syntax, custom_set)
                                } else {
                                    (syntax, &syntax_set)
                                }
                            } else {
                                (syntax, &syntax_set)
                            };

                            let mut html_generator = ClassedHTMLGenerator::new_with_class_style(
                                syntax_ref,
                                syntax_set_ref,
                                ClassStyle::Spaced,
                            );

                            // Split by lines and add newlines back for syntect
                            for line in code_block_content.lines() {
                                let line_with_newline = format!("{}\n", line);
                                let _ = html_generator
                                    .parse_html_for_line_which_includes_newline(&line_with_newline);
                            }

                            let highlighted = html_generator.finalize();
                            // Convert syntect classes to Prism-compatible classes
                            let prism_html = convert_syntect_classes_to_prism(&highlighted);
                            html_output.push_str(&format!(
                                r#"<div class="remark-highlight"><pre class="language-{}"><code class="language-{}">{}</code></pre></div>"#,
                                lang, lang, prism_html
                            ));
                        } else {
                            // Language not found, use plain code block
                            let mut escaped = String::new();
                            escape_html(&mut escaped, &code_block_content).unwrap();
                            html_output.push_str(&format!(
                                r#"<pre><code class="language-{}">{}</code></pre>"#,
                                lang, escaped
                            ));
                        }
                    } else {
                        // No language specified - use language-text for consistent styling with Prism CSS
                        let mut escaped = String::new();
                        escape_html(&mut escaped, &code_block_content).unwrap();
                        html_output.push_str(&format!(
                            r#"<div class="remark-highlight"><pre class="language-text"><code class="language-text">{}</code></pre></div>"#,
                            escaped
                        ));
                    }
                    in_code_block = false;
                    code_block_lang = None;
                    code_block_content.clear();
                }
            }
            Event::Text(text) => {
                if in_code_block {
                    code_block_content.push_str(&text);
                } else {
                    let mut escaped = String::new();
                    escape_html(&mut escaped, &text).unwrap();
                    html_output.push_str(&escaped);
                }
            }
            Event::Code(code) => {
                let mut escaped = String::new();
                escape_html(&mut escaped, &code).unwrap();
                html_output.push_str(&format!("<code>{}</code>", escaped));
            }
            Event::Start(Tag::Table(_)) => {
                in_table = true;
                table_body_started = false;
                html_output.push_str(r#"<div class="table-wrapper"><table>"#);
            }
            Event::End(TagEnd::Table) => {
                if table_body_started {
                    html_output.push_str("</tbody>");
                    table_body_started = false;
                }
                in_table = false;
                html_output.push_str("</table></div>");
            }
            Event::Start(Tag::TableHead) => {
                in_table_head = true;
                html_output.push_str("<thead>");
            }
            Event::End(TagEnd::TableHead) => {
                in_table_head = false;
                html_output.push_str("</thead>");
            }
            Event::Start(Tag::TableRow) => {
                if in_table && !in_table_head && !table_body_started {
                    html_output.push_str("<tbody>");
                    table_body_started = true;
                }
                html_output.push_str("<tr>");
            }
            Event::End(TagEnd::TableRow) => {
                html_output.push_str("</tr>");
            }
            Event::Start(Tag::TableCell) => {
                if in_table_head {
                    html_output.push_str("<th>");
                } else {
                    html_output.push_str("<td>");
                }
            }
            Event::End(TagEnd::TableCell) => {
                if in_table_head {
                    html_output.push_str("</th>");
                } else {
                    html_output.push_str("</td>");
                }
            }
            _ => {
                // For all other events, use the default HTML rendering
                let mut temp = String::new();
                html::push_html(&mut temp, std::iter::once(event));
                html_output.push_str(&temp);
            }
        }
    }

    // Add IDs to headings
    html_output = add_heading_ids(&html_output);

    // Convert hashtags to links
    html_output = convert_hashtags_to_links(&html_output, tags);

    html_output
}

fn convert_syntect_classes_to_prism(html: &str) -> String {
    // Map syntect scope classes to Prism token classes
    // Syntect uses TextMate scope names, we need to convert to Prism's simpler classes

    let result = html.to_string();

    // Replace all class attributes with Prism-compatible ones
    let re = Regex::new(r#"<span class="([^"]+)">"#).unwrap();
    let result = re.replace_all(&result, |caps: &regex::Captures| {
        let class_content = &caps[1];

        // Determine the appropriate Prism token class based on syntect scopes
        let token_class = if class_content.contains("keyword") {
            "token keyword"
        } else if class_content.contains("string") {
            "token string"
        } else if class_content.contains("comment") {
            "token comment"
        } else if class_content.contains("constant") && class_content.contains("numeric") {
            "token number"
        } else if class_content.contains("function") || class_content.contains("support function") {
            "token function"
        } else if class_content.contains("operator") {
            "token operator"
        } else if class_content.contains("punctuation") {
            "token punctuation"
        } else if class_content.contains("support") || class_content.contains("builtin") {
            "token builtin"
        } else if class_content.contains("variable") {
            "token variable"
        } else {
            // For any other scope, just return empty span (no class needed)
            return "".to_string();
        };

        format!(r#"<span class="{}">"#, token_class)
    });

    result.to_string()
}

fn add_heading_ids(html: &str) -> String {
    let re = Regex::new(r"<h([1-6])>(.*?)</h[1-6]>").unwrap();
    re.replace_all(html, |caps: &regex::Captures| {
        let level = &caps[1];
        let content = &caps[2];
        let plain_text = strip_html_tags(content);
        let id = plain_text
            .to_lowercase()
            .chars()
            .map(|c| if c.is_alphanumeric() { c } else { '-' })
            .collect::<String>()
            .split('-')
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join("-");
        format!(r#"<h{} id="{}">{}</h{}>"#, level, id, content, level)
    })
    .to_string()
}

fn strip_html_tags(html: &str) -> String {
    let re = Regex::new(r"<[^>]*>").unwrap();
    re.replace_all(html, "").to_string()
}

fn convert_hashtags_to_links(html: &str, tags: &[String]) -> String {
    if tags.is_empty() {
        return html.to_string();
    }

    let tag_pattern = tags
        .iter()
        .map(|tag| format!("#{}", regex::escape(tag)))
        .collect::<Vec<_>>()
        .join("|");

    let re = Regex::new(&format!(r"<p>((?:(?:{})\s*)+)</p>", tag_pattern)).unwrap();

    re.replace_all(html, |caps: &regex::Captures| {
        let hashtags_text = &caps[1];
        let hashtag_re = Regex::new(r"#([a-zA-Z0-9_-]+)").unwrap();
        let links = hashtag_re.replace_all(hashtags_text, |c: &regex::Captures| {
            let tag = &c[1];
            format!(
                r#"<a href="/index.html#{}">{}</a>"#,
                tag,
                format!("#{}", tag)
            )
        });
        format!(r#"<p class="post-hashtags">{}</p>"#, links)
    })
    .to_string()
}

pub fn extract_all_tags(posts: &[Post]) -> Vec<String> {
    let mut tags = HashSet::new();
    for post in posts {
        for tag in &post.tags {
            tags.insert(tag.clone());
        }
    }
    let mut tags: Vec<String> = tags.into_iter().collect();
    tags.sort();
    tags
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

pub fn format_date(date_str: &str) -> String {
    if let Ok(date) = NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
        date.format("%B %-d, %Y").to_string()
    } else {
        date_str.to_string()
    }
}

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

    let has_toc = post.content_html.contains("<h1")
        || post.content_html.contains("<h2")
        || post.content_html.contains("<h3");

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

fn read_inline_css() -> Result<String> {
    use lightningcss::stylesheet::{ParserOptions, PrinterOptions, StyleSheet};

    let global = fs::read_to_string("website/styles/global.css")?;
    let utils = fs::read_to_string("website/styles/utils.module.css")?;
    let layout = fs::read_to_string("website/styles/layout.module.css")?;

    // Remove :global() wrappers more carefully
    let layout = Regex::new(r":global\(([^)]+)\)")
        .unwrap()
        .replace_all(&layout, "$1")
        .to_string();

    let combined = format!("{}{}{}", global, layout, utils);

    // Minify the CSS
    let stylesheet = StyleSheet::parse(&combined, ParserOptions::default())
        .map_err(|e| anyhow::anyhow!("CSS parse error: {:?}", e))?;

    let minified = stylesheet
        .to_css(PrinterOptions {
            minify: true,
            ..Default::default()
        })
        .map_err(|e| anyhow::anyhow!("CSS minify error: {:?}", e))?;

    Ok(minified.code)
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
