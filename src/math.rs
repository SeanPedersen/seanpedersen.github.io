//! LaTeX -> MathML rendering.
//!
//! `latex2mathml` implements a subset of LaTeX: it knows `\geq` but not the
//! KaTeX/AMS alias `\ge`, `\langle` but not `\lvert`. Unknown commands are not
//! reported as errors, they are silently embedded into the MathML as
//! `<mtext>[PARSE ERROR: ...]</mtext>`. So posts are normalized to the
//! supported spelling before conversion, and any remaining parse error is
//! surfaced to the caller as `None` for a raw-LaTeX fallback.

use latex2mathml::{latex_to_mathml, DisplayStyle};
use once_cell::sync::Lazy;
use regex::Regex;

/// Marker `latex2mathml` embeds instead of failing on unknown constructs.
const PARSE_ERROR_MARKER: &str = "[PARSE ERROR:";

/// Commands common in KaTeX-flavored markdown, mapped to their `latex2mathml`
/// equivalent. An empty replacement drops a purely presentational command.
const COMMAND_ALIASES: &[(&str, &str)] = &[
    // Relations
    ("ge", r"\geq"),
    ("le", r"\leq"),
    ("neg", r"\lnot"),
    ("isin", r"\in"),
    // Delimiters
    ("lvert", "|"),
    ("rvert", "|"),
    ("vert", "|"),
    ("lVert", "‖"),
    ("rVert", "‖"),
    ("Vert", "‖"),
    ("lbrace", r"\{"),
    ("rbrace", r"\}"),
    ("lbrack", "["),
    ("rbrack", "]"),
    ("lang", r"\langle"),
    ("rang", r"\rangle"),
    // Sizing hints without a MathML counterpart (browsers stretch on their own)
    ("big", ""),
    ("Big", ""),
    ("bigg", ""),
    ("Bigg", ""),
    ("limits", ""),
    ("nolimits", ""),
    ("displaystyle", ""),
    ("textstyle", ""),
    ("scriptstyle", ""),
    // Fractions and stacking
    ("dfrac", r"\frac"),
    ("tfrac", r"\frac"),
    ("cfrac", r"\frac"),
    ("stackrel", r"\overset"),
    // Symbols and styles
    ("mathcal", r"\mathscr"),
    ("pmb", r"\boldsymbol"),
    ("varnothing", r"\emptyset"),
    ("empty", r"\emptyset"),
    ("infin", r"\infty"),
    ("ast", "*"),
    ("dots", r"\ldots"),
    ("bmod", r"\mathrm{mod}"),
];

/// HTML5 parsers don't honor XML self-closing syntax (`/>`) for non-void
/// elements. latex2mathml emits MathML as XML (e.g. `<mspace width="1em"/>`),
/// which the minifier strips to `<mspace width=1em>`, leaving the element
/// unclosed. This converts every self-closing tag to an explicit open+close pair.
fn expand_self_closing(mathml: &str) -> String {
    static SELF_CLOSING: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"<([a-zA-Z][a-zA-Z0-9]*)([^>]*?)/>").unwrap());

    SELF_CLOSING
        .replace_all(mathml, |caps: &regex::Captures| {
            format!("<{}{}></{}>", &caps[1], &caps[2], &caps[1])
        })
        .to_string()
}

/// Reads the command name following a backslash, using the same rule as the
/// latex2mathml lexer: always one character, extended greedily while ASCII
/// alphabetic. Returns the name and the byte offset just past it.
fn read_command(latex: &str, backslash: usize) -> (&str, usize) {
    let rest = &latex[backslash + 1..];
    let Some(first) = rest.chars().next() else {
        return ("", backslash + 1);
    };

    if !first.is_ascii_alphabetic() {
        let end = backslash + 1 + first.len_utf8();
        return (&latex[backslash + 1..end], end);
    }

    let name_len = rest
        .find(|c: char| !c.is_ascii_alphabetic())
        .unwrap_or(rest.len());
    let end = backslash + 1 + name_len;
    (&latex[backslash + 1..end], end)
}

/// Rewrites unsupported command aliases into their supported spelling.
fn normalize(latex: &str) -> String {
    let mut out = String::with_capacity(latex.len());
    let mut cursor = 0;

    while let Some(offset) = latex[cursor..].find('\\') {
        let backslash = cursor + offset;
        out.push_str(&latex[cursor..backslash]);

        let (command, end) = read_command(latex, backslash);
        match COMMAND_ALIASES.iter().find(|(alias, _)| *alias == command) {
            Some((_, replacement)) => out.push_str(replacement),
            None => out.push_str(&latex[backslash..end]),
        }
        cursor = end;
    }

    out.push_str(&latex[cursor..]);
    out
}

/// Converts a LaTeX fragment to MathML, or `None` if it cannot be rendered.
pub fn to_mathml(latex: &str, style: DisplayStyle) -> Option<String> {
    let mathml = latex_to_mathml(&normalize(latex), style).ok()?;
    if mathml.contains(PARSE_ERROR_MARKER) {
        return None;
    }
    Some(expand_self_closing(&mathml))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn render(latex: &str) -> String {
        to_mathml(latex, DisplayStyle::Inline).expect("should render")
    }

    #[test]
    fn renders_schroedinger_equation() {
        let mathml = render(
            r"i\hbar\frac{\partial}{\partial t}\lvert\psi(t)\rangle=\hat{H}\lvert\psi(t)\rangle",
        );
        assert!(!mathml.contains(PARSE_ERROR_MARKER));
        assert!(mathml.contains("&rang;"));
        assert!(mathml.contains(">|<"));
    }

    #[test]
    fn renders_uncertainty_principle() {
        let mathml = render(r"\Delta x \cdot \Delta p \ge \frac{\hbar}{2}");
        assert!(mathml.contains('≥'));
    }

    #[test]
    fn leaves_supported_commands_untouched() {
        assert_eq!(normalize(r"\geq \alpha_1"), r"\geq \alpha_1");
    }

    #[test]
    fn matches_whole_command_names_only() {
        assert_eq!(normalize(r"\gets\bigl\big\lvert"), r"\gets\bigl|");
    }

    #[test]
    fn keeps_non_alphabetic_commands() {
        assert_eq!(normalize(r"a\,b\\c"), r"a\,b\\c");
    }

    #[test]
    fn reports_unrenderable_latex() {
        assert!(to_mathml(r"\thiscommanddoesnotexist", DisplayStyle::Inline).is_none());
    }

    #[test]
    fn expands_self_closing_tags() {
        assert_eq!(
            expand_self_closing(r#"<mspace width="1em"/>"#),
            r#"<mspace width="1em"></mspace>"#
        );
    }
}
