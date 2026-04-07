//! Translation prompts

pub const TRANSLATE_HTML_PROMPT: &str = r#"Translate HTML content from Vietnamese to English

IMPORTANT REQUIREMENTS:
- Keep ALL HTML tags, CSS classes, IDs, and structure exactly the same unless otherwise specified below.
- Only translate text content inside the tags. Do not translate code, attribute values, IDs, classes, or comments except where explicitly requested.
- Preserve all formatting, spacing, and indentation.
- Do not add, remove, or modify any HTML attributes.
- Maintain the exact same structure.

ADDITIONAL PROJECT-SPECIFIC REQUIREMENT:
- For every chart container ID used in the report (for example `fear-greed-gauge-container`, `btc-dominance-doughnut-container`, `price-line-chart-container`, `volume-bar-chart-container`), REPLACE the original ID with a translated-English variant by appending the suffix `-en` to the ID. That means the English fragment should contain chart containers with IDs suffixed by `-en` (for example `fear-greed-gauge-container-en`). 
- IMPORTANT: Remove the original chart container IDs (without `-en` suffix) from the English fragment. The English version should only contain the `-en` suffixed IDs for chart containers to avoid duplicate IDs in the document.

Content to translate (follow the rules above):

{content}"#;

pub const TRANSLATE_JS_PROMPT: &str = r#"Translate JavaScript content from Vietnamese to English

REQUIREMENTS:
- Translate only legend/label content to English (keep crypto terms like BTC, ETH, RSI unchanged)
- Add `_en` suffix to all function names
- Add `-en` suffix to all container IDs  
- Keep all other code structure and logic exactly the same

EXAMPLES:
- `initializeAllVisuals_report()` → `initializeAllVisuals_report_en()`
- `initializeFearGreedGauge_report()` → `initializeFearGreedGauge_report_en()`
- `fear-greed-gauge-container` → `fear-greed-gauge-container-en`
- `label: 'Cực kỳ sợ hãi'` → `label: 'Extreme Fear'`
- `label: 'Tham lam'` → `label: 'Greed'`

Content to translate:

{js_content}"#;
