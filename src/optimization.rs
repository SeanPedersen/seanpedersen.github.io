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

pub fn copy_static_assets(out_dir: &Path) -> Result<()> {
    // Copy images
    let images_src = Path::new("website/images");
    if images_src.exists() {
        let images_dest = out_dir.join("images");
        copy_dir_recursive(images_src, &images_dest)?;
        println!("  ✓ Copied images/");
    }

    // Copy favicon
    let favicon_src = Path::new("website/favicon.ico");
    if favicon_src.exists() {
        fs::copy(favicon_src, out_dir.join("favicon.ico"))?;
        println!("  ✓ Copied favicon.ico");
    }

    // Create styles directory and copy CSS
    let styles_out = out_dir.join("styles");
    fs::create_dir_all(&styles_out)?;

    fs::copy("website/styles/global.css", styles_out.join("global.css"))?;
    println!("  ✓ Copied global.css");

    fs::copy(
        "node_modules/prismjs/themes/prism-tomorrow.css",
        styles_out.join("prism-tomorrow.css"),
    )?;
    println!("  ✓ Copied prism-tomorrow.css");

    fs::copy("website/styles/utils.module.css", styles_out.join("utils.css"))?;
    println!("  ✓ Copied utils.css");

    let layout_css = fs::read_to_string("website/styles/layout.module.css")?;
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

    let js_src = Path::new("website/js");
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

pub fn optimize_assets(out_dir: &Path) -> Result<()> {
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
