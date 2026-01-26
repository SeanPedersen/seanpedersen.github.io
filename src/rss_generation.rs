use anyhow::Result;
use chrono::NaiveDate;
use regex::Regex;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::Path;
use std::sync::Arc;
use std::time::Instant;

use crate::page_generation::Post;

pub fn build_rss_feed(out_dir: &Path, posts: &Arc<Vec<Post>>) -> Result<()> {
    let start = Instant::now();

    generate_rss(out_dir, posts)?;

    println!(
        "✓ Generated RSS feed in {:.2}s",
        start.elapsed().as_secs_f64()
    );
    Ok(())
}

pub fn generate_rss(out_dir: &Path, posts: &Arc<Vec<Post>>) -> Result<()> {
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
        let html_path = out_dir.join("posts").join(&post.id).join("index.html");
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
