use crate::math;
use anyhow::Result;
use chrono::NaiveDate;
use gix::object::tree::diff::{Action, Change};
use latex2mathml::DisplayStyle;
use once_cell::sync::Lazy;
use pulldown_cmark::{html, CodeBlockKind, Event, Options, Parser, Tag, TagEnd};
use pulldown_cmark_escape::escape_html;
use rayon::prelude::*;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Instant;
use syntect::html::{ClassStyle, ClassedHTMLGenerator};
use syntect::parsing::{SyntaxSet, SyntaxSetBuilder};

// Load custom syntaxes once at startup
static CUSTOM_SYNTAXES: Lazy<Option<SyntaxSet>> = Lazy::new(|| {
    if Path::new("src/syntaxes").exists() {
        let mut builder = SyntaxSetBuilder::new();
        match builder.add_from_folder("src/syntaxes", true) {
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

// Load default syntaxes once at startup (avoids reparsing the whole set per post)
static DEFAULT_SYNTAXES: Lazy<SyntaxSet> = Lazy::new(SyntaxSet::load_defaults_newlines);

static FRONTMATTER_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?s)^---\s*\n(.*?)\n---\s*\n(.*)$").unwrap());
static HTML_TITLE_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"<h1[^>]*>(.*?)</h1>").unwrap());
static HASHTAG_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"#([a-zA-Z0-9_-]+)").unwrap());
static SYNTAX_CLASS_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"<span class="([^"]+)">"#).unwrap());
static HEADING_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"<h([1-6])>(.*?)</h[1-6]>").unwrap());
static HEADING_WITH_ATTRS_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"<h([1-6])([^>]*)>(.*?)</h[1-6]>").unwrap());
static ID_ATTR_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"\bid\s*=\s*["']([^"']+)["']"#).unwrap());
static ENTITY_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"&(#(?:x[0-9a-fA-F]+|\d+)|[a-zA-Z]+);").unwrap());
static HTML_TAG_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"<[^>]*>").unwrap());
static URL_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(https?://[^\s<>]+?)([.,;:!?)]*(?:\s|$))").unwrap());

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PostMetadata {
    pub date: Option<String>,
    pub icon: Option<String>,
    pub title: Option<String>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct Post {
    pub id: String,
    pub title: String,
    pub date: String,
    pub date_modified: String,
    pub tags: Vec<String>,
    pub icon: Option<String>,
    pub content_html: String,
    #[cfg_attr(not(feature = "smart-similar"), allow(dead_code))]
    pub content_raw: String,
}

#[derive(Debug, Clone)]
pub struct PostSummary {
    pub id: String,
    pub title: String,
    pub date: String,
    pub tags: Vec<String>,
    pub icon: Option<String>,
}

pub fn get_posts_data(_out_dir: &Path) -> Result<Arc<Vec<Post>>> {
    let start = Instant::now();
    let posts_dir = Path::new("posts");
    let mut posts = read_all_posts(posts_dir)?;
    posts.sort_by(|a, b| b.date.cmp(&a.date));
    println!(
        "✓ Loaded {} posts in {:.2}s",
        posts.len(),
        start.elapsed().as_secs_f64()
    );
    Ok(Arc::new(posts))
}

pub fn read_all_posts(posts_dir: &Path) -> Result<Vec<Post>> {
    let entries: Vec<_> = fs::read_dir(posts_dir)?
        .filter_map(|e| e.ok())
        .filter(|e| {
            matches!(
                e.path().extension().and_then(|s| s.to_str()),
                Some("md") | Some("html")
            )
        })
        .map(|e| e.path())
        .collect();

    let git_dates = get_all_git_dates(&entries);

    let posts: Vec<Post> = entries
        .par_iter()
        .filter_map(|path| {
            let content = fs::read_to_string(path).ok()?;
            let id = path.file_stem()?.to_str()?.to_string();
            let is_html = path.extension().and_then(|s| s.to_str()) == Some("html");
            let (metadata, body) = parse_frontmatter(&content);

            let (git_first, git_last) = git_dates.get(path).cloned().unwrap_or_default();
            let date = metadata.date.or(git_first)?;
            let date_modified = git_last.unwrap_or_else(|| date.clone());

            let (title, tags, content_html, content_raw) = if is_html {
                let title = metadata
                    .title
                    .unwrap_or_else(|| extract_html_title(&body).unwrap_or_default());
                let tags = metadata.tags.unwrap_or_default();
                (title, tags, body.clone(), body)
            } else {
                let title = metadata.title.unwrap_or_else(|| extract_title(&body));
                let tags = metadata.tags.unwrap_or_else(|| extract_tags(&content));
                let content_raw = body.clone();
                let markdown_without_title = remove_first_h1(&body);
                let content_html = markdown_to_html(&markdown_without_title, &tags);
                (title, tags, content_html, content_raw)
            };

            if title.is_empty() {
                eprintln!("Warning: Skipping post '{}' — no title found", id);
                return None;
            }

            Some(Post {
                id,
                title,
                date,
                date_modified,
                tags,
                icon: metadata.icon.and_then(|icon| {
                    if icon.trim().is_empty() {
                        None
                    } else {
                        Some(icon)
                    }
                }),
                content_html,
                content_raw,
            })
        })
        .collect();

    Ok(posts)
}

fn parse_frontmatter(content: &str) -> (PostMetadata, String) {
    if let Some(caps) = FRONTMATTER_RE.captures(content) {
        let yaml = caps.get(1).unwrap().as_str();
        let markdown = caps.get(2).unwrap().as_str();
        let metadata: PostMetadata = serde_yaml::from_str(yaml).unwrap_or_default();
        (metadata, markdown.to_string())
    } else {
        (PostMetadata::default(), content.to_string())
    }
}

fn ts_to_date(ts: i64, offset: i32) -> Option<String> {
    let local_ts = ts + offset as i64;
    chrono::DateTime::from_timestamp(local_ts, 0).map(|dt| dt.format("%Y-%m-%d").to_string())
}

// Cache for decoded tree/commit objects while walking history (adjacent commits share most trees).
const GIT_OBJECT_CACHE_SIZE: usize = 64 * 1024 * 1024;

fn get_all_git_dates(paths: &[PathBuf]) -> HashMap<PathBuf, (Option<String>, Option<String>)> {
    let mut repo = match gix::open(".") {
        Ok(r) => r,
        Err(_) => return HashMap::new(),
    };
    repo.object_cache_size(GIT_OBJECT_CACHE_SIZE);
    let head = match repo.head_commit() {
        Ok(h) => h,
        Err(_) => return HashMap::new(),
    };

    let wanted: HashSet<PathBuf> = paths.iter().cloned().collect();
    // (oldest_ts, oldest_offset, newest_ts, newest_offset); walking newest -> oldest commits,
    // the first change seen for a path is its newest, later changes update the oldest.
    let mut dates: HashMap<PathBuf, (i64, i32, i64, i32)> = HashMap::new();
    let mut resource_cache = match repo.diff_resource_cache_for_tree_diff() {
        Ok(c) => c,
        Err(_) => return HashMap::new(),
    };

    for info in head.ancestors().all().ok().into_iter().flatten().flatten() {
        let Some(commit) = info
            .id()
            .object()
            .ok()
            .and_then(|o| o.try_into_commit().ok())
        else {
            continue;
        };
        let Ok(tree) = commit.tree() else {
            continue;
        };
        let parent_tree = match commit.parent_ids().next() {
            Some(parent_id) => match parent_id
                .object()
                .ok()
                .and_then(|o| o.try_into_commit().ok())
                .and_then(|c| c.tree().ok())
            {
                Some(t) => t,
                None => continue,
            },
            None => repo.empty_tree(),
        };

        let mut platform = match parent_tree.changes() {
            Ok(p) => p,
            Err(_) => continue,
        };
        platform.options(|opts| {
            opts.track_rewrites(None);
            opts.track_path();
        });

        let mut changed: Vec<PathBuf> = Vec::new();
        if platform
            .for_each_to_obtain_tree_with_cache(&tree, &mut resource_cache, |change| {
                let location = match change {
                    Change::Addition { location, .. }
                    | Change::Deletion { location, .. }
                    | Change::Modification { location, .. } => location,
                    Change::Rewrite { .. } => {
                        return Ok::<_, std::convert::Infallible>(Action::Continue)
                    }
                };
                let path = PathBuf::from(std::str::from_utf8(location).unwrap_or(""));
                if wanted.contains(&path) {
                    changed.push(path);
                }
                Ok::<_, std::convert::Infallible>(Action::Continue)
            })
            .is_err()
        {
            continue;
        }

        if changed.is_empty() {
            continue;
        }
        let Ok(time) = commit.time() else {
            continue;
        };
        for path in changed {
            let entry =
                dates
                    .entry(path)
                    .or_insert((time.seconds, time.offset, time.seconds, time.offset));
            if time.seconds < entry.0 {
                entry.0 = time.seconds;
                entry.1 = time.offset;
            }
        }
    }

    paths
        .iter()
        .map(|path| {
            let (first, last) = dates
                .get(path)
                .map_or((None, None), |&(o_ts, o_off, n_ts, n_off)| {
                    (ts_to_date(o_ts, o_off), ts_to_date(n_ts, n_off))
                });
            (path.clone(), (first, last))
        })
        .collect()
}

fn extract_html_title(html: &str) -> Option<String> {
    HTML_TITLE_RE
        .captures(html)
        .map(|caps| strip_html_tags(&caps[1]).trim().to_string())
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
        return HASHTAG_RE
            .captures_iter(last_line)
            .map(|cap| cap[1].to_string())
            .collect();
    }
    Vec::new()
}

fn remove_first_h1(markdown: &str) -> String {
    if let Some(first_line) = markdown.lines().next() {
        if first_line.starts_with("# ") {
            let mut rest = &markdown[first_line.len()..];
            rest = rest.strip_prefix('\n').unwrap_or(rest);
            return rest.to_string();
        }
    }

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
    options.insert(Options::ENABLE_MATH);

    let parser = Parser::new_ext(markdown, options);

    let syntax_set = &*DEFAULT_SYNTAXES;

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
    let mut in_link = false;

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
                        if let Some(syntax) = find_syntax(syntax_set, lang) {
                            // Determine which syntax set to use for the generator
                            let (syntax_ref, syntax_set_ref) = if let Some(custom_set) =
                                &*CUSTOM_SYNTAXES
                            {
                                if let Some(custom_syntax) = custom_set.find_syntax_by_token(lang) {
                                    (custom_syntax, custom_set)
                                } else {
                                    (syntax, syntax_set)
                                }
                            } else {
                                (syntax, syntax_set)
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
                            // Language not found, use plain code block with consistent styling
                            let mut escaped = String::new();
                            escape_html(&mut escaped, &code_block_content).unwrap();
                            html_output.push_str(&format!(
                                r#"<div class="remark-highlight"><pre class="language-{}"><code class="language-{}">{}</code></pre></div>"#,
                                lang, lang, escaped
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
            Event::Start(Tag::Link {
                link_type,
                dest_url,
                title,
                id,
            }) => {
                in_link = true;
                let transformed_url = if dest_url.ends_with(".md") {
                    dest_url.strip_suffix(".md").unwrap().to_string().into()
                } else {
                    dest_url
                };
                let new_event = Event::Start(Tag::Link {
                    link_type,
                    dest_url: transformed_url,
                    title,
                    id,
                });
                let mut temp = String::new();
                html::push_html(&mut temp, std::iter::once(new_event));
                html_output.push_str(&temp);
            }
            Event::End(TagEnd::Link) => {
                in_link = false;
                html_output.push_str("</a>");
            }
            Event::Text(text) => {
                if in_code_block {
                    code_block_content.push_str(&text);
                } else if in_image {
                    image_alt_text.push_str(&text);
                } else if in_link {
                    let mut escaped = String::new();
                    escape_html(&mut escaped, &text).unwrap();
                    html_output.push_str(&escaped);
                } else {
                    html_output.push_str(&linkify_bare_urls(&text));
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
                html_output.push_str(&format!(
                    r#"<code class="language-text">{}</code>"#,
                    escaped
                ));
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
            Event::InlineMath(latex) => match math::to_mathml(&latex, DisplayStyle::Inline) {
                Some(mathml) => html_output.push_str(&mathml),
                None => {
                    let mut escaped = String::new();
                    escape_html(&mut escaped, &latex).unwrap();
                    html_output.push_str(&format!(
                        r#"<code class="language-text">{}</code>"#,
                        escaped
                    ));
                }
            },
            Event::DisplayMath(latex) => match math::to_mathml(&latex, DisplayStyle::Block) {
                Some(mathml) => {
                    html_output.push_str(&format!(r#"<div class="math-block">{}</div>"#, mathml))
                }
                None => {
                    let mut escaped = String::new();
                    escape_html(&mut escaped, &latex).unwrap();
                    html_output.push_str(&format!(
                        r#"<pre class="language-text"><code>{}</code></pre>"#,
                        escaped
                    ));
                }
            },
            Event::SoftBreak => {
                html_output.push_str("<br>");
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
    let result = SYNTAX_CLASS_RE.replace_all(&result, |caps: &regex::Captures| {
        let class_content = &caps[1];

        // Determine the appropriate Prism token class based on syntect scopes
        let token_class = if class_content.contains("keyword") || class_content.contains("storage")
        {
            "token keyword"
        } else if class_content.contains("string") {
            "token string"
        } else if class_content.contains("comment") {
            "token comment"
        } else if class_content.contains("constant") && class_content.contains("numeric") {
            "token number"
        } else if class_content.contains("entity.name.function")
            || class_content.contains("support.function")
        {
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
    HEADING_RE
        .replace_all(html, |caps: &regex::Captures| {
            let level = &caps[1];
            let content = &caps[2];
            let plain_text = strip_html_tags(content);
            let text = decode_html_entities(&plain_text);
            let id = heading_id_from_text(&text);
            format!(r#"<h{} id="{}">{}</h{}>"#, level, id, content, level)
        })
        .to_string()
}

fn heading_id_from_text(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .map(|character| {
            if character.is_alphanumeric() {
                character
            } else {
                '-'
            }
        })
        .collect::<String>()
        .split('-')
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

#[derive(Debug, Clone, Serialize)]
pub struct Heading {
    pub level: u8,
    pub text: String,
    pub id: String,
}

pub fn extract_headings(html: &str) -> Vec<Heading> {
    HEADING_WITH_ATTRS_RE
        .captures_iter(html)
        .map(|caps| {
            let level = caps[1].parse::<u8>().unwrap_or(1);
            let attributes = &caps[2];
            let raw_text = &caps[3];
            let plain_text = strip_html_tags(raw_text);
            let text = decode_html_entities(&plain_text);
            let id = ID_ATTR_RE
                .captures(attributes)
                .map(|id_caps| id_caps[1].to_string())
                .unwrap_or_else(|| heading_id_from_text(&text));
            Heading { level, text, id }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::extract_headings;

    #[test]
    fn extract_headings_preserves_explicit_ids() {
        let headings =
            extract_headings(r#"<h2 id="outlook">Outlook: why this trick stops at the plane</h2>"#);

        assert_eq!(headings.len(), 1);
        assert_eq!(headings[0].id, "outlook");
    }

    #[test]
    fn extract_headings_generates_ids_when_missing() {
        let headings = extract_headings("<h2>Why this trick works</h2>");

        assert_eq!(headings.len(), 1);
        assert_eq!(headings[0].id, "why-this-trick-works");
    }
}

fn decode_html_entities(text: &str) -> String {
    ENTITY_RE
        .replace_all(text, |caps: &regex::Captures| {
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
    HTML_TAG_RE.replace_all(html, "").to_string()
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
        let links = HASHTAG_RE.replace_all(hashtags_text, |c: &regex::Captures| {
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

fn linkify_bare_urls(text: &str) -> String {
    // Match http/https URLs; stop before trailing punctuation that's likely not part of the URL
    let mut result = String::with_capacity(text.len());
    let mut last = 0;

    for caps in URL_RE.captures_iter(text) {
        let full_match = caps.get(0).unwrap();
        let url = &caps[1];
        let trailing = &caps[2];

        // Escape plain text before this URL
        let before = &text[last..full_match.start()];
        let mut escaped_before = String::new();
        escape_html(&mut escaped_before, before).unwrap();
        result.push_str(&escaped_before);

        // Emit link (URL is already a valid URL, no escaping needed for href)
        result.push_str(&format!(r#"<a href="{}">{}</a>"#, url, url));

        // Preserve trailing punctuation/whitespace as plain text
        let mut escaped_trailing = String::new();
        escape_html(&mut escaped_trailing, trailing).unwrap();
        result.push_str(&escaped_trailing);

        last = full_match.end();
    }

    // Escape remaining text after last URL
    let remainder = &text[last..];
    let mut escaped_remainder = String::new();
    escape_html(&mut escaped_remainder, remainder).unwrap();
    result.push_str(&escaped_remainder);

    result
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
