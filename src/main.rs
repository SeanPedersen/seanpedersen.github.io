mod index_generation;
mod optimization;
mod page_generation;
mod post_generation;
mod rss_generation;

use anyhow::Result;
use index_generation::generate_index_page;
use page_generation::{extract_all_tags, read_all_posts, PostSummary};
use post_generation::generate_all_post_pages;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;

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

    let start_posts = Instant::now();
    generate_all_post_pages(&posts_out_dir, &posts_arc, total_posts)?;
    let posts_duration = start_posts.elapsed().as_secs_f64();
    println!(
        "✓ Generated {} post pages in {:.2}s\n",
        total_posts, posts_duration
    );

    // Copy static assets
    println!("Copying static assets...");
    optimization::copy_static_assets(&out_dir)?;

    // Generate RSS feed (before optimization to get clean, unminified HTML)
    println!("\nGenerating RSS feed...");
    let rss_start = Instant::now();
    rss_generation::generate_rss(&out_dir, &posts_arc)?;
    let rss_duration = rss_start.elapsed().as_secs_f64();
    println!("✓ Generated RSS feed in {:.2}s", rss_duration);

    // Optimize assets
    println!("\nOptimizing assets...");
    let optimize_start = Instant::now();
    optimization::optimize_assets(&out_dir)?;
    let optimize_duration = optimize_start.elapsed().as_secs_f64();
    println!("✓ Optimized assets in {:.2}s\n", optimize_duration);

    let total_duration = total_start.elapsed().as_secs_f64();
    println!("\n✅ Build complete in {:.2}s!", total_duration);
    println!("   Output directory: {}", out_dir.display());
    println!("   Total files: {} HTML pages + RSS feed", total_posts + 1);

    Ok(())
}
