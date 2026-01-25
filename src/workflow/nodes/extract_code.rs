//! Extract code node
//!
//! Extracts HTML, CSS, and JS from generated response.
//! Equivalent to `app/services/workflow_nodes/extract_code.py`

use crate::workflow::state::ReportState;
use regex::Regex;
use tracing::{error, info, warn};

/// Extracts code blocks from the generated interface response.
pub async fn extract_code(mut state: ReportState) -> Result<ReportState, anyhow::Error> {
    let session_id = &state.session_id.clone();
    info!("[{}] Step 6: Extract code", session_id);

    // Get the response content
    let content = match &state.report_content {
        Some(c) => c.clone(),
        None => {
            let error_msg = "No content to extract code from";
            error!("[{}] {}", session_id, error_msg);
            state.add_error(error_msg);
            state.success = false;
            return Ok(state);
        }
    };

    // Extract code blocks
    let (html, css, js) = extract_code_blocks(&content);

    // Validate extraction
    if html.is_none() {
        warn!("[{}] Failed to extract HTML content", session_id);
        state.add_error("Failed to extract HTML content");
        state.success = false;
        return Ok(state);
    }

    state.html_content = html;
    state.css_content = css;
    state.js_content = js;
    state.success = true;

    info!("[{}] Code extraction completed", session_id);
    Ok(state)
}

/// Extracts HTML, CSS, and JS code blocks from response text.
fn extract_code_blocks(text: &str) -> (Option<String>, Option<String>, Option<String>) {
    let html = extract_block(text, "html");
    let css = extract_block(text, "css");
    let js = extract_block(text, "javascript").or_else(|| extract_block(text, "js"));

    (html, css, js)
}

/// Extracts a specific code block by language.
fn extract_block(text: &str, language: &str) -> Option<String> {
    // Pattern: ```language ... ```
    let pattern = format!(r"```{}\s*\n([\s\S]*?)\n```", language);

    if let Ok(regex) = Regex::new(&pattern) {
        if let Some(captures) = regex.captures(text) {
            if let Some(code) = captures.get(1) {
                let code_str = code.as_str().trim();
                if !code_str.is_empty() {
                    return Some(code_str.to_string());
                }
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_html() {
        let content = r#"
Here is the code:
```html
<div>Hello</div>
```
"#;
        let (html, _, _) = extract_code_blocks(content);
        assert_eq!(html, Some("<div>Hello</div>".to_string()));
    }

    #[test]
    fn test_extract_all_blocks() {
        let content = r#"
```html
<html></html>
```

```css
body { color: red; }
```

```javascript
console.log("hello");
```
"#;
        let (html, css, js) = extract_code_blocks(content);
        assert!(html.is_some());
        assert!(css.is_some());
        assert!(js.is_some());
    }
}
