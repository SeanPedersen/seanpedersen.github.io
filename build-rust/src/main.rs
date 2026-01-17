use anyhow::{Context, Result};
use chrono::{Datelike, NaiveDate};
use pulldown_cmark::{html, CodeBlockKind, Event, Options, Parser, Tag, TagEnd};
use pulldown_cmark_escape::escape_html;
use rayon::prelude::*;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashSet;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Instant;
use syntect::html::{ClassStyle, ClassedHTMLGenerator};
use syntect::parsing::SyntaxSet;
use tera::Tera;

// Optimization imports
use lightningcss::stylesheet::{ParserOptions, PrinterOptions, StyleSheet};
use minify_html::{minify, Cfg};
use minify_js::minify as minify_js_code;
use minify_js::{Session, TopLevelMode};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PostMetadata {
    date: String,
}

#[derive(Debug, Clone)]
struct Post {
    id: String,
    title: String,
    date: String,
    tags: Vec<String>,
    content_html: String,
}

#[derive(Debug, Clone)]
struct PostSummary {
    id: String,
    title: String,
    date: String,
    tags: Vec<String>,
}

fn main() -> Result<()> {
    let total_start = Instant::now();

    println!("Starting static site generation...\n");

    let out_dir = PathBuf::from("out");

    // Clean output directory
    if out_dir.exists() {
        println!("Cleaning output directory...");
        fs::remove_dir_all(&out_dir)?;
    }

    // Create output directory
    fs::create_dir_all(&out_dir)?;
    println!("Created output directory: out/\n");

    // Read posts
    println!("Reading posts...");
    let posts_dir = PathBuf::from("posts");
    let mut posts = read_all_posts(&posts_dir)?;

    // Sort by date descending
    posts.sort_by(|a, b| b.date.cmp(&a.date));

    let all_tags = extract_all_tags(&posts);
    println!("Found {} posts and {} tags\n", posts.len(), all_tags.len());

    // Generate index page
    println!("Generating index page...");
    let post_summaries: Vec<PostSummary> = posts
        .iter()
        .map(|p| PostSummary {
            id: p.id.clone(),
            title: p.title.clone(),
            date: p.date.clone(),
            tags: p.tags.clone(),
        })
        .collect();

    generate_index_page(&out_dir, &post_summaries, &all_tags)?;
    println!("✓ Generated index.html\n");

    // Generate post pages in parallel
    println!("Generating post pages...");
    let posts_out_dir = out_dir.join("posts");
    fs::create_dir_all(&posts_out_dir)?;

    let total_posts = posts.len();
    let posts_arc = Arc::new(posts);
    let post_ids: Vec<String> = posts_arc.iter().map(|p| p.id.clone()).collect();

    let start_posts = Instant::now();
    let completed: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

    post_ids.par_iter().try_for_each(|post_id| -> Result<()> {
        let post = posts_arc.iter().find(|p| &p.id == post_id).unwrap();
        let related = get_related_posts(&posts_arc, post_id, &post.tags, 3);
        generate_post_page(&posts_out_dir, post, &related)?;

        let done = completed.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;
        if done % 10 == 0 || done == total_posts {
            println!("  [{}/{}] posts completed...", done, total_posts);
        }
        Ok(())
    })?;

    let posts_duration = start_posts.elapsed().as_secs_f64();
    println!(
        "✓ Generated {} post pages in {:.2}s\n",
        total_posts, posts_duration
    );

    // Copy static assets
    println!("Copying static assets...");
    copy_static_assets(&out_dir)?;

    // Generate RSS feed (before optimization to get clean, unminified HTML)
    println!("\nGenerating RSS feed...");
    let rss_start = Instant::now();
    generate_rss(&out_dir, &posts_arc)?;
    let rss_duration = rss_start.elapsed().as_secs_f64();
    println!("✓ Generated RSS feed in {:.2}s", rss_duration);

    // Optimize assets
    println!("\nOptimizing assets...");
    let optimize_start = Instant::now();
    optimize_assets(&out_dir)?;
    let optimize_duration = optimize_start.elapsed().as_secs_f64();
    println!("✓ Optimized assets in {:.2}s\n", optimize_duration);

    let total_duration = total_start.elapsed().as_secs_f64();
    println!("\n✅ Build complete in {:.2}s!", total_duration);
    println!("   Output directory: {}", out_dir.display());
    println!("   Total files: {} HTML pages + RSS feed", total_posts + 1);

    Ok(())
}

fn read_all_posts(posts_dir: &Path) -> Result<Vec<Post>> {
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
                        if let Some(syntax) = syntax_set.find_syntax_by_token(lang) {
                            let mut html_generator = ClassedHTMLGenerator::new_with_class_style(
                                syntax,
                                &syntax_set,
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

fn extract_all_tags(posts: &[Post]) -> Vec<String> {
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

fn get_related_posts(
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

fn format_date(date_str: &str) -> String {
    if let Ok(date) = NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
        date.format("%B %-d, %Y").to_string()
    } else {
        date_str.to_string()
    }
}

fn generate_index_page(out_dir: &Path, posts: &[PostSummary], tags: &[String]) -> Result<()> {
    let tera = Tera::new("build-rust/templates/**/*")?;

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

fn generate_post_page(out_dir: &Path, post: &Post, related: &[PostSummary]) -> Result<()> {
    let tera = Tera::new("build-rust/templates/**/*")?;

    let css = read_inline_css()?;
    let prism_css = fs::read_to_string("node_modules/prismjs/themes/prism-tomorrow.css")?;

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
    let global = fs::read_to_string("styles/global.css")?;
    let utils = fs::read_to_string("styles/utils.module.css")?;
    let layout = fs::read_to_string("styles/layout.module.css")?;

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

fn copy_static_assets(out_dir: &Path) -> Result<()> {
    // Copy images
    let images_src = Path::new("public/images");
    if images_src.exists() {
        let images_dest = out_dir.join("images");
        copy_dir_recursive(images_src, &images_dest)?;
        println!("  ✓ Copied images/");
    }

    // Copy favicon
    let favicon_src = Path::new("public/favicon.ico");
    if favicon_src.exists() {
        fs::copy(favicon_src, out_dir.join("favicon.ico"))?;
        println!("  ✓ Copied favicon.ico");
    }

    // Create styles directory and copy CSS
    let styles_out = out_dir.join("styles");
    fs::create_dir_all(&styles_out)?;

    fs::copy("styles/global.css", styles_out.join("global.css"))?;
    println!("  ✓ Copied global.css");

    fs::copy(
        "node_modules/prismjs/themes/prism-tomorrow.css",
        styles_out.join("prism-tomorrow.css"),
    )?;
    println!("  ✓ Copied prism-tomorrow.css");

    fs::copy("styles/utils.module.css", styles_out.join("utils.css"))?;
    println!("  ✓ Copied utils.css");

    let layout_css = fs::read_to_string("styles/layout.module.css")?;
    let layout_css = Regex::new(r":global\(([^)]+)\)")
        .unwrap()
        .replace_all(&layout_css, "$1")
        .to_string();
    fs::write(styles_out.join("layout.css"), &layout_css)?;
    println!("  ✓ Copied layout.css");

    // Merge layout into global
    let mut global = fs::read_to_string(styles_out.join("global.css"))?;
    global.push_str("\n\n/* Layout styles */\n");
    global.push_str(&layout_css);
    fs::write(styles_out.join("global.css"), global)?;
    println!("  ✓ Merged layout.css into global.css");

    // Copy JS files
    let js_out = out_dir.join("js");
    fs::create_dir_all(&js_out)?;
    println!("  ✓ Created js/ directory");

    let js_src = Path::new("public/js");
    if js_src.exists() {
        copy_dir_recursive(js_src, &js_out)?;
        println!("  ✓ Copied JavaScript files");
    }

    Ok(())
}

fn copy_dir_recursive(src: &Path, dest: &Path) -> Result<()> {
    fs::create_dir_all(dest)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();
        let dest_path = dest.join(entry.file_name());

        if path.is_dir() {
            copy_dir_recursive(&path, &dest_path)?;
        } else {
            fs::copy(&path, &dest_path)?;
        }
    }
    Ok(())
}

// ============================================================================
// Asset Optimization
// ============================================================================

fn optimize_assets(out_dir: &Path) -> Result<()> {
    // Collect all files to optimize
    let mut css_files = Vec::new();
    let mut js_files = Vec::new();
    let mut html_files = Vec::new();
    let mut image_files = Vec::new();

    collect_files(
        out_dir,
        &mut css_files,
        &mut js_files,
        &mut html_files,
        &mut image_files,
    )?;

    let total_files = css_files.len() + js_files.len() + html_files.len() + image_files.len();
    println!("  Found {} files to optimize", total_files);

    // Optimize in parallel
    let css_count = AtomicUsize::new(0);
    let js_count = AtomicUsize::new(0);
    let html_count = AtomicUsize::new(0);
    let img_count = AtomicUsize::new(0);

    rayon::scope(|s| {
        s.spawn(|_| {
            css_files.par_iter().for_each(|path| {
                if minify_css_file(path).is_ok() {
                    css_count.fetch_add(1, Ordering::Relaxed);
                }
            });
        });

        s.spawn(|_| {
            js_files.par_iter().for_each(|path| {
                if minify_js_file(path).is_ok() {
                    js_count.fetch_add(1, Ordering::Relaxed);
                }
            });
        });

        s.spawn(|_| {
            html_files.par_iter().for_each(|path| {
                if minify_html_file(path).is_ok() {
                    html_count.fetch_add(1, Ordering::Relaxed);
                }
            });
        });

        s.spawn(|_| {
            image_files.par_iter().for_each(|path| {
                if optimize_image_file(path).is_ok() {
                    img_count.fetch_add(1, Ordering::Relaxed);
                }
            });
        });
    });

    println!("  ✓ CSS: {} files", css_count.load(Ordering::Relaxed));
    println!("  ✓ JS: {} files", js_count.load(Ordering::Relaxed));
    println!("  ✓ HTML: {} files", html_count.load(Ordering::Relaxed));
    println!("  ✓ Images: {} files", img_count.load(Ordering::Relaxed));

    Ok(())
}

fn collect_files(
    dir: &Path,
    css_files: &mut Vec<PathBuf>,
    js_files: &mut Vec<PathBuf>,
    html_files: &mut Vec<PathBuf>,
    image_files: &mut Vec<PathBuf>,
) -> Result<()> {
    if !dir.is_dir() {
        return Ok(());
    }

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            collect_files(&path, css_files, js_files, html_files, image_files)?;
        } else if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
            match ext {
                "css" => css_files.push(path),
                "js" => js_files.push(path),
                "html" => html_files.push(path),
                "png" | "jpg" | "jpeg" | "webp" => image_files.push(path),
                _ => {}
            }
        }
    }

    Ok(())
}

fn minify_css_file(path: &Path) -> Result<()> {
    let css = fs::read_to_string(path)?;

    let stylesheet = StyleSheet::parse(&css, ParserOptions::default())
        .map_err(|e| anyhow::anyhow!("CSS parse error: {:?}", e))?;

    let minified = stylesheet
        .to_css(PrinterOptions {
            minify: true,
            ..Default::default()
        })
        .map_err(|e| anyhow::anyhow!("CSS minify error: {:?}", e))?;

    fs::write(path, minified.code)?;
    Ok(())
}

fn minify_js_file(path: &Path) -> Result<()> {
    let js = fs::read_to_string(path)?;

    // Use minify-js for proper JS minification
    let session = Session::new();
    let mut output = Vec::new();

    minify_js_code(&session, TopLevelMode::Global, js.as_bytes(), &mut output)
        .map_err(|e| anyhow::anyhow!("JS minify error: {:?}", e))?;

    let mut minified = String::from_utf8(output).unwrap_or_else(|_| js.clone());

    // Fix minify-js bug: it over-escapes newlines in string literals
    // Replace `\\n` with `\n` in string contexts
    // This is a workaround for: split('\\n') -> split('\n')
    minified = minified.replace(r#"split(`\\n`)"#, r#"split(`\n`)"#);
    minified = minified.replace(r#"split('\\n')"#, r#"split('\n')"#);
    minified = minified.replace(r#"split("\\n")"#, r#"split("\n")"#);

    fs::write(path, minified.as_bytes())?;
    Ok(())
}

fn minify_html_file(path: &Path) -> Result<()> {
    let html = fs::read(path)?;

    let cfg = Cfg {
        do_not_minify_doctype: true,
        ensure_spec_compliant_unquoted_attribute_values: true,
        keep_html_and_head_opening_tags: true,
        minify_css: false, // Don't minify CSS inside HTML - lightningcss already did it
        minify_js: true,
        ..Default::default()
    };

    let minified = minify(&html, &cfg);
    fs::write(path, minified)?;
    Ok(())
}

fn optimize_image_file(_path: &Path) -> Result<()> {
    // Images are already optimized at the source level using Sharp.
    // No need to re-optimize during build - just copy them as-is.
    Ok(())
}

// ============================================================================
// RSS Generation
// ============================================================================

fn generate_rss(out_dir: &Path, posts: &Arc<Vec<Post>>) -> Result<()> {
    let rss_path = out_dir.join("rss.xml");
    let mut file = BufWriter::new(File::create(rss_path)?);

    let now = chrono::Utc::now();
    let rss_date = now.to_rfc2822();

    // Write RSS header (match Node.js format with 2-space indentation)
    write!(
        file,
        r#"<?xml version="1.0" encoding="UTF-8"?>
<rss version="2.0" xmlns:atom="http://www.w3.org/2005/Atom" xmlns:content="http://purl.org/rss/1.0/modules/content/">
  <channel>
    <title>{}</title>
    <description>Another place for thought infusion</description>
    <link>https://seanpedersen.github.io</link>
    <atom:link href="https://seanpedersen.github.io/rss.xml" rel="self" type="application/rss+xml" />
    <language>en</language>
    <pubDate>{}</pubDate>
    <lastBuildDate>{}</lastBuildDate>
    <generator>Custom RSS Generator</generator>
    <managingEditor>Sean Pedersen</managingEditor>
    <webMaster>Sean Pedersen</webMaster>
    <ttl>60</ttl>
"#,
        escape_xml("Sean's Blog"),
        rss_date,
        rss_date
    )?;

    // Write each post
    for post in posts.iter() {
        let post_url = format!("https://seanpedersen.github.io/posts/{}", post.id);
        let post_date = NaiveDate::parse_from_str(&post.date, "%Y-%m-%d")
            .ok()
            .and_then(|d| d.and_hms_opt(0, 0, 0))
            .and_then(|dt| {
                let utc =
                    chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(dt, chrono::Utc);
                Some(utc.format("%a, %d %b %Y %H:%M:%S GMT").to_string())
            })
            .unwrap_or_else(|| post.date.clone());

        write!(file, "    <item>\n      <title>{}</title>\n      <link>{}</link>\n      <guid isPermaLink=\"true\">{}</guid>\n      <pubDate>{}</pubDate>\n      <author>Sean Pedersen</author>\n",
            escape_xml(&post.title), post_url, post_url, post_date)?;

        // Write tags as categories
        for tag in &post.tags {
            write!(file, "      <category>{}</category>\n", escape_xml(tag))?;
        }

        // Read minified HTML content from the optimized post file
        let html_path = out_dir.join("posts").join(format!("{}.html", post.id));
        let html_content = fs::read_to_string(&html_path).unwrap_or_default();

        // Extract content from markdown-content div
        let mut content = extract_content_from_html(&html_content);

        // Clean up content for RSS (remove syntax highlighting spans)
        content = clean_content_for_rss(&content);

        // Write content
        write!(
            file,
            "      <content:encoded><![CDATA[{}]]></content:encoded>\n    </item>\n",
            content
        )?;
    }

    write!(file, "  </channel>\n</rss>")?;

    Ok(())
}

fn extract_content_from_html(html: &str) -> String {
    // Find the markdown-content div (class can be anywhere in the tag)
    let re = Regex::new(r#"<div[^>]*\bclass=["']?markdown-content["']?[^>]*>"#).unwrap();

    if let Some(mat) = re.find(html) {
        let content_start = mat.end();

        // Find the matching closing </div> before <footer
        // We need to count div depth
        let mut depth = 1;
        let mut pos = content_start;
        let bytes = html.as_bytes();

        while pos < bytes.len() && depth > 0 {
            if pos + 5 <= bytes.len() && &bytes[pos..pos + 5] == b"<div " {
                depth += 1;
                pos += 5;
            } else if pos + 4 <= bytes.len() && &bytes[pos..pos + 4] == b"<div>" {
                depth += 1;
                pos += 4;
            } else if pos + 6 <= bytes.len() && &bytes[pos..pos + 6] == b"</div>" {
                depth -= 1;
                if depth == 0 {
                    return html[content_start..pos].to_string();
                }
                pos += 6;
            } else {
                pos += 1;
            }
        }
    }

    String::new()
}

fn clean_content_for_rss(html: &str) -> String {
    let mut result = html.to_string();

    // Remove all <span class="token ..."> tags but keep their text content
    let span_re = Regex::new(r#"<span[^>]*class="token[^"]*"[^>]*>"#).unwrap();
    result = span_re.replace_all(&result, "").to_string();

    // Remove closing </span> tags
    result = result.replace("</span>", "");

    // Replace remark-highlight divs with just the code block
    // Match: <div class="remark-highlight"><pre class="language-X"><code class="language-X">
    // Replace with: <pre><code>
    let remark_re = Regex::new(r#"<div class="remark-highlight"><pre class="language-[^"]*"><code class="language-[^"]*">"#).unwrap();
    result = remark_re.replace_all(&result, "<pre><code>").to_string();

    // Close the remark-highlight div properly: </code></pre></div>
    result = result.replace("</code></pre></div>", "</code></pre>");

    result
}

fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}
