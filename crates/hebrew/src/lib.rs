pub mod decompose;
pub mod hanzi;
pub mod letters;
pub mod narrative;
pub mod oracle_bone;
pub mod radicals;
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

/// Decompose a Chinese character into its semantic components.
/// Takes a single character string, returns JSON CharBreakdown.
#[wasm_bindgen]
pub fn decompose_hanzi(ch: &str) -> String {
    let c = ch.chars().next().unwrap_or('\0');
    match hanzi::decompose_char(c) {
        Some(breakdown) => serde_json::to_string(&breakdown).unwrap_or_default(),
        None => String::from("null"),
    }
}

/// Generate a narrative memory story for a Chinese character.
/// Takes a single character string, returns a mnemonic narrative.
#[wasm_bindgen]
pub fn hanzi_narrative(ch: &str) -> String {
    let c = ch.chars().next().unwrap_or('\0');
    if let Some(breakdown) = hanzi::decompose_char(c) {
        return hanzi::char_narrative(&breakdown);
    }
    // Fall back to Oracle Bone description if available
    if let Some(ob) = oracle_bone::lookup_oracle_bone(c) {
        return format!("{} — {}", ob.description, ob.evolution);
    }
    String::new()
}

/// Look up Oracle Bone Script information for a Chinese character.
/// Takes a single character string, returns JSON OracleBoneInfo or "null".
#[wasm_bindgen]
pub fn get_oracle_bone(ch: &str) -> String {
    let c = ch.chars().next().unwrap_or('\0');
    match oracle_bone::lookup_oracle_bone(c) {
        Some(info) => serde_json::to_string(info).unwrap_or_else(|_| String::from("null")),
        None => String::from("null"),
    }
}
