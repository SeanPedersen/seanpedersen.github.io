use anyhow::{Context, Result};
use chrono::{Datelike, NaiveDate};
use pulldown_cmark::{html, Options, Parser};
use rayon::prelude::*;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;

// Optimization imports
use lightningcss::stylesheet::{ParserOptions, PrinterOptions, StyleSheet};
use minify_html::{Cfg, minify};
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
    let post_summaries: Vec<PostSummary> = posts.iter().map(|p| PostSummary {
        id: p.id.clone(),
        title: p.title.clone(),
        date: p.date.clone(),
        tags: p.tags.clone(),
    }).collect();

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
    println!("✓ Generated {} post pages in {:.2}s\n", total_posts, posts_duration);

    // Copy static assets
    println!("Copying static assets...");
    copy_static_assets(&out_dir)?;

    // Optimize assets
    println!("\nOptimizing assets...");
    let optimize_start = Instant::now();
    optimize_assets(&out_dir)?;
    let optimize_duration = optimize_start.elapsed().as_secs_f64();
    println!("✓ Optimized assets in {:.2}s\n", optimize_duration);

    // Generate RSS feed (after optimization so we get minified HTML)
    println!("Generating RSS feed...");
    let rss_start = Instant::now();
    generate_rss(&out_dir, &posts_arc)?;
    let rss_duration = rss_start.elapsed().as_secs_f64();
    println!("✓ Generated RSS feed in {:.2}s\n", rss_duration);

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

    let posts: Vec<Post> = entries.par_iter()
        .filter_map(|entry| {
            read_post(&entry.path()).ok().flatten()
        })
        .collect();

    Ok(posts)
}

fn read_post(path: &Path) -> Result<Option<Post>> {
    let content = fs::read_to_string(path)?;
    let id = path.file_stem()
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
        return re.captures_iter(last_line)
            .map(|cap| cap[1].to_string())
            .collect();
    }
    Vec::new()
}

fn remove_first_h1(markdown: &str) -> String {
    let lines: Vec<&str> = markdown.lines().collect();
    let mut found_h1 = false;

    lines.into_iter()
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

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    // Add IDs to headings
    html_output = add_heading_ids(&html_output);

    // Convert hashtags to links
    html_output = convert_hashtags_to_links(&html_output, tags);

    html_output
}

fn add_heading_ids(html: &str) -> String {
    let re = Regex::new(r"<h([1-6])>(.*?)</h[1-6]>").unwrap();
    re.replace_all(html, |caps: &regex::Captures| {
        let level = &caps[1];
        let content = &caps[2];
        let plain_text = strip_html_tags(content);
        let id = plain_text.to_lowercase()
            .chars()
            .map(|c| if c.is_alphanumeric() { c } else { '-' })
            .collect::<String>()
            .split('-')
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join("-");
        format!(r#"<h{} id="{}">{}</h{}>"#, level, id, content, level)
    }).to_string()
}

fn strip_html_tags(html: &str) -> String {
    let re = Regex::new(r"<[^>]*>").unwrap();
    re.replace_all(html, "").to_string()
}

fn convert_hashtags_to_links(html: &str, tags: &[String]) -> String {
    if tags.is_empty() {
        return html.to_string();
    }

    let tag_pattern = tags.iter()
        .map(|tag| format!("#{}", regex::escape(tag)))
        .collect::<Vec<_>>()
        .join("|");

    let re = Regex::new(&format!(r"<p>((?:(?:{})\s*)+)</p>", tag_pattern)).unwrap();

    re.replace_all(html, |caps: &regex::Captures| {
        let hashtags_text = &caps[1];
        let hashtag_re = Regex::new(r"#([a-zA-Z0-9_-]+)").unwrap();
        let links = hashtag_re.replace_all(hashtags_text, |c: &regex::Captures| {
            let tag = &c[1];
            format!(r#"<a href="/index.html#{}">{}</a>"#, tag, format!("#{}", tag))
        });
        format!(r#"<p class="post-hashtags">{}</p>"#, links)
    }).to_string()
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

fn get_related_posts(all_posts: &[Post], current_id: &str, tags: &[String], limit: usize) -> Vec<PostSummary> {
    if tags.is_empty() {
        return Vec::new();
    }

    let first_tag = &tags[0];
    let mut related: Vec<_> = all_posts.iter()
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

fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#039;")
}

fn format_date(date_str: &str) -> String {
    if let Ok(date) = NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
        date.format("%B %-d, %Y").to_string()
    } else {
        date_str.to_string()
    }
}

fn generate_index_page(out_dir: &Path, posts: &[PostSummary], tags: &[String]) -> Result<()> {
    let mut file = BufWriter::new(File::create(out_dir.join("index.html"))?);

    let css = read_inline_css()?;

    write!(file, r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>Sean's Blog</title>
  <meta name="description" content="Another place for thought infusion">
  <meta property="og:image" content="https://seanpedersen.github.io/images/sierpinski-twitter-square.png">
  <meta name="og:title" content="Sean's Blog">
  <meta name="twitter:card" content="summary">
  <meta name="twitter:image" content="https://seanpedersen.github.io/images/sierpinski-twitter-square.png">
  <link rel="icon" href="/favicon.ico">
  <link rel="alternate" type="application/rss+xml" href="/rss.xml" title="RSS Feed">
  <style>{}</style>
  <script>{}</script>
</head>
<body>
  <script>{}</script>
  <div class="flexer">
    <div class="container">
      <header class="header">
        <div class="headerContainer">
          <div class="speechBubble">
            Building <a href="https://solo.digger.lol/" target="_blank" rel="noopener noreferrer">Digger Solo</a>
          </div>
          <div class="profileContainer">
            <img
              src="/images/profile.webp"
              class="headerHomeImage borderCircle"
              alt="Sean Pedersen"
              decoding="async"
              fetchpriority="high"
            />
            <div style="display: flex; flex-direction: column; align-items: flex-start;">
              <h1 class="heading2Xl nameBreak">
                <span>Sean</span>
                <span>Pedersen</span>
              </h1>
              <div class="linkRow">
                <a href="/rss.xml" class="iconLink" aria-label="RSS" title="RSS Feed">
                  <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none"></path><path d="M5 19m-1 0a1 1 0 1 0 2 0a1 1 0 1 0 -2 0"></path><path d="M4 4a16 16 0 0 1 16 16"></path><path d="M4 11a9 9 0 0 1 9 9"></path></svg>
                </a>
                <div class="socialLinks">
                  <a href="https://github.com/SeanPedersen" class="iconLink" aria-label="GitHub" title="GitHub" target="_blank" rel="noreferrer noopener">
                    <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none"></path><path d="M9 19c-4.3 1.4 -4.3 -2.5 -6 -3m12 5v-3.5c0 -1 .1 -1.4 -.5 -2c2.8 -.3 5.5 -1.4 5.5 -6a4.6 4.6 0 0 0 -1.3 -3.2a4.2 4.2 0 0 0 -.1 -3.2s-1.1 -.3 -3.5 1.3a12.3 12.3 0 0 0 -6.2 0c-2.4 -1.6 -3.5 -1.3 -3.5 -1.3a4.2 4.2 0 0 0 -.1 3.2a4.6 4.6 0 0 0 -1.3 3.2c0 4.6 2.7 5.7 5.5 6c-.6 .6 -.6 1.2 -.5 2v3.5"></path></svg>
                  </a>
                  <a href="https://x.com/SeanPedersen96" class="iconLink" aria-label="X (Twitter)" title="X (Twitter)" target="_blank" rel="noreferrer noopener">
                    <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none"></path><path d="M4 4l11.733 16h4.267l-11.733 -16z"></path><path d="M4 20l6.768 -6.768m2.46 -2.46l6.772 -6.772"></path></svg>
                  </a>
                </div>
              </div>
            </div>
          </div>
        </div>
      </header>
      <main class="main homePage">
        <section class="headingMd">
          <p>Machine Learning / Data Privacy / Cybernetics / Memetics</p>
        </section>
        <section class="headingMd padding1px">
          <div class="searchTagsWrapper" id="searchTagsWrapper">
            <div id="searchContainer"></div>
            <div class="tagsContainer" id="tagsContainer">
              <span class="tag tagSelected" data-tag="">All</span>
"#, css, get_theme_init_script(), get_theme_body_script())?;

    // Write tags
    for tag in tags {
        write!(file, r#"              <span class="tag" data-tag="{}">{}</span>
"#, escape_html(tag), escape_html(tag))?;
    }

    write!(file, r#"            </div>
          </div>
          <ul class="list" id="postList">
"#)?;

    // Write posts
    for post in posts {
        let tags_json = serde_json::to_string(&post.tags).unwrap();
        write!(file, r#"            <li class="listItem" data-id="{}" data-tags="{}">
              <a href="/posts/{}.html">{}</a>
"#, escape_html(&post.id), escape_html(&tags_json), escape_html(&post.id), escape_html(&post.title))?;

        if !post.tags.is_empty() {
            write!(file, r#"              <small class="lightText post-tags-container">
                •<span class="postTags">"#)?;
            for (i, tag) in post.tags.iter().enumerate() {
                write!(file, r##"<span><a href="#{}" onclick="return false;">{}</a>{}</span>"##,
                    escape_html(tag), escape_html(tag),
                    if i < post.tags.len() - 1 { ", " } else { "" })?;
            }
            write!(file, r#"</span>
              </small>
"#)?;
        }
        write!(file, r#"            </li>
"#)?;
    }

    // Footer and scripts
    let year = chrono::Local::now().year();
    write!(file, r#"          </ul>
        </section>
      </main>
    </div>
    <footer class="footer">
      <p>Copy©at ᓚᘏᗢ {} | All lights served .:.</p>
    </footer>
    <button id="themeToggle" class="themeToggleButton" aria-label="Toggle theme" title="Toggle theme">
      <svg id="themeIcon" width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" strokeLinecap="round" strokeLinejoin="round">
        <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z" />
      </svg>
    </button>
  </div>
  <script src="/js/theme.js" defer></script>
  <script>{}</script>
  <script src="/js/tags.js" defer></script>
  <script src="/js/prefetch.js" defer></script>
  <script>{}</script>
</body>
</html>"#, year, get_search_script(), get_posthog_script())?;

    Ok(())
}

fn generate_post_page(out_dir: &Path, post: &Post, related: &[PostSummary]) -> Result<()> {
    let mut file = BufWriter::new(File::create(out_dir.join(format!("{}.html", post.id)))?);

    let css = read_inline_css()?;
    let prism_css = fs::read_to_string("node_modules/prismjs/themes/prism-tomorrow.css")?;

    let excerpt = strip_html_tags(&post.content_html)
        .chars()
        .take(160)
        .collect::<String>();

    let title_id = post.title.to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-");

    let has_toc = post.content_html.contains("<h1") ||
                  post.content_html.contains("<h2") ||
                  post.content_html.contains("<h3");

    write!(file, r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>{} - Sean's Blog</title>
  <link rel="canonical" href="https://seanpedersen.github.io/posts/{}">
  <meta name="description" content="{}">
  <meta name="keywords" content="{}">
  <meta property="og:type" content="article">
  <meta property="og:title" content="{}">
  <meta property="og:description" content="{}">
  <meta property="og:url" content="https://seanpedersen.github.io/posts/{}">
  <meta property="og:image" content="https://seanpedersen.github.io/images/sierpinski-twitter-square.png">
  <meta name="twitter:card" content="summary">
  <meta name="twitter:title" content="{}">
  <meta name="twitter:description" content="{}">
  <meta name="twitter:image" content="https://seanpedersen.github.io/images/sierpinski-twitter-square.png">
  <link rel="icon" href="/favicon.ico">
  <link rel="alternate" type="application/rss+xml" href="/rss.xml" title="RSS Feed">
  <style>{}{}</style>
  <script>{}</script>
</head>
<body>
  <script>{}</script>
  <div class="container">
    <header class="header">
      <div style="display: flex; align-items: center; gap: 1rem; padding: 0.5rem;">
        <h2 class="headingLg">
          <a href="/" class="colorInherit">Sean's Blog</a>
        </h2>
      </div>
    </header>
    <main class="main postPage{}">
      <article class="postContainer">
"#, escape_html(&post.title), escape_html(&post.id), escape_html(&excerpt),
    post.tags.iter().map(|t| escape_html(t)).collect::<Vec<_>>().join(", "),
    escape_html(&post.title), escape_html(&excerpt), escape_html(&post.id),
    escape_html(&post.title), escape_html(&excerpt),
    css, prism_css, get_theme_init_script(), get_theme_body_script(),
    if has_toc { "" } else { " noToc" })?;

    if has_toc {
        write!(file, r#"        <div id="tocContainer"></div>
"#)?;
    }

    write!(file, r#"        <span id="{}" style="position: absolute; top: 0; visibility: hidden;" aria-hidden="true"></span>
        <h1 class="headingXl">{}</h1>
        <div class="postMeta">
          <div class="lightText">{}</div>
          <a href="https://github.com/SeanPedersen/seanpedersen.github.io/edit/main/posts/{}.md" target="_blank" rel="noopener noreferrer" class="editOnGithubLink">
            Edit on GitHub
          </a>
        </div>
        <div class="markdown-content" style="padding-bottom: 0.25rem; margin-bottom: 0;">
          {}
        </div>
"#, title_id, escape_html(&post.title), format_date(&post.date), escape_html(&post.id), post.content_html)?;

    // Related posts
    if !related.is_empty() {
        write!(file, r#"        <footer class="relatedPostsFooter">
          <h3>Related Articles</h3>
          <ul class="relatedPostsList" style="padding-left: 0">
"#)?;
        for rel in related {
            write!(file, r#"            <li class="relatedPostItem">
              <a href="/posts/{}.html">{}</a>
              <small class="lightText">{}</small>
            </li>
"#, escape_html(&rel.id), escape_html(&rel.title), format_date(&rel.date))?;
        }
        write!(file, r#"          </ul>
        </footer>
"#)?;
    }

    write!(file, r#"        <footer class="backToTopFooter">
          <hr aria-hidden="true" style="width: 100%; border: 0; border-top: 1px solid rgba(127,127,127,0.35); margin: 0.25rem 0;" />
          <div id="backToTopContainer"></div>
          <p>omnia mirari, gaudium explorandi .:.</p>
        </footer>
      </article>
    </main>
  </div>
  <script>
    window.__POST_DATA__ = {{
      title: {},
      titleId: {},
      hasTableOfContents: {}
    }};
  </script>
  <script src="/js/theme.js" defer></script>
  <script src="/js/post.js" defer></script>
  <script src="/js/prefetch.js" defer></script>
  <script>{}</script>
</body>
</html>"#, serde_json::to_string(&post.title)?, serde_json::to_string(&title_id)?,
    has_toc, get_posthog_script())?;

    Ok(())
}

fn read_inline_css() -> Result<String> {
    let global = fs::read_to_string("styles/global.css")?;
    let utils = fs::read_to_string("styles/utils.module.css")?;
    let layout = fs::read_to_string("styles/layout.module.css")?;

    // Remove :global() wrappers more carefully
    let layout = Regex::new(r":global\(([^)]+)\)").unwrap()
        .replace_all(&layout, "$1")
        .to_string();

    Ok(format!("{}{}{}", global, layout, utils))
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

    fs::copy("node_modules/prismjs/themes/prism-tomorrow.css", styles_out.join("prism-tomorrow.css"))?;
    println!("  ✓ Copied prism-tomorrow.css");

    fs::copy("styles/utils.module.css", styles_out.join("utils.css"))?;
    println!("  ✓ Copied utils.css");

    let layout_css = fs::read_to_string("styles/layout.module.css")?;
    let layout_css = Regex::new(r":global\(([^)]+)\)").unwrap()
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

fn get_theme_init_script() -> &'static str {
    r#"(function() {
  try {
    var storedTheme = document.cookie
      .split('; ')
      .find(function(row) { return row.startsWith('theme='); });

    var theme = storedTheme ? storedTheme.split('=')[1] :
      (window.matchMedia && window.matchMedia('(prefers-color-scheme: light)').matches ? 'light' : 'dark');

    var style = document.createElement('style');
    style.innerHTML = theme === 'light'
      ? 'body{background-color:#f0f0f0!important;color:#111!important}'
      : 'body{background-color:#000!important;color:#fff!important}';
    document.head.appendChild(style);

    if (theme === 'light') {
      document.documentElement.classList.add('light-theme-pending');
    }
  } catch (e) {}
})();"#
}

fn get_theme_body_script() -> &'static str {
    r#"(function() {
  try {
    if (document.documentElement.classList.contains('light-theme-pending')) {
      document.documentElement.classList.remove('light-theme-pending');
      document.body.classList.add('light-theme');
    }
  } catch (e) {}
})();"#
}

fn get_search_script() -> &'static str {
    r#"(function() {
      let searchScriptLoaded = false;
      let searchInitialized = false;

      function initInlineSearch() {
        const container = document.getElementById('searchContainerEl');
        const wrapper = document.getElementById('searchTagsWrapper');
        if (!container || !wrapper || searchInitialized) return;

        searchInitialized = true;

        container.classList.remove('collapsed');
        container.classList.add('expanded');
        container.innerHTML = `
          <div class="searchInputWrapper" style="--wrapper-width: ${wrapper.offsetWidth}px">
            <svg class="searchIcon" width="1em" height="1em" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
              <circle cx="11" cy="11" r="6" />
              <line x1="19.59" y1="19.59" x2="15.24" y2="15.24" />
            </svg>
            <input id="searchInput" type="text" class="searchInput" placeholder="Search posts..." />
            <button class="clearButton" id="clearSearchBtn" aria-label="Clear search" style="display: none;">×</button>
          </div>
        `;

        wrapper.classList.add('searchExpanded');
        const tagsContainer = document.getElementById('tagsContainer');
        if (tagsContainer) tagsContainer.classList.add('tagsHidden');

        const input = document.getElementById('searchInput');
        const clearBtn = document.getElementById('clearSearchBtn');

        if (input) {
          input.focus();
          requestAnimationFrame(() => input.focus());

          input.addEventListener('input', function(e) {
            const query = e.target.value;
            const lowerQuery = query.toLowerCase();
            if (clearBtn) clearBtn.style.display = query ? 'flex' : 'none';

            const postList = document.getElementById('postList');
            if (!postList) return;

            Array.from(postList.children).forEach(post => {
              const link = post.querySelector('a');
              if (!link) return;

              const title = link.textContent || '';
              const tagsAttr = post.getAttribute('data-tags');
              const tags = tagsAttr ? JSON.parse(tagsAttr) : [];

              const titleMatches = title.toLowerCase().includes(lowerQuery);
              const tagMatches = tags.some(tag => tag.toLowerCase().includes(lowerQuery));

              if (!query || titleMatches || tagMatches) {
                post.style.display = '';

                if (query && titleMatches) {
                  const index = title.toLowerCase().indexOf(lowerQuery);
                  const before = title.slice(0, index);
                  const match = title.slice(index, index + query.length);
                  const after = title.slice(index + query.length);

                  const escapeHtml = (text) => {
                    const div = document.createElement('div');
                    div.textContent = text;
                    return div.innerHTML;
                  };

                  link.innerHTML = `${escapeHtml(before)}<mark style="background-color: var(--primary-color); color: var(--color-background); padding: 2px 4px; border-radius: 3px;">${escapeHtml(match)}</mark>${escapeHtml(after)}`;
                } else {
                  link.textContent = title;
                }
              } else {
                post.style.display = 'none';
              }
            });
          });

          if (clearBtn) {
            clearBtn.addEventListener('mousedown', function(e) {
              e.preventDefault();
              input.value = '';
              clearBtn.style.display = 'none';
              const postList = document.getElementById('postList');
              if (postList) {
                Array.from(postList.children).forEach(post => post.style.display = '');
              }
              setTimeout(() => input.focus(), 0);
            });
          }

          if (!searchScriptLoaded) {
            searchScriptLoaded = true;
            window.__searchInputValue = input.value || '';
            const script = document.createElement('script');
            script.src = '/js/search.js';
            document.head.appendChild(script);
          }
        }
      }

      function setupSearchButton() {
        const container = document.getElementById('searchContainer');
        if (!container) return;

        container.innerHTML = `
          <div class="searchContainer collapsed" id="searchContainerEl">
            <button class="searchIconButton" id="searchIconBtn" aria-label="Open search">
              <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <circle cx="11" cy="11" r="6" />
                <line x1="19.59" y1="19.59" x2="15.24" y2="15.24" />
              </svg>
            </button>
          </div>
        `;

        document.getElementById('searchIconBtn').addEventListener('click', function(e) {
          e.preventDefault();
          initInlineSearch();
        });
      }

      if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', setupSearchButton);
      } else {
        setupSearchButton();
      }
    })();"#
}

fn get_posthog_script() -> &'static str {
    r#"(function() {
  if (typeof window === 'undefined') return;

  var script = document.createElement('script');
  script.src = '/js/posthog-lite.js';
  script.onload = function() {
    if (window.PostHogLite) {
      window.posthog = new window.PostHogLite('phc_9XPlyPALuefIMAMSfsvBk4jVuSelJyjl7xwhXigkHAP', {
        host: 'https://us.i.posthog.com',
        person_profiles: 'identified_only',
        capturePageview: true
      });
    }
  };
  document.head.appendChild(script);
})();"#
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

    collect_files(out_dir, &mut css_files, &mut js_files, &mut html_files, &mut image_files)?;

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

    let stylesheet = StyleSheet::parse(
        &css,
        ParserOptions::default(),
    ).map_err(|e| anyhow::anyhow!("CSS parse error: {:?}", e))?;

    let minified = stylesheet.to_css(PrinterOptions {
        minify: true,
        ..Default::default()
    }).map_err(|e| anyhow::anyhow!("CSS minify error: {:?}", e))?;

    fs::write(path, minified.code)?;
    Ok(())
}

fn minify_js_file(path: &Path) -> Result<()> {
    let js = fs::read_to_string(path)?;

    // Use minify-js for proper JS minification
    let session = Session::new();
    let mut output = Vec::new();

    minify_js_code(
        &session,
        TopLevelMode::Global,
        js.as_bytes(),
        &mut output,
    ).map_err(|e| anyhow::anyhow!("JS minify error: {:?}", e))?;

    let minified = String::from_utf8(output)
        .unwrap_or_else(|_| js.clone());

    // Only write if smaller or same size
    if minified.len() <= js.len() {
        fs::write(path, minified)?;
    }

    Ok(())
}

fn minify_html_file(path: &Path) -> Result<()> {
    let html = fs::read(path)?;

    let cfg = Cfg {
        minify_css: false,  // Don't minify CSS inside HTML - lightningcss already did it
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
    write!(file, r#"<?xml version="1.0" encoding="UTF-8"?>
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
"#, escape_xml("Sean's Blog"), rss_date, rss_date)?;

    // Write each post
    for post in posts.iter() {
        let post_url = format!("https://seanpedersen.github.io/posts/{}", post.id);
        let post_date = NaiveDate::parse_from_str(&post.date, "%Y-%m-%d")
            .ok()
            .and_then(|d| d.and_hms_opt(0, 0, 0))
            .and_then(|dt| {
                let utc = chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(dt, chrono::Utc);
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
        let html_content = fs::read_to_string(&html_path)
            .unwrap_or_default();

        // Extract content from markdown-content div
        let content = extract_content_from_html(&html_content);

        // Write content
        write!(file, "      <content:encoded><![CDATA[{}]]></content:encoded>\n    </item>\n", content)?;
    }

    write!(file, "  </channel>\n</rss>")?;

    Ok(())
}

fn extract_content_from_html(html: &str) -> String {
    // Find the start of markdown-content div
    if let Some(start_idx) = html.find("<div class=markdown-content") {
        // Find the end of the opening tag
        if let Some(content_start) = html[start_idx..].find('>') {
            let content_start = start_idx + content_start + 1;

            // Find the matching closing </div> before <footer
            // We need to count div depth
            let mut depth = 1;
            let mut pos = content_start;
            let bytes = html.as_bytes();

            while pos < bytes.len() && depth > 0 {
                if pos + 5 < bytes.len() && &bytes[pos..pos+5] == b"<div " || &bytes[pos..pos+4] == b"<div>" {
                    depth += 1;
                    pos += 1;
                } else if pos + 6 < bytes.len() && &bytes[pos..pos+6] == b"</div>" {
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
    }

    String::new()
}

fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

