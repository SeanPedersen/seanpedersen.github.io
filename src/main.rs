mod class_minifier;
mod global_generation;
mod index_generation;
mod optimization;
mod page_generation;
mod post_generation;
mod rss_generation;
mod similarity;

use anyhow::Result;
use clap::Parser;
use global_generation::build_global_html_pages;
use index_generation::build_index_page;
use post_generation::build_post_pages;
use rss_generation::build_rss_feed;
use std::path::PathBuf;
use std::time::Instant;

#[derive(Parser)]
#[command(name = "blog-builder")]
struct Cli {
    /// Use model2vec embeddings for smarter related-post recommendations
    #[arg(long)]
    smart_similar: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let total_start = Instant::now();
    let out_dir = PathBuf::from("out");

    println!("Starting static site generation...\n");
    setup_output_directory(&out_dir)?;

    // Build pipeline
    let posts = page_generation::get_posts_data(&out_dir)?;

    let similar_map = if cli.smart_similar {
        Some(similarity::compute_similar_posts(&posts)?)
    } else {
        None
    };

    build_index_page(&out_dir, &posts)?;
    build_post_pages(&out_dir, &posts, similar_map.as_ref())?;
    build_global_html_pages(&out_dir)?;
    build_rss_feed(&out_dir, &posts)?;
    optimization::optimize_website_assets(&out_dir)?;

    println!("\nCompleted in {:.2}s", total_start.elapsed().as_secs_f64());
    println!("Output directory: {}/", out_dir.display());

    Ok(())
}

fn setup_output_directory(out_dir: &PathBuf) -> Result<()> {
    if out_dir.exists() {
        std::fs::remove_dir_all(out_dir)?;
    }
    std::fs::create_dir_all(out_dir)?;
    Ok(())
}
