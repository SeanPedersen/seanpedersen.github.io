use anyhow::Result;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::sync::Arc;
use std::time::Instant;

use crate::page_generation::Post;

const BASE_URL: &str = "https://seanpedersen.github.io";

pub fn build_sitemap_and_extras(out_dir: &Path, posts: &Arc<Vec<Post>>) -> Result<()> {
    let start = Instant::now();

    generate_sitemap(out_dir, posts)?;
    generate_robots_txt(out_dir)?;
    generate_llms_txt(out_dir, posts)?;

    println!(
        "✓ Generated sitemap.xml, robots.txt, llms.txt in {:.2}s",
        start.elapsed().as_secs_f64()
    );
    Ok(())
}

fn generate_sitemap(out_dir: &Path, posts: &Arc<Vec<Post>>) -> Result<()> {
    let path = out_dir.join("sitemap.xml");
    let mut file = BufWriter::new(File::create(path)?);

    write!(
        file,
        r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
  <url>
    <loc>{BASE_URL}/</loc>
    <changefreq>weekly</changefreq>
    <priority>1.0</priority>
  </url>
"#
    )?;

    for post in posts.iter() {
        write!(
            file,
            "  <url>\n    <loc>{BASE_URL}/posts/{}/</loc>\n    <lastmod>{}</lastmod>\n    <changefreq>monthly</changefreq>\n    <priority>0.8</priority>\n  </url>\n",
            post.id, post.date
        )?;
    }

    write!(file, "</urlset>")?;
    Ok(())
}

fn generate_robots_txt(out_dir: &Path) -> Result<()> {
    let path = out_dir.join("robots.txt");
    let mut file = BufWriter::new(File::create(path)?);

    write!(
        file,
        "User-agent: *\nAllow: /\n\nSitemap: {BASE_URL}/sitemap.xml\n"
    )?;
    Ok(())
}

fn generate_llms_txt(out_dir: &Path, posts: &Arc<Vec<Post>>) -> Result<()> {
    let path = out_dir.join("llms.txt");
    let mut file = BufWriter::new(File::create(path)?);

    write!(
        file,
        "# Sean Pedersen's Blog\n\n> Personal blog exploring Machine Learning, Data Privacy, Cybernetics and Memetics.\n\n## Posts\n\n"
    )?;

    for post in posts.iter() {
        writeln!(file, "- [{}]({}/posts/{}/)", post.title, BASE_URL, post.id)?;
    }

    Ok(())
}
