use anyhow::Result;
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;

static CSS_CLASS_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\.([a-zA-Z_][a-zA-Z0-9_-]*)").unwrap());
static HTML_CLASS_ATTR_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"class\s*=\s*["']([^"']*)["']"#).unwrap());
static HTML_STYLE_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?s)<style[^>]*>(.*?)</style>").unwrap());
static HTML_SCRIPT_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?s)<script[^>]*>(.*?)</script>").unwrap());
static JS_SELECTOR_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"(['"`])\.([a-zA-Z_][a-zA-Z0-9_-]*)"#).unwrap());
static JS_SPACED_SELECTOR_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"( )\.([a-zA-Z_][a-zA-Z0-9_-]*)"#).unwrap());
static JS_CLASSLIST_SINGLE_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"classList\.(add|remove|toggle|contains)\s*\(\s*'([^']*)'\s*"#).unwrap()
});
static JS_CLASSLIST_DOUBLE_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"classList\.(add|remove|toggle|contains)\s*\(\s*\"([^\"]*)\"\s*"#).unwrap()
});
static JS_CLASSNAME_SINGLE_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"\.className\s*=\s*'([^']*)'"#).unwrap());
static JS_CLASSNAME_DOUBLE_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"\.className\s*=\s*\"([^\"]*)\""#).unwrap());
static JS_CLASS_ATTR_DOUBLE_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"class=\"([^\"]*)\""#).unwrap());
static JS_CLASS_ATTR_SINGLE_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"class='([^']*)'"#).unwrap());
static JS_TERNARY_SINGLE_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"([?:])\s*'([a-zA-Z_][a-zA-Z0-9_-]*)'"#).unwrap());
static JS_TERNARY_DOUBLE_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"([?:])\s*\"([a-zA-Z_][a-zA-Z0-9_-]*)\""#).unwrap());

/// Generates short class names: a, b, c, ..., z, A, B, ..., Z, aa, ab, ...
fn generate_short_name(index: usize) -> String {
    const CHARS: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let base = CHARS.len();

    if index < base {
        return (CHARS[index] as char).to_string();
    }

    let mut n = index;
    let mut bytes = Vec::new();

    while n >= base {
        bytes.push(CHARS[n % base]);
        n = n / base - 1;
    }
    bytes.push(CHARS[n]);
    bytes.reverse();

    String::from_utf8(bytes).unwrap()
}

/// Extract all CSS class names from CSS content
fn extract_classes_from_css(css: &str) -> HashSet<String> {
    let mut classes = HashSet::new();

    // Match class selectors: .classname (followed by various things like :, [, ., {, space, comma, >)
    // This regex captures the class name after a dot, stopping at selector boundaries
    for cap in CSS_CLASS_RE.captures_iter(css) {
        let class_name = &cap[1];
        // Skip classes that look like decimal numbers (e.g., .5rem -> skip "5rem")
        if !class_name.chars().next().unwrap_or('0').is_ascii_digit() {
            classes.insert(class_name.to_string());
        }
    }

    classes
}

/// Build a mapping from original class names to minified names
fn build_class_map(classes: &HashSet<String>) -> HashMap<String, String> {
    let mut sorted_classes: Vec<_> = classes.iter().collect();
    // Sort by frequency of use (longer names save more bytes) or alphabetically for determinism
    sorted_classes.sort();

    sorted_classes
        .into_iter()
        .enumerate()
        .map(|(i, class)| (class.clone(), generate_short_name(i)))
        .collect()
}

/// Replace class names in CSS content
fn replace_classes_in_css(css: &str, class_map: &HashMap<String, String>) -> String {
    // Use regex with word boundary to match complete class names only
    CSS_CLASS_RE
        .replace_all(css, |caps: &regex::Captures| {
            let class_name = &caps[1];
            if let Some(short_name) = class_map.get(class_name) {
                format!(".{}", short_name)
            } else {
                caps[0].to_string()
            }
        })
        .to_string()
}

/// Replace class names in HTML class attributes
fn replace_classes_in_html(html: &str, class_map: &HashMap<String, String>) -> String {
    // Match class="..." or class='...'
    let result = HTML_CLASS_ATTR_RE.replace_all(html, |caps: &regex::Captures| {
        let classes_str = &caps[1];
        if let Some(replaced) = replace_space_separated_classes(classes_str, class_map) {
            format!("class=\"{}\"", replaced)
        } else {
            caps[0].to_string()
        }
    });

    result.to_string()
}

/// Replace class names in JavaScript code
/// Handles:
/// - CSS selectors in strings: '.container .item', '.tag'
/// - classList operations: classList.add('className'), classList.toggle('className')
/// - className assignments: className = 'class1 class2'
fn replace_classes_in_js(js: &str, class_map: &HashMap<String, String>) -> String {
    // Use regex-based replacements for safety - avoids corrupting nested strings

    // 1. Replace .className patterns in querySelector-style strings
    // Match: '.className' or ".className" where className is a known class
    let result = JS_SELECTOR_RE.replace_all(js, |caps: &regex::Captures| {
        let quote = &caps[1];
        let class_name = &caps[2];
        if let Some(minified) = class_map.get(class_name) {
            format!("{}.{}", quote, minified)
        } else {
            caps[0].to_string()
        }
    });

    // 2. Replace class names after space in selectors: ' .className'
    let result = JS_SPACED_SELECTOR_RE.replace_all(&result, |caps: &regex::Captures| {
        let space = &caps[1];
        let class_name = &caps[2];
        if let Some(minified) = class_map.get(class_name) {
            format!("{}.{}", space, minified)
        } else {
            caps[0].to_string()
        }
    });

    // 3. Replace classList.add/remove/toggle/contains('className') - single quotes
    let result = JS_CLASSLIST_SINGLE_RE.replace_all(&result, |caps: &regex::Captures| {
        let method = &caps[1];
        let classes_str = &caps[2];
        if let Some(replaced) = replace_space_separated_classes(classes_str, class_map) {
            format!("classList.{}('{}'", method, replaced)
        } else {
            caps[0].to_string()
        }
    });

    // 4. Replace classList.add/remove/toggle/contains("className") - double quotes
    let result = JS_CLASSLIST_DOUBLE_RE.replace_all(&result, |caps: &regex::Captures| {
        let method = &caps[1];
        let classes_str = &caps[2];
        if let Some(replaced) = replace_space_separated_classes(classes_str, class_map) {
            format!("classList.{}(\"{}\"", method, replaced)
        } else {
            caps[0].to_string()
        }
    });

    // 5. Replace className = 'class1 class2' assignments - single quotes
    let result = JS_CLASSNAME_SINGLE_RE.replace_all(&result, |caps: &regex::Captures| {
        let classes_str = &caps[1];
        if let Some(replaced) = replace_space_separated_classes(classes_str, class_map) {
            format!(".className = '{}'", replaced)
        } else {
            caps[0].to_string()
        }
    });

    // 6. Replace className = "class1 class2" assignments - double quotes
    let result = JS_CLASSNAME_DOUBLE_RE.replace_all(&result, |caps: &regex::Captures| {
        let classes_str = &caps[1];
        if let Some(replaced) = replace_space_separated_classes(classes_str, class_map) {
            format!(".className = \"{}\"", replaced)
        } else {
            caps[0].to_string()
        }
    });

    // 7. Replace class="className" in HTML template strings (double quotes)
    let result = JS_CLASS_ATTR_DOUBLE_RE.replace_all(&result, |caps: &regex::Captures| {
        let classes_str = &caps[1];
        if let Some(replaced) = replace_space_separated_classes(classes_str, class_map) {
            format!("class=\"{}\"", replaced)
        } else {
            caps[0].to_string()
        }
    });

    // 8. Replace class='className' in HTML template strings (single quotes)
    let result = JS_CLASS_ATTR_SINGLE_RE.replace_all(&result, |caps: &regex::Captures| {
        let classes_str = &caps[1];
        if let Some(replaced) = replace_space_separated_classes(classes_str, class_map) {
            format!("class='{}'", replaced)
        } else {
            caps[0].to_string()
        }
    });

    // 9. Replace class names in ternary expression results (? 'className' or : 'className')
    // This handles patterns like: isExpanded ? 'copyButtonExpanded' : 'copyButton'
    let result = JS_TERNARY_SINGLE_RE.replace_all(&result, |caps: &regex::Captures| {
        let op = &caps[1];
        let class_name = &caps[2];
        if let Some(minified) = class_map.get(class_name) {
            format!("{} '{}'", op, minified)
        } else {
            caps[0].to_string()
        }
    });

    // 10. Replace class names in ternary expression results (double quotes)
    let result = JS_TERNARY_DOUBLE_RE.replace_all(&result, |caps: &regex::Captures| {
        let op = &caps[1];
        let class_name = &caps[2];
        if let Some(minified) = class_map.get(class_name) {
            format!("{} \"{}\"", op, minified)
        } else {
            caps[0].to_string()
        }
    });

    result.to_string()
}

/// Replace class names in inline <style> tags within HTML
fn replace_classes_in_inline_styles(html: &str, class_map: &HashMap<String, String>) -> String {
    HTML_STYLE_RE
        .replace_all(html, |caps: &regex::Captures| {
            let style_content = &caps[1];
            let replaced = replace_classes_in_css(style_content, class_map);
            format!("<style>{}</style>", replaced)
        })
        .to_string()
}

/// Replace class names in inline <script> tags within HTML
fn replace_classes_in_inline_scripts(html: &str, class_map: &HashMap<String, String>) -> String {
    HTML_SCRIPT_RE
        .replace_all(html, |caps: &regex::Captures| {
            let script_content = &caps[1];
            // Extract the opening tag to preserve attributes
            let full_match = &caps[0];
            let opening_tag_end = full_match.find('>').unwrap() + 1;
            let opening_tag = &full_match[..opening_tag_end];

            let replaced = replace_classes_in_js(script_content, class_map);
            format!("{}{}</script>", opening_tag, replaced)
        })
        .to_string()
}

fn replace_space_separated_classes(
    classes_str: &str,
    class_map: &HashMap<String, String>,
) -> Option<String> {
    let mut changed = false;
    let mut out = String::with_capacity(classes_str.len());
    for (i, class) in classes_str.split_whitespace().enumerate() {
        if i > 0 {
            out.push(' ');
        }
        if let Some(minified) = class_map.get(class) {
            changed = true;
            out.push_str(minified);
        } else {
            out.push_str(class);
        }
    }

    if changed {
        Some(out)
    } else {
        None
    }
}

/// Main entry point: minify CSS class names across all assets
pub fn minify_css_classes(
    css_files: &[std::path::PathBuf],
    html_files: &[std::path::PathBuf],
    js_files: &[std::path::PathBuf],
) -> Result<()> {
    // Step 1: Extract all class names from CSS files
    let mut all_classes = HashSet::new();

    for css_path in css_files {
        let css = fs::read_to_string(css_path)?;
        let classes = extract_classes_from_css(&css);
        all_classes.extend(classes);
    }

    if all_classes.is_empty() {
        return Ok(());
    }

    // Step 2: Build the mapping
    let class_map = build_class_map(&all_classes);

    // Step 3: Replace in CSS files
    for css_path in css_files {
        let css = fs::read_to_string(css_path)?;
        let replaced = replace_classes_in_css(&css, &class_map);
        fs::write(css_path, replaced)?;
    }

    // Step 4: Replace in HTML files (class attributes, inline styles, inline scripts)
    for html_path in html_files {
        let html = fs::read_to_string(html_path)?;
        let html = replace_classes_in_html(&html, &class_map);
        let html = replace_classes_in_inline_styles(&html, &class_map);
        let html = replace_classes_in_inline_scripts(&html, &class_map);
        fs::write(html_path, html)?;
    }

    // Step 5: Replace in external JS files
    for js_path in js_files {
        let js = fs::read_to_string(js_path)?;
        let replaced = replace_classes_in_js(&js, &class_map);
        fs::write(js_path, replaced)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_short_name() {
        assert_eq!(generate_short_name(0), "a");
        assert_eq!(generate_short_name(25), "z");
        assert_eq!(generate_short_name(26), "A");
        assert_eq!(generate_short_name(51), "Z");
        assert_eq!(generate_short_name(52), "aa");
        assert_eq!(generate_short_name(53), "ab");
    }

    #[test]
    fn test_extract_classes() {
        let css = ".container { } .foo-bar:hover { } .baz.qux { }";
        let classes = extract_classes_from_css(css);
        assert!(classes.contains("container"));
        assert!(classes.contains("foo-bar"));
        assert!(classes.contains("baz"));
        assert!(classes.contains("qux"));
    }

    #[test]
    fn test_replace_css() {
        let mut map = HashMap::new();
        map.insert("container".to_string(), "a".to_string());
        map.insert("flex".to_string(), "b".to_string());

        let css = ".container { display: flex; } .container.flex { }";
        let result = replace_classes_in_css(css, &map);
        assert_eq!(result, ".a { display: flex; } .a.b { }");
    }

    #[test]
    fn test_replace_html_classes() {
        let mut map = HashMap::new();
        map.insert("container".to_string(), "a".to_string());
        map.insert("flex".to_string(), "b".to_string());

        let html = r#"<div class="container flex">test</div>"#;
        let result = replace_classes_in_html(html, &map);
        assert_eq!(result, r#"<div class="a b">test</div>"#);
    }

    #[test]
    fn test_replace_js_classes() {
        let mut map = HashMap::new();
        map.insert("isHidden".to_string(), "a".to_string());

        let js = r#"el.classList.add('isHidden'); el.classList.toggle("isHidden");"#;
        let result = replace_classes_in_js(js, &map);
        assert_eq!(
            result,
            r#"el.classList.add('a'); el.classList.toggle("a");"#
        );
    }

    #[test]
    fn test_replace_js_selectors() {
        let mut map = HashMap::new();
        map.insert("container".to_string(), "a".to_string());
        map.insert("tag".to_string(), "b".to_string());

        let js = r#"document.querySelectorAll('.container .tag');"#;
        let result = replace_classes_in_js(js, &map);
        assert_eq!(result, r#"document.querySelectorAll('.a .b');"#);
    }

    #[test]
    fn test_does_not_replace_dom_classname() {
        let mut map = HashMap::new();
        map.insert("className".to_string(), "a".to_string());

        let js = r#"element.className = 'foo';"#;
        let result = replace_classes_in_js(js, &map);
        // Should NOT replace element.className (DOM property)
        assert_eq!(result, r#"element.className = 'foo';"#);
    }

    #[test]
    fn test_replace_ternary_class_names() {
        let mut map = HashMap::new();
        map.insert("copyButtonExpanded".to_string(), "a".to_string());
        map.insert("copyButton".to_string(), "b".to_string());
        map.insert("copyButtonNormal".to_string(), "c".to_string());

        let js = r#"btn.className = isExpandable ? (isExpanded ? 'copyButtonExpanded' : 'copyButton') : 'copyButtonNormal';"#;
        let result = replace_classes_in_js(js, &map);
        assert_eq!(
            result,
            r#"btn.className = isExpandable ? (isExpanded ? 'a' : 'b') : 'c';"#
        );
    }

    #[test]
    fn test_replace_space_separated_classes() {
        let mut map = HashMap::new();
        map.insert("codeBlock".to_string(), "a".to_string());
        map.insert("collapsed".to_string(), "b".to_string());

        let js = r#"el.className = 'codeBlock collapsed';"#;
        let result = replace_classes_in_js(js, &map);
        assert_eq!(result, r#"el.className = 'a b';"#);
    }
}
