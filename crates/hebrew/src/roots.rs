use crate::decompose;
use serde::Serialize;

// Hebrew consonant constants
const BET: char = '\u{05D1}';     // ב
const HE: char = '\u{05D4}';      // ה
const VAV: char = '\u{05D5}';     // ו
const YOD: char = '\u{05D9}';     // י
const KAPH: char = '\u{05DB}';    // כ
const LAMED: char = '\u{05DC}';   // ל
const MEM: char = '\u{05DE}';     // מ
const NUN: char = '\u{05E0}';     // נ
const SHIN: char = '\u{05E9}';    // ש
const TAV: char = '\u{05EA}';     // ת

const FINAL_MEM: char = '\u{05DD}';  // ם
const FINAL_NUN: char = '\u{05DF}';  // ן
const FINAL_KAPH: char = '\u{05DA}'; // ך

/// Result of analyzing a Hebrew word for its root.
#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct RootAnalysis {
    /// Original consonants (niqqud stripped)
    pub original: String,
    /// Prefixes that were identified and removed
    pub prefixes: Vec<String>,
    /// The probable root consonants
    pub root: String,
    /// Suffixes that were identified and removed
    pub suffixes: Vec<String>,
}

/// Normalize final-form letters to their base form.
fn normalize_finals(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            FINAL_MEM => MEM,
            FINAL_NUN => NUN,
            FINAL_KAPH => KAPH,
            '\u{05E3}' => '\u{05E4}', // final pe -> pe
            '\u{05E5}' => '\u{05E6}', // final tsade -> tsade
            other => other,
        })
        .collect()
}

/// Extract consonants only from a Hebrew string (strip niqqud, keep only
/// Hebrew consonant code points U+05D0..U+05EA and final forms U+05DA..U+05DF).
fn extract_consonants(word: &str) -> String {
    let stripped = decompose::strip_niqqud(word);
    stripped
        .chars()
        .filter(|&c| {
            let cp = c as u32;
            (0x05D0..=0x05EA).contains(&cp)
        })
        .collect()
}

/// Known single-letter prefixes (order matters: try longer combos first via
/// the two-letter list, then fall back to single).
const SINGLE_PREFIXES: &[char] = &[BET, KAPH, LAMED, MEM, HE, VAV, SHIN];

/// Known two-letter prefix combinations.
const DOUBLE_PREFIXES: &[[char; 2]] = &[
    [VAV, BET],   // וב
    [VAV, KAPH],  // וכ
    [VAV, LAMED], // ול
    [VAV, MEM],   // ומ
    [VAV, HE],    // וה
    [MEM, HE],    // מה
    [SHIN, HE],   // שה
    [BET, HE],    // בה
    [KAPH, SHIN], // כש
    [LAMED, KAPH],// לכ
    [MEM, SHIN],  // מש
    [VAV, SHIN],  // וש
];

/// Known three-letter prefix combinations.
const TRIPLE_PREFIXES: &[[char; 3]] = &[
    [VAV, BET, HE],    // ובה
    [VAV, KAPH, SHIN], // וכש
    [VAV, MEM, HE],    // ומה
    [VAV, LAMED, HE],  // ולה
];

/// Suffix definitions: (suffix chars, display string).
/// Ordered longest first so we match greedily.
struct SuffixDef {
    chars: &'static [char],
}

const SUFFIXES: &[SuffixDef] = &[
    // 3-letter suffixes
    SuffixDef { chars: &[TAV, YOD, MEM] },    // תים (not standard but safety)
    // 2-letter suffixes
    SuffixDef { chars: &[YOD, MEM] },          // ים masculine plural
    SuffixDef { chars: &[VAV, TAV] },          // ות feminine plural
    SuffixDef { chars: &[NUN, VAV] },          // נו our
    SuffixDef { chars: &[KAPH, MEM] },         // כם your pl masc
    SuffixDef { chars: &[KAPH, NUN] },         // כן your pl fem
    SuffixDef { chars: &[HE, MEM] },           // הם their masc
    SuffixDef { chars: &[HE, NUN] },           // הן their fem
    SuffixDef { chars: &[YOD, TAV] },          // ית (e.g. bereshit suffix)
    // 1-letter suffixes
    SuffixDef { chars: &[HE] },               // ה feminine singular / directional
    SuffixDef { chars: &[YOD] },              // י my / 1st person
    SuffixDef { chars: &[KAPH] },             // ך your 2nd masc
    SuffixDef { chars: &[VAV] },              // ו his 3rd masc
    SuffixDef { chars: &[TAV] },              // ת feminine / 2fs verb
];

/// Try to strip a suffix from the end of the consonant string.
/// Returns (remaining, suffix_string) if a suffix was stripped, leaving
/// at least 2 consonants in the remaining.
fn try_strip_suffix(consonants: &[char]) -> Option<(Vec<char>, String)> {
    for suf in SUFFIXES {
        let slen = suf.chars.len();
        if consonants.len() > slen && consonants.len() - slen >= 2 {
            let tail = &consonants[consonants.len() - slen..];
            if tail == suf.chars {
                let remaining = consonants[..consonants.len() - slen].to_vec();
                let suffix_str: String = suf.chars.iter().collect();
                return Some((remaining, suffix_str));
            }
        }
    }
    None
}

/// Try to strip prefixes from the beginning of the consonant string.
/// Returns (remaining, list_of_prefix_strings).
/// We require at least 2 consonants to remain after stripping.
fn try_strip_prefixes(consonants: &[char]) -> (Vec<char>, Vec<String>) {
    let mut prefixes_found: Vec<String> = Vec::new();

    // Try triple prefixes first
    if consonants.len() >= 5 {
        for triple in TRIPLE_PREFIXES {
            if consonants[0] == triple[0] && consonants[1] == triple[1] && consonants[2] == triple[2] {
                let prefix_str: String = triple.iter().collect();
                prefixes_found.push(prefix_str);
                return (consonants[3..].to_vec(), prefixes_found);
            }
        }
    }

    // Try double prefixes
    if consonants.len() >= 4 {
        for double in DOUBLE_PREFIXES {
            if consonants[0] == double[0] && consonants[1] == double[1] {
                let prefix_str: String = double.iter().collect();
                prefixes_found.push(prefix_str);
                return (consonants[2..].to_vec(), prefixes_found);
            }
        }
    }

    // Try single prefixes
    if consonants.len() >= 3 {
        for &prefix in SINGLE_PREFIXES {
            if consonants[0] == prefix {
                let prefix_str: String = [prefix].iter().collect();
                prefixes_found.push(prefix_str);
                return (consonants[1..].to_vec(), prefixes_found);
            }
        }
    }

    (consonants.to_vec(), prefixes_found)
}

/// Extract the probable Hebrew root from a word.
///
/// Steps:
/// 1. Strip niqqud and extract consonants only
/// 2. Normalize final forms
/// 3. Try to identify and remove known prefixes
/// 4. Try to identify and remove known suffixes
/// 5. Return the remaining consonants as the probable root
pub fn extract_root(word: &str) -> RootAnalysis {
    let consonants_raw = extract_consonants(word);
    let consonants_normalized = normalize_finals(&consonants_raw);
    let chars: Vec<char> = consonants_normalized.chars().collect();

    if chars.len() <= 2 {
        // Too short to strip anything
        return RootAnalysis {
            original: consonants_normalized.clone(),
            prefixes: vec![],
            root: consonants_normalized,
            suffixes: vec![],
        };
    }

    // Strip prefixes first
    let (after_prefix, prefixes) = try_strip_prefixes(&chars);

    // Strip suffixes from what remains
    let (root_chars, suffixes) = if let Some((remaining, suffix_str)) = try_strip_suffix(&after_prefix) {
        (remaining, vec![suffix_str])
    } else {
        (after_prefix, vec![])
    };

    let root: String = root_chars.iter().collect();

    RootAnalysis {
        original: consonants_normalized,
        prefixes,
        root,
        suffixes,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bereshit() {
        // בְּרֵאשִׁית -> prefix ב, root ראש, suffix ית
        let result = extract_root("\u{05D1}\u{05BC}\u{05B0}\u{05E8}\u{05B5}\u{05D0}\u{05E9}\u{05C1}\u{05B4}\u{05D9}\u{05EA}");
        assert_eq!(result.prefixes, vec!["\u{05D1}"]);          // ב
        assert_eq!(result.root, "\u{05E8}\u{05D0}\u{05E9}");   // ראש
        assert_eq!(result.suffixes, vec!["\u{05D9}\u{05EA}"]);  // ית
    }

    #[test]
    fn test_hashamayim() {
        // הַשָּׁמַיִם -> prefix ה, root שמ, suffix ים
        let result = extract_root("\u{05D4}\u{05B7}\u{05E9}\u{05C1}\u{05BC}\u{05B8}\u{05DE}\u{05B7}\u{05D9}\u{05B4}\u{05DD}");
        assert_eq!(result.prefixes, vec!["\u{05D4}"]);          // ה
        assert_eq!(result.root, "\u{05E9}\u{05DE}");            // שמ
        assert_eq!(result.suffixes, vec!["\u{05D9}\u{05DE}"]);  // ים
    }

    #[test]
    fn test_vehaaretz() {
        // וְהָאָרֶץ -> prefix וה, root ארץ
        let result = extract_root("\u{05D5}\u{05B0}\u{05D4}\u{05B8}\u{05D0}\u{05B8}\u{05E8}\u{05B6}\u{05E5}");
        assert_eq!(result.prefixes, vec!["\u{05D5}\u{05D4}"]);  // וה
        assert_eq!(result.root, "\u{05D0}\u{05E8}\u{05E6}");    // ארץ (final tsade normalized)
        assert!(result.suffixes.is_empty());
    }

    #[test]
    fn test_elohim() {
        // אֱלֹהִים -> root אלה, suffix ים
        let result = extract_root("\u{05D0}\u{05B1}\u{05DC}\u{05B9}\u{05D4}\u{05B4}\u{05D9}\u{05DD}");
        assert_eq!(result.root, "\u{05D0}\u{05DC}\u{05D4}");    // אלה
        assert_eq!(result.suffixes, vec!["\u{05D9}\u{05DE}"]);   // ים (final mem normalized)
        assert!(result.prefixes.is_empty());
    }

    #[test]
    fn test_merachefet() {
        // מְרַחֶפֶת -> prefix מ, root רחפ, suffix ת
        let result = extract_root("\u{05DE}\u{05B0}\u{05E8}\u{05B7}\u{05D7}\u{05B6}\u{05E4}\u{05B6}\u{05EA}");
        assert_eq!(result.prefixes, vec!["\u{05DE}"]);           // מ
        assert_eq!(result.root, "\u{05E8}\u{05D7}\u{05E4}");    // רחפ
        assert_eq!(result.suffixes, vec!["\u{05EA}"]);           // ת
    }

    #[test]
    fn test_short_word() {
        // אל (el - God) - only 2 consonants, no stripping
        let result = extract_root("\u{05D0}\u{05DC}");
        assert_eq!(result.root, "\u{05D0}\u{05DC}");
        assert!(result.prefixes.is_empty());
        assert!(result.suffixes.is_empty());
    }

    #[test]
    fn test_normalize_finals() {
        // Word ending with final mem should normalize
        let normalized = normalize_finals("\u{05E9}\u{05DC}\u{05D5}\u{05DD}"); // שלום
        assert_eq!(normalized, "\u{05E9}\u{05DC}\u{05D5}\u{05DE}"); // שלומ
    }

    #[test]
    fn test_no_prefix_no_suffix() {
        // ברא (bara - created) - 3 consonants, bet could be prefix but would leave only 2
        // Actually bet IS a known prefix but we need at least 2 remaining after prefix
        // + suffix stripping. With 3 letters, stripping bet leaves רא (2 chars) which is valid.
        // But since רא is only 2 and no suffix, that's our root.
        let result = extract_root("\u{05D1}\u{05E8}\u{05D0}");
        // bet is a prefix, leaving רא
        assert_eq!(result.prefixes, vec!["\u{05D1}"]);
        assert_eq!(result.root, "\u{05E8}\u{05D0}");
    }

    #[test]
    fn test_masculine_plural_suffix() {
        // סוסים (susim - horses) -> root סוס, suffix ים
        let result = extract_root("\u{05E1}\u{05D5}\u{05E1}\u{05D9}\u{05DD}");
        assert!(result.prefixes.is_empty());
        assert_eq!(result.root, "\u{05E1}\u{05D5}\u{05E1}");    // סוס
        assert_eq!(result.suffixes, vec!["\u{05D9}\u{05DE}"]);   // ים
    }

    #[test]
    fn test_feminine_plural_suffix() {
        // תורות (torot - Torahs) -> root תור, suffix ות
        let result = extract_root("\u{05EA}\u{05D5}\u{05E8}\u{05D5}\u{05EA}");
        assert!(result.prefixes.is_empty());
        assert_eq!(result.root, "\u{05EA}\u{05D5}\u{05E8}");    // תור
        assert_eq!(result.suffixes, vec!["\u{05D5}\u{05EA}"]);   // ות
    }

    #[test]
    fn test_double_prefix() {
        // ובשמים (uvashamayim - and in the heavens)
        // וב prefix, שמ root, ים suffix
        let result = extract_root("\u{05D5}\u{05D1}\u{05E9}\u{05DE}\u{05D9}\u{05DD}");
        assert_eq!(result.prefixes, vec!["\u{05D5}\u{05D1}"]);  // וב
        assert_eq!(result.root, "\u{05E9}\u{05DE}");            // שמ
        assert_eq!(result.suffixes, vec!["\u{05D9}\u{05DE}"]);  // ים
    }

    #[test]
    fn test_extract_consonants_strips_niqqud() {
        let cons = extract_consonants("\u{05D1}\u{05BC}\u{05B0}\u{05E8}\u{05B5}\u{05D0}\u{05E9}\u{05C1}\u{05B4}\u{05D9}\u{05EA}");
        assert_eq!(cons, "\u{05D1}\u{05E8}\u{05D0}\u{05E9}\u{05D9}\u{05EA}"); // בראשית
    }
}
