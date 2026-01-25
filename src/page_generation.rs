use anyhow::Result;
use chrono::NaiveDate;
use once_cell::sync::Lazy;
use pulldown_cmark::{html, CodeBlockKind, Event, Options, Parser, Tag, TagEnd};
use pulldown_cmark_escape::escape_html;
use regex::Regex;
use serde::{Deserialize, Serialize};
use rayon::prelude::*;
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use std::time::Instant;
use syntect::html::{ClassStyle, ClassedHTMLGenerator};
use syntect::parsing::{SyntaxSet, SyntaxSetBuilder};

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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PostMetadata {
    pub date: Option<String>,
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

pub fn get_posts_data(_out_dir: &Path) -> Result<Arc<Vec<Post>>> {
    let start = Instant::now();
    let posts_dir = Path::new("posts");
    let repo = gix::open(".").ok();
    let mut posts = read_all_posts(posts_dir, repo.as_ref())?;
    posts.sort_by(|a, b| b.date.cmp(&a.date));
    println!(
        "✓ Loaded {} posts in {:.2}s",
        posts.len(),
        start.elapsed().as_secs_f64()
    );
    Ok(Arc::new(posts))
}

pub fn read_all_posts(posts_dir: &Path, repo: Option<&gix::Repository>) -> Result<Vec<Post>> {
    let entries: Vec<_> = fs::read_dir(posts_dir)?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("md"))
        .map(|e| e.path())
        .collect();

    // First pass: resolve dates for files missing frontmatter dates (sequential for git access)
    let mut git_dates: std::collections::HashMap<std::path::PathBuf, String> =
        std::collections::HashMap::new();
    for path in &entries {
        // Quick check: read just enough to extract frontmatter date
        if let Ok(content) = fs::read_to_string(path) {
            let (metadata, _) = parse_frontmatter(&content);
            if metadata.date.is_none() {
                if let Some(date) = repo.and_then(|r| get_git_first_add_date(r, path)) {
                    git_dates.insert(path.clone(), date);
                }
            }
        }
    }

    // Second pass: parse everything in parallel
    let posts: Vec<Post> = entries
        .par_iter()
        .filter_map(|path| {
            let content = fs::read_to_string(path).ok()?;
            let id = path.file_stem()?.to_str()?.to_string();
            let (metadata, markdown) = parse_frontmatter(&content);

            let date = metadata
                .date
                .or_else(|| git_dates.get(path).cloned())?;

            let title = extract_title(&markdown);
            if title.is_empty() {
                return None;
            }
            let tags = extract_tags(&content);
            let markdown_without_title = remove_first_h1(&markdown);
            let content_html = markdown_to_html(&markdown_without_title, &tags);

            Some(Post {
                id,
                title,
                date,
                tags,
                content_html,
            })
        })
        .collect();

    Ok(posts)
}

fn parse_frontmatter(content: &str) -> (PostMetadata, String) {
    let re = Regex::new(r"(?s)^---\s*\n(.*?)\n---\s*\n(.*)$").unwrap();

    if let Some(caps) = re.captures(content) {
        let yaml = caps.get(1).unwrap().as_str();
        let markdown = caps.get(2).unwrap().as_str();
        let metadata: PostMetadata = serde_yaml::from_str(yaml).unwrap_or_default();
        (metadata, markdown.to_string())
    } else {
        (PostMetadata::default(), content.to_string())
    }
}

fn get_git_first_add_date(repo: &gix::Repository, file_path: &Path) -> Option<String> {
    let head = repo.head_commit().ok()?;

    // Walk newest to oldest, track last commit where file exists
    let mut first_add_date = None;
    for info in head.ancestors().all().ok()?.flatten() {
        let commit = info.id().object().ok()?.try_into_commit().ok()?;
        let tree = commit.tree().ok()?;

        if tree.lookup_entry_by_path(file_path).ok().flatten().is_some() {
            let time = commit.time().ok()?;
            first_add_date = chrono::DateTime::from_timestamp(time.seconds, 0)
                .map(|dt| dt.format("%Y-%m-%d").to_string());
        } else if first_add_date.is_some() {
            break;
        }
    }

    first_add_date
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

fn find_syntax<'a>(
    syntax_set: &'a SyntaxSet,
    lang: &str,
) -> Option<&'a syntect::parsing::SyntaxReference> {
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
    let mut in_blockquote = false;
    let mut in_image = false;
    let mut image_alt_text = String::new();
    let mut image_url = String::new();

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
                            let (syntax_ref, syntax_set_ref) = if let Some(custom_set) =
                                &*CUSTOM_SYNTAXES
                            {
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
                } else if in_image {
                    image_alt_text.push_str(&text);
                } else {
                    let mut escaped = String::new();
                    escape_html(&mut escaped, &text).unwrap();
                    html_output.push_str(&escaped);
                }
            }
            Event::Start(Tag::Image { dest_url, .. }) => {
                in_image = true;
                image_alt_text.clear();
                image_url = dest_url.to_string();
            }
            Event::End(TagEnd::Image) => {
                let mut escaped_alt = String::new();
                escape_html(&mut escaped_alt, &image_alt_text).unwrap();
                html_output.push_str(&format!(
                    r#"<img src="{}" alt="{}">"#,
                    image_url, escaped_alt
                ));
                in_image = false;
                image_alt_text.clear();
                image_url.clear();
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
            Event::Start(Tag::BlockQuote(_)) => {
                in_blockquote = true;
                html_output.push_str("<blockquote>");
            }
            Event::End(TagEnd::BlockQuote(_)) => {
                in_blockquote = false;
                html_output.push_str("</blockquote>");
            }
            Event::Start(Tag::Paragraph) => {
                if in_blockquote {
                    html_output.push_str(r#"<p class="quote-line">"#);
                } else {
                    html_output.push_str("<p>");
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

#[derive(Debug, Clone, Serialize)]
pub struct Heading {
    pub level: u8,
    pub text: String,
    pub id: String,
}

pub fn extract_headings(html: &str) -> Vec<Heading> {
    let re = Regex::new(r"<h([1-6])[^>]*>(.*?)</h[1-6]>").unwrap();
    re.captures_iter(html)
        .map(|caps| {
            let level = caps[1].parse::<u8>().unwrap_or(1);
            let raw_text = &caps[2];
            let plain_text = strip_html_tags(raw_text);
            let text = decode_html_entities(&plain_text);
            let id = text
                .to_lowercase()
                .chars()
                .map(|c| if c.is_alphanumeric() { c } else { '-' })
                .collect::<String>()
                .split('-')
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>()
                .join("-");
            Heading { level, text, id }
        })
        .collect()
}

fn decode_html_entities(text: &str) -> String {
    let re = Regex::new(r"&(#(?:x[0-9a-fA-F]+|\d+)|[a-zA-Z]+);").unwrap();
    re.replace_all(text, |caps: &regex::Captures| {
        let code = &caps[1];
        if code.starts_with('#') {
            let is_hex = code.starts_with("#x") || code.starts_with("#X");
            let num_str = if is_hex { &code[2..] } else { &code[1..] };
            if let Ok(num) = u32::from_str_radix(num_str, if is_hex { 16 } else { 10 }) {
                if let Some(ch) = char::from_u32(num) {
                    return ch.to_string();
                }
            }
            caps[0].to_string()
        } else {
            match code {
                "amp" => "&".to_string(),
                "lt" => "<".to_string(),
                "gt" => ">".to_string(),
                "quot" => "\"".to_string(),
                "apos" => "'".to_string(),
                "nbsp" => "\u{00A0}".to_string(),
                _ => caps[0].to_string(),
            }
        }
    })
    .to_string()
}

pub fn strip_html_tags(html: &str) -> String {
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

pub fn format_date(date_str: &str) -> String {
    if let Ok(date) = NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
        date.format("%B %-d, %Y").to_string()
    } else {
        date_str.to_string()
    }
}
