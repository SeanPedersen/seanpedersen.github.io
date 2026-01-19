mod index_generation;
mod optimization;
mod page_generation;
mod post_generation;
mod rss_generation;

use anyhow::Result;
use index_generation::build_index_page;
use post_generation::build_post_pages;
use rss_generation::build_rss_feed;
use std::path::PathBuf;
use std::time::Instant;

fn main() -> Result<()> {
    let total_start = Instant::now();
    let out_dir = PathBuf::from("out");

    println!("Starting static site generation...\n");
    setup_output_directory(&out_dir)?;

    // Build pipeline
    let posts = page_generation::get_posts_data(&out_dir)?;
    build_index_page(&out_dir, &posts)?;
    build_post_pages(&out_dir, &posts)?;
    build_rss_feed(&out_dir, &posts)?;
    optimization::optimize_website_assets(&out_dir)?;

    println!("\nComplete in {:.2}s!", total_start.elapsed().as_secs_f64());
    println!("   Output directory: {}", out_dir.display());

    Ok(())
}

fn setup_output_directory(out_dir: &PathBuf) -> Result<()> {
    if out_dir.exists() {
        std::fs::remove_dir_all(out_dir)?;
    }
    std::fs::create_dir_all(out_dir)?;
    Ok(())
}
