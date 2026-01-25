use anyhow::Result;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;

/// Generates short class names: a, b, c, ..., z, A, B, ..., Z, aa, ab, ...
fn generate_short_name(index: usize) -> String {
    const CHARS: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let base = CHARS.len();

    if index < base {
        return (CHARS[index] as char).to_string();
    }

    let mut result = String::new();
    let mut n = index;

    while n >= base {
        result.insert(0, CHARS[n % base] as char);
        n = n / base - 1;
    }
    result.insert(0, CHARS[n] as char);

    result
}

/// Extract all CSS class names from CSS content
fn extract_classes_from_css(css: &str) -> HashSet<String> {
    let mut classes = HashSet::new();

    // Match class selectors: .classname (followed by various things like :, [, ., {, space, comma, >)
    // This regex captures the class name after a dot, stopping at selector boundaries
    let class_re = Regex::new(r"\.([a-zA-Z_][a-zA-Z0-9_-]*)").unwrap();

    for cap in class_re.captures_iter(css) {
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
    let class_re = Regex::new(r"\.([a-zA-Z_][a-zA-Z0-9_-]*)").unwrap();

    class_re
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
    let class_attr_re = Regex::new(r#"class\s*=\s*["']([^"']*)["']"#).unwrap();

    let result = class_attr_re.replace_all(html, |caps: &regex::Captures| {
        let classes_str = &caps[1];
        let replaced: Vec<_> = classes_str
            .split_whitespace()
            .map(|class| {
                class_map
                    .get(class)
                    .map(|s| s.as_str())
                    .unwrap_or(class)
            })
            .collect();
        format!("class=\"{}\"", replaced.join(" "))
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
    let selector_re = Regex::new(r#"(['"`])\.([a-zA-Z_][a-zA-Z0-9_-]*)"#).unwrap();
    let result = selector_re.replace_all(js, |caps: &regex::Captures| {
        let quote = &caps[1];
        let class_name = &caps[2];
        if let Some(minified) = class_map.get(class_name) {
            format!("{}.{}", quote, minified)
        } else {
            caps[0].to_string()
        }
    });

    // 2. Replace class names after space in selectors: ' .className'
    let spaced_selector_re = Regex::new(r#"( )\.([a-zA-Z_][a-zA-Z0-9_-]*)"#).unwrap();
    let result = spaced_selector_re.replace_all(&result, |caps: &regex::Captures| {
        let space = &caps[1];
        let class_name = &caps[2];
        if let Some(minified) = class_map.get(class_name) {
            format!("{}.{}", space, minified)
        } else {
            caps[0].to_string()
        }
    });

    // 3. Replace classList.add/remove/toggle/contains('className') - single quotes
    let classlist_single_re =
        Regex::new(r#"classList\.(add|remove|toggle|contains)\s*\(\s*'([^']*)'\s*\)"#).unwrap();
    let result = classlist_single_re.replace_all(&result, |caps: &regex::Captures| {
        let method = &caps[1];
        let classes_str = &caps[2];
        let replaced: Vec<_> = classes_str
            .split_whitespace()
            .map(|class| class_map.get(class).map(|s| s.as_str()).unwrap_or(class))
            .collect();
        format!("classList.{}('{}')", method, replaced.join(" "))
    });

    // 4. Replace classList.add/remove/toggle/contains("className") - double quotes
    let classlist_double_re =
        Regex::new(r#"classList\.(add|remove|toggle|contains)\s*\(\s*"([^"]*)"\s*\)"#).unwrap();
    let result = classlist_double_re.replace_all(&result, |caps: &regex::Captures| {
        let method = &caps[1];
        let classes_str = &caps[2];
        let replaced: Vec<_> = classes_str
            .split_whitespace()
            .map(|class| class_map.get(class).map(|s| s.as_str()).unwrap_or(class))
            .collect();
        format!("classList.{}(\"{}\")", method, replaced.join(" "))
    });

    // 5. Replace className = 'class1 class2' assignments - single quotes
    let classname_single_re = Regex::new(r#"\.className\s*=\s*'([^']*)'"#).unwrap();
    let result = classname_single_re.replace_all(&result, |caps: &regex::Captures| {
        let classes_str = &caps[1];
        let replaced: Vec<_> = classes_str
            .split_whitespace()
            .map(|class| class_map.get(class).map(|s| s.as_str()).unwrap_or(class))
            .collect();
        format!(".className = '{}'", replaced.join(" "))
    });

    // 6. Replace className = "class1 class2" assignments - double quotes
    let classname_double_re = Regex::new(r#"\.className\s*=\s*"([^"]*)""#).unwrap();
    let result = classname_double_re.replace_all(&result, |caps: &regex::Captures| {
        let classes_str = &caps[1];
        let replaced: Vec<_> = classes_str
            .split_whitespace()
            .map(|class| class_map.get(class).map(|s| s.as_str()).unwrap_or(class))
            .collect();
        format!(".className = \"{}\"", replaced.join(" "))
    });

    // 7. Replace class="className" in HTML template strings (double quotes)
    let class_attr_double_re = Regex::new(r#"class="([^"]*)""#).unwrap();
    let result = class_attr_double_re.replace_all(&result, |caps: &regex::Captures| {
        let classes_str = &caps[1];
        let replaced: Vec<_> = classes_str
            .split_whitespace()
            .map(|class| class_map.get(class).map(|s| s.as_str()).unwrap_or(class))
            .collect();
        format!("class=\"{}\"", replaced.join(" "))
    });

    // 8. Replace class='className' in HTML template strings (single quotes)
    let class_attr_single_re = Regex::new(r#"class='([^']*)'"#).unwrap();
    let result = class_attr_single_re.replace_all(&result, |caps: &regex::Captures| {
        let classes_str = &caps[1];
        let replaced: Vec<_> = classes_str
            .split_whitespace()
            .map(|class| class_map.get(class).map(|s| s.as_str()).unwrap_or(class))
            .collect();
        format!("class='{}'", replaced.join(" "))
    });

    // 9. Replace class names in ternary expression results (? 'className' or : 'className')
    // This handles patterns like: isExpanded ? 'copyButtonExpanded' : 'copyButton'
    let ternary_single_re = Regex::new(r#"([?:])\s*'([a-zA-Z_][a-zA-Z0-9_-]*)'"#).unwrap();
    let result = ternary_single_re.replace_all(&result, |caps: &regex::Captures| {
        let op = &caps[1];
        let class_name = &caps[2];
        if let Some(minified) = class_map.get(class_name) {
            format!("{} '{}'", op, minified)
        } else {
            caps[0].to_string()
        }
    });

    // 10. Replace class names in ternary expression results (double quotes)
    let ternary_double_re = Regex::new(r#"([?:])\s*"([a-zA-Z_][a-zA-Z0-9_-]*)""#).unwrap();
    let result = ternary_double_re.replace_all(&result, |caps: &regex::Captures| {
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
    let style_re = Regex::new(r"(?s)<style[^>]*>(.*?)</style>").unwrap();

    style_re
        .replace_all(html, |caps: &regex::Captures| {
            let style_content = &caps[1];
            let replaced = replace_classes_in_css(style_content, class_map);
            format!("<style>{}</style>", replaced)
        })
        .to_string()
}

/// Replace class names in inline <script> tags within HTML
fn replace_classes_in_inline_scripts(html: &str, class_map: &HashMap<String, String>) -> String {
    let script_re = Regex::new(r"(?s)<script[^>]*>(.*?)</script>").unwrap();

    script_re
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
