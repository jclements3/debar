use crate::letters;
use serde::Serialize;

/// A single letter breakdown in a word.
#[derive(Debug, Serialize, Clone)]
pub struct LetterBreakdown {
    /// Emoji pictograph
    pub p: String,
    /// Phoenician character
    pub ph: String,
    /// Hebrew consonant (base form)
    pub h: String,
    /// Letter name
    pub n: String,
    /// Core meaning
    pub m: String,
}

/// Result of decomposing a Hebrew word.
#[derive(Debug, Serialize, Clone)]
pub struct WordBreakdown {
    /// Original word with niqqud
    pub heb: String,
    /// Consonants only (niqqud stripped)
    pub cons: String,
    /// Letter-by-letter breakdown
    pub letters: Vec<LetterBreakdown>,
}

/// Unicode ranges for Hebrew niqqud (vowel points) and cantillation marks.
fn is_niqqud(ch: char) -> bool {
    let cp = ch as u32;
    // Hebrew points: U+05B0..U+05BD, U+05BF, U+05C1..U+05C2, U+05C4..U+05C5, U+05C7
    // Cantillation: U+0591..U+05AF
    (0x0591..=0x05AF).contains(&cp)
        || (0x05B0..=0x05BD).contains(&cp)
        || cp == 0x05BF
        || (0x05C1..=0x05C2).contains(&cp)
        || (0x05C4..=0x05C5).contains(&cp)
        || cp == 0x05C7
}

/// Is this a Hebrew consonant (aleph through tav, including final forms)?
fn is_hebrew_consonant(ch: char) -> bool {
    let cp = ch as u32;
    (0x05D0..=0x05EA).contains(&cp)
}

/// Strip niqqud from a Hebrew string, returning only consonants.
pub fn strip_niqqud(s: &str) -> String {
    s.chars().filter(|&c| !is_niqqud(c)).collect()
}

/// Decompose a Hebrew word into its letter-by-letter pictograph breakdown.
pub fn decompose(word: &str) -> WordBreakdown {
    let consonants: String = word.chars().filter(|&c| is_hebrew_consonant(c)).collect();

    let letter_breakdowns: Vec<LetterBreakdown> = consonants
        .chars()
        .filter_map(|ch| {
            letters::lookup(ch).map(|info| LetterBreakdown {
                p: info.pictograph.to_string(),
                ph: info.phoenician.to_string(),
                h: info.hebrew.to_string(),
                n: info.name.to_string(),
                m: info.meaning.to_string(),
            })
        })
        .collect();

    WordBreakdown {
        heb: word.to_string(),
        cons: consonants,
        letters: letter_breakdowns,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_niqqud() {
        // bereshit with niqqud -> bare consonants
        assert_eq!(strip_niqqud("\u{05D1}\u{05BC}\u{05B0}\u{05E8}\u{05B5}\u{05D0}\u{05E9}\u{05C1}\u{05B4}\u{05D9}\u{05EA}"), "\u{05D1}\u{05E8}\u{05D0}\u{05E9}\u{05D9}\u{05EA}");
    }

    #[test]
    fn test_decompose_bara() {
        // bara = בָּרָא
        let result = decompose("\u{05D1}\u{05BC}\u{05B8}\u{05E8}\u{05B8}\u{05D0}");
        assert_eq!(result.cons, "\u{05D1}\u{05E8}\u{05D0}");
        assert_eq!(result.letters.len(), 3);
        assert_eq!(result.letters[0].n, "Beth");
        assert_eq!(result.letters[1].n, "Resh");
        assert_eq!(result.letters[2].n, "Aleph");
    }

    #[test]
    fn test_final_form_mem() {
        // shalom = שלום (final mem)
        let result = decompose("\u{05E9}\u{05DC}\u{05D5}\u{05DD}");
        assert_eq!(result.letters.len(), 4);
        assert_eq!(result.letters[3].n, "Mem");
        assert_eq!(result.letters[3].h, "\u{05DE}"); // normalized to base form
    }
}
