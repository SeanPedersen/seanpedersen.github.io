use anyhow::Result;
use chrono::Datelike;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::time::Instant;
use tera::{Tera, Value};

/// Tera function that outputs a placeholder for CSS inlining.
/// Usage in template: {{ inline_css(path="/styles/global.css") }}
/// The placeholder is replaced with actual CSS content during optimization.
fn inline_css_placeholder(args: &std::collections::HashMap<String, Value>) -> tera::Result<Value> {
    let path = args
        .get("path")
        .and_then(|v| v.as_str())
        .ok_or_else(|| tera::Error::msg("inline_css requires a 'path' argument"))?;
    Ok(Value::String(format!("<!-- INLINE_CSS:{} -->", path)))
}

pub fn build_global_html_pages(out_dir: &Path) -> Result<()> {
    let start = Instant::now();

    let global_dir = Path::new("website/global");
    if !global_dir.exists() {
        return Ok(());
    }

    let mut tera = Tera::new("website/global/**/*")?;
    tera.register_function("inline_css", inline_css_placeholder);

    let year = chrono::Local::now().year();

    let mut context = tera::Context::new();
    context.insert("year", &year);

    let mut count = 0;
    for entry in std::fs::read_dir(global_dir)? {
        let entry = entry?;
        let path = entry.path();
        if let Some(ext) = path.extension() {
            if ext == "html" {
                if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                    let html = tera.render(file_name, &context)?;
                    let mut file = BufWriter::new(File::create(out_dir.join(file_name))?);
                    write!(file, "{}", html)?;
                    count += 1;
                }
            }
        }
    }

    if count > 0 {
        println!(
            "✓ Generated {} global HTML pages in {:.2}s",
            count,
            start.elapsed().as_secs_f64()
        );
    }

    Ok(())
}
