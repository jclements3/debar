pub mod decompose;
pub mod letters;

use wasm_bindgen::prelude::*;

/// Decompose a single Hebrew word into its pictograph letters.
/// Returns JSON: { heb, cons, letters: [{p, ph, h, n, m}] }
#[wasm_bindgen]
pub fn decompose_word(word: &str) -> String {
    let result = decompose::decompose(word);
    serde_json::to_string(&result).unwrap_or_default()
}

/// Decompose multiple words (space-separated or newline-separated).
/// Returns JSON array of WordBreakdown objects.
#[wasm_bindgen]
pub fn decompose_text(text: &str) -> String {
    let results: Vec<decompose::WordBreakdown> = text
        .split_whitespace()
        .map(|w| decompose::decompose(w))
        .collect();
    serde_json::to_string(&results).unwrap_or_default()
}

/// Strip niqqud (vowel points) from Hebrew text.
#[wasm_bindgen]
pub fn strip_niqqud(text: &str) -> String {
    decompose::strip_niqqud(text)
}
