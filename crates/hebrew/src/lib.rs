pub mod decompose;
pub mod letters;
pub mod narrative;
pub mod roots;

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

/// Generate a pictorial memory narrative for a Hebrew word.
/// Takes a Hebrew word (with or without niqqud), decomposes it,
/// and returns a template narrative string.
#[wasm_bindgen]
pub fn generate_narrative_wasm(word: &str) -> String {
    narrative::generate_narrative_for_word(word)
}

/// Extract the probable Hebrew root from a word, stripping prefixes and suffixes.
/// Returns JSON: { original, prefixes, root, suffixes }
#[wasm_bindgen]
pub fn extract_root_json(word: &str) -> String {
    let result = roots::extract_root(word);
    serde_json::to_string(&result).unwrap_or_default()
}
