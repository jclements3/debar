use crate::decompose::{self, LetterBreakdown};

/// Connecting words used to join letter phrases in a narrative.
const CONNECTORS: &[&str] = &[
    "of the",
    "with the",
    "upon the",
    "through the",
    "over the",
    "inside the",
    "toward the",
    "bringing forth the",
];

/// Extract the first meaning keyword from a comma-separated meaning string.
/// e.g. "house, inside, family" -> "house"
fn primary_meaning(meaning: &str) -> &str {
    meaning.split(',').next().unwrap_or(meaning).trim()
}

/// Generate a pictorial memory narrative from a sequence of letter breakdowns.
///
/// For each letter, creates a phrase like "the [meaning keyword] ([Name])"
/// and joins them with rotating connecting words, ending with a summary dash.
///
/// This produces a template narrative -- not as poetic as hand-written ones,
/// but gives a structural starting point that mirrors the pattern in
/// narratives.json.
pub fn generate_narrative(letters: &[LetterBreakdown]) -> String {
    if letters.is_empty() {
        return String::from("(empty word -- no letters to narrate)");
    }

    if letters.len() == 1 {
        let m = primary_meaning(&letters[0].m);
        let n = &letters[0].n;
        return format!(
            "The {m} ({n}) -- a single letter standing alone, the essence distilled to one sign."
        );
    }

    // Build the chain of letter phrases.
    let mut phrases: Vec<String> = Vec::with_capacity(letters.len());

    for (i, letter) in letters.iter().enumerate() {
        let m = primary_meaning(&letter.m);
        let n = &letter.n;

        if i == 0 {
            // First letter: "The [meaning] ([Name])"
            phrases.push(format!("The {m} ({n})"));
        } else {
            // Subsequent letters: "[connector] [meaning] ([Name])"
            let connector = CONNECTORS[(i - 1) % CONNECTORS.len()];
            phrases.push(format!("{connector} {m} ({n})"));
        }
    }

    // Join the phrases and append a summary line.
    let chain = phrases.join(" ");

    // Build a summary from first and last letter meanings.
    let first_m = primary_meaning(&letters[0].m);
    let last_m = primary_meaning(&letters.last().unwrap().m);

    format!(
        "{chain} -- {first_m} joined to {last_m}, a picture traced in ancient signs."
    )
}

/// Decompose a Hebrew word and generate a template narrative for it.
pub fn generate_narrative_for_word(word: &str) -> String {
    let breakdown = decompose::decompose(word);
    generate_narrative(&breakdown.letters)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::decompose::LetterBreakdown;

    /// Helper to build a LetterBreakdown for testing.
    fn lb(name: &str, meaning: &str) -> LetterBreakdown {
        LetterBreakdown {
            p: String::new(),
            ph: String::new(),
            h: String::new(),
            n: name.to_string(),
            m: meaning.to_string(),
        }
    }

    #[test]
    fn test_empty_letters() {
        let result = generate_narrative(&[]);
        assert!(result.contains("empty word"));
    }

    #[test]
    fn test_single_letter() {
        let letters = vec![lb("Aleph", "strength, leader, first")];
        let result = generate_narrative(&letters);
        assert!(result.contains("strength"));
        assert!(result.contains("Aleph"));
        assert!(result.contains("single letter"));
    }

    #[test]
    fn test_two_letters() {
        let letters = vec![
            lb("Aleph", "strength, leader, first"),
            lb("Tav", "mark, covenant, sign"),
        ];
        let result = generate_narrative(&letters);
        assert!(result.contains("The strength (Aleph)"));
        assert!(result.contains("mark (Tav)"));
        assert!(result.contains("--"));
    }

    #[test]
    fn test_bara_narrative() {
        // bara = Beth Resh Aleph
        let result = generate_narrative_for_word("\u{05D1}\u{05BC}\u{05B8}\u{05E8}\u{05B8}\u{05D0}");
        assert!(result.contains("house (Beth)"));
        assert!(result.contains("head (Resh)"));
        assert!(result.contains("strength (Aleph)"));
        assert!(result.contains("--"));
    }

    #[test]
    fn test_primary_meaning_extraction() {
        assert_eq!(primary_meaning("house, inside, family"), "house");
        assert_eq!(primary_meaning("strength"), "strength");
        assert_eq!(primary_meaning("ox goad, toward, teach"), "ox goad");
    }

    #[test]
    fn test_longer_word_uses_connectors() {
        // bereshit = Beth Resh Aleph Shin Yod Tav (6 letters)
        let result = generate_narrative_for_word(
            "\u{05D1}\u{05BC}\u{05B0}\u{05E8}\u{05B5}\u{05D0}\u{05E9}\u{05C1}\u{05B4}\u{05D9}\u{05EA}",
        );
        // Should contain multiple connector phrases
        assert!(result.contains("house (Beth)"));
        assert!(result.contains("mark (Tav)"));
        // The narrative should end with the summary pattern
        assert!(result.contains("house joined to mark"));
    }

    #[test]
    fn test_shalom_narrative() {
        // shalom = Shin Lamed Vav Mem (with final mem)
        let result =
            generate_narrative_for_word("\u{05E9}\u{05DC}\u{05D5}\u{05DD}");
        assert!(result.contains("teeth (Shin)"));
        assert!(result.contains("ox goad (Lamed)"));
        assert!(result.contains("tent peg (Vav)"));
        assert!(result.contains("water (Mem)"));
    }
}
