//! `MathJax`, LaTeX, and technical indicator symbol processing.

use regex::Regex;
use std::sync::OnceLock;

static MATHJAX_REPLACEMENTS: OnceLock<Vec<(Regex, &'static str)>> = OnceLock::new();

#[allow(clippy::expect_used)]
fn get_mathjax_replacements() -> &'static Vec<(Regex, &'static str)> {
    MATHJAX_REPLACEMENTS.get_or_init(|| {
        vec![
            // Arrow combinations (Trend indicators)
            (
                Regex::new(r"\$\s*\\uparrow\s*\\rightarrow\s*\$").expect("Invalid regex: up-right"),
                r#"<i class="fas fa-arrow-up mx-1 text-green-500"></i><i class="fas fa-arrow-right mx-1"></i>"#,
            ),
            (
                Regex::new(r"\$\s*\\downarrow\s*\\rightarrow\s*\$").expect("Invalid regex: down-right"),
                r#"<i class="fas fa-arrow-down mx-1 text-red-500"></i><i class="fas fa-arrow-right mx-1"></i>"#,
            ),
            (
                Regex::new(r"\$\s*\\rightarrow\s*\\uparrow\s*\$").expect("Invalid regex: right-up"),
                r#"<i class="fas fa-arrow-right mx-1"></i><i class="fas fa-arrow-up mx-1 text-green-500"></i>"#,
            ),
            (
                Regex::new(r"\$\s*\\rightarrow\s*\\downarrow\s*\$").expect("Invalid regex: right-down"),
                r#"<i class="fas fa-arrow-right mx-1"></i><i class="fas fa-arrow-down mx-1 text-red-500"></i>"#,
            ),
            (
                Regex::new(r"\$\s*\\uparrow\s*\\uparrow\s*\$").expect("Invalid regex: double-up"),
                r#"<i class="fas fa-arrow-up mx-1 text-green-500"></i><i class="fas fa-arrow-up mx-1 text-green-500"></i>"#,
            ),
            (
                Regex::new(r"\$\s*\\downarrow\s*\\downarrow\s*\$").expect("Invalid regex: double-down"),
                r#"<i class="fas fa-arrow-down mx-1 text-red-500"></i><i class="fas fa-arrow-down mx-1 text-red-500"></i>"#,
            ),
            // Single Right arrow
            (
                Regex::new(r"\$\s*\\rightarrow\s*\$").expect("Invalid regex: rightarrow"),
                r#"<i class="fas fa-arrow-right mx-1"></i>"#,
            ),
            // Single Left arrow
            (
                Regex::new(r"\$\s*\\leftarrow\s*\$").expect("Invalid regex: leftarrow"),
                r#"<i class="fas fa-arrow-left mx-1"></i>"#,
            ),
            // Right double arrow
            (
                Regex::new(r"\$\s*\\Rightarrow\s*\$").expect("Invalid regex: Rightarrow"),
                r#"<i class="fas fa-arrow-right mx-1"></i>"#,
            ),
            // Left double arrow
            (
                Regex::new(r"\$\s*\\Leftarrow\s*\$").expect("Invalid regex: Leftarrow"),
                r#"<i class="fas fa-arrow-left mx-1"></i>"#,
            ),
            // Up arrow (Bullish)
            (
                Regex::new(r"\$\s*\\uparrow\s*\$").expect("Invalid regex: uparrow"),
                r#"<i class="fas fa-arrow-up mx-1 text-green-500"></i>"#,
            ),
            // Down arrow (Bearish)
            (
                Regex::new(r"\$\s*\\downarrow\s*\$").expect("Invalid regex: downarrow"),
                r#"<i class="fas fa-arrow-down mx-1 text-red-500"></i>"#,
            ),
            // Plus-minus sign
            (
                Regex::new(r"\$\s*\\pm\s*\$").expect("Invalid regex: pm"),
                "±",
            ),
            // Greater than or equal to
            (
                Regex::new(r"\$\s*\\ge(q)?\s*\$").expect("Invalid regex: ge"),
                "≥",
            ),
            // Less than or equal to
            (
                Regex::new(r"\$\s*\\le(q)?\s*\$").expect("Invalid regex: le"),
                "≤",
            ),
            // Approximation
            (
                Regex::new(r"\$\s*\\approx\s*\$").expect("Invalid regex: approx"),
                "≈",
            ),
            // Ellipsis
            (
                Regex::new(r"\$\s*\\dots\s*\$").expect("Invalid regex: dots"),
                "…",
            ),
            // Inline text in MathJax
            (
                Regex::new(r"\$\s*\\text\{([^}]+)\}\s*\$").expect("Invalid regex: text"),
                "$1",
            ),
        ]
    })
}

/// Processes content to convert common symbols to MathJax/LaTeX equivalents.
///
/// This is used to replace backend-side technical symbols with LaTeX/MathJax or
/// `FontAwesome` icons for consistent rendering across platforms.
#[must_use]
pub fn process_mathjax(content: &str) -> String {
    let replacements = get_mathjax_replacements();
    let mut result = std::borrow::Cow::Borrowed(content);

    for (re, rep) in replacements {
        if re.is_match(&result) {
            result = std::borrow::Cow::Owned(re.replace_all(&result, *rep).into_owned());
        }
    }

    result.into_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_mathjax_symbols() {
        let input = "BTC $ \\approx $ $100k, trend: $ \\uparrow $";
        let output = process_mathjax(input);
        assert!(output.contains('≈'));
        assert!(output.contains(r#"<i class="fas fa-arrow-up mx-1 text-green-500"></i>"#));
    }
}
