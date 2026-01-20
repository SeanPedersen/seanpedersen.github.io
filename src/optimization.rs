use anyhow::Result;
use lightningcss::stylesheet::{ParserOptions, PrinterOptions, StyleSheet};
use minify_html::{minify, Cfg};
use minify_js::minify as minify_js_code;
use minify_js::{Session, TopLevelMode};
use rayon::prelude::*;
use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;

pub fn optimize_website_assets(out_dir: &Path) -> Result<()> {
    copy_static_assets(out_dir)?;
    optimize_assets(out_dir)?;

    Ok(())
}

pub fn copy_static_assets(out_dir: &Path) -> Result<()> {
    // Copy images
    let images_src = Path::new("website/images");
    if images_src.exists() {
        let images_dest = out_dir.join("images");
        copy_dir_recursive(images_src, &images_dest)?;
    }

    // Copy favicons
    for favicon in &["favicon.svg", "favicon.ico"] {
        let favicon_src = Path::new("website").join(favicon);
        if favicon_src.exists() {
            fs::copy(&favicon_src, out_dir.join(favicon))?;
        }
    }

    // Create styles directory and copy CSS
    let styles_src = Path::new("website/styles");
    if styles_src.exists() {
        let styles_dest = out_dir.join("styles");
        copy_dir_recursive(styles_src, &styles_dest)?;
    }

    // Copy JS files
    let js_out = out_dir.join("js");
    fs::create_dir_all(&js_out)?;

    let js_src = Path::new("website/js");
    if js_src.exists() {
        copy_dir_recursive(js_src, &js_out)?;
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

pub fn optimize_assets(out_dir: &Path) -> Result<()> {
    let start = Instant::now();
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

    // Calculate total size before optimization
    let before_size: u64 = [&css_files, &js_files, &html_files, &image_files]
        .iter()
        .flat_map(|files| files.iter())
        .filter_map(|path| fs::metadata(path).ok())
        .map(|meta| meta.len())
        .sum();

    // Optimize in phases:
    // 1. Minify CSS and JS files (in parallel)
    // 2. Inline CSS into HTML files (requires minified CSS)
    // 3. Minify HTML files (after CSS inlining)
    let css_count = AtomicUsize::new(0);
    let js_count = AtomicUsize::new(0);
    let html_count = AtomicUsize::new(0);
    let img_count = AtomicUsize::new(0);
    let inline_count = AtomicUsize::new(0);

    // Phase 1: Minify CSS, JS, and optimize images in parallel
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
            image_files.par_iter().for_each(|path| {
                if optimize_image_file(path).is_ok() {
                    img_count.fetch_add(1, Ordering::Relaxed);
                }
            });
        });
    });

    // Phase 2: Inline CSS into HTML files (now that CSS is minified)
    html_files.par_iter().for_each(|path| {
        if inline_css_in_html_file(out_dir, path).is_ok() {
            inline_count.fetch_add(1, Ordering::Relaxed);
        }
    });

    // Phase 3: Minify HTML files (after CSS has been inlined)
    html_files.par_iter().for_each(|path| {
        if minify_html_file(path).is_ok() {
            html_count.fetch_add(1, Ordering::Relaxed);
        }
    });

    // Calculate total size after optimization
    let after_size: u64 = [&css_files, &js_files, &html_files, &image_files]
        .iter()
        .flat_map(|files| files.iter())
        .filter_map(|path| fs::metadata(path).ok())
        .map(|meta| meta.len())
        .sum();

    let saved = before_size.saturating_sub(after_size);
    let percent_saved = if before_size > 0 {
        (saved as f64 / before_size as f64) * 100.0
    } else {
        0.0
    };

    println!(
        "✓ Optimized {} assets in {:.2}s",
        total_files,
        start.elapsed().as_secs_f64()
    );
    println!(
        "  ({:.2} KB → {:.2} KB, saved {:.1}%)",
        before_size as f64 / 1024.0,
        after_size as f64 / 1024.0,
        percent_saved
    );
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
        // Don't use keep_closing_tags or keep_spaces_between_attributes
        // as they interfere with anti-flicker style handling
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

/// Inline CSS files into HTML by replacing <!-- INLINE_CSS:/path/to/file.css --> placeholders.
/// This should be called AFTER CSS minification so the inlined CSS is already minified.
fn inline_css_in_html_file(out_dir: &Path, html_path: &Path) -> Result<()> {
    let html = fs::read_to_string(html_path)?;

    // Match placeholders like <!-- INLINE_CSS:/styles/global.css -->
    let re = Regex::new(r"<!-- INLINE_CSS:(/[^>]+\.css) -->")?;

    let result = re.replace_all(&html, |caps: &regex::Captures| {
        let css_path = &caps[1];
        // Convert absolute URL path to filesystem path (remove leading /)
        let full_css_path = out_dir.join(&css_path[1..]);

        match fs::read_to_string(&full_css_path) {
            Ok(css) => format!("<style>{}</style>", css),
            Err(e) => {
                eprintln!("Warning: Could not inline CSS {}: {}", css_path, e);
                caps[0].to_string() // Keep placeholder if file not found
            }
        }
    });

    if result != html {
        fs::write(html_path, result.as_bytes())?;
    }

    Ok(())
}
