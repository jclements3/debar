/// The 22 Hebrew letters with their pictograph data.
/// Each entry: (hebrew_char, final_form, pictograph, phoenician, name, meaning)
pub struct LetterInfo {
    pub hebrew: char,
    pub final_form: Option<char>,
    pub pictograph: &'static str,
    pub phoenician: char,
    pub name: &'static str,
    pub meaning: &'static str,
}

pub const LETTERS: &[LetterInfo] = &[
    LetterInfo { hebrew: '\u{05D0}', final_form: None,             pictograph: "\u{1F402}", phoenician: '\u{10900}', name: "Aleph",  meaning: "strength, leader, first" },
    LetterInfo { hebrew: '\u{05D1}', final_form: None,             pictograph: "\u{1F3E0}", phoenician: '\u{10901}', name: "Beth",   meaning: "house, inside, family" },
    LetterInfo { hebrew: '\u{05D2}', final_form: None,             pictograph: "\u{1F42A}", phoenician: '\u{10902}', name: "Gimel",  meaning: "camel, carry, lift" },
    LetterInfo { hebrew: '\u{05D3}', final_form: None,             pictograph: "\u{1F6AA}", phoenician: '\u{10903}', name: "Dalet",  meaning: "door, pathway, entry" },
    LetterInfo { hebrew: '\u{05D4}', final_form: None,             pictograph: "\u{1F450}", phoenician: '\u{10904}', name: "He",     meaning: "window, breath, behold" },
    LetterInfo { hebrew: '\u{05D5}', final_form: None,             pictograph: "\u{1FA9D}", phoenician: '\u{10905}', name: "Vav",    meaning: "tent peg, hook, connect" },
    LetterInfo { hebrew: '\u{05D6}', final_form: None,             pictograph: "\u{2694}\u{FE0F}", phoenician: '\u{10906}', name: "Zayin",  meaning: "weapon, cut, sustain" },
    LetterInfo { hebrew: '\u{05D7}', final_form: None,             pictograph: "\u{1F9F1}", phoenician: '\u{10907}', name: "Chet",   meaning: "fence, inner room, separate" },
    LetterInfo { hebrew: '\u{05D8}', final_form: None,             pictograph: "\u{1F40D}", phoenician: '\u{10908}', name: "Tet",    meaning: "snake, surround, contain" },
    LetterInfo { hebrew: '\u{05D9}', final_form: None,             pictograph: "\u{1F932}", phoenician: '\u{10909}', name: "Yod",    meaning: "hand, deed, work" },
    LetterInfo { hebrew: '\u{05DB}', final_form: Some('\u{05DA}'), pictograph: "\u{270B}",  phoenician: '\u{1090A}', name: "Kaph",   meaning: "open palm, bend, yield" },
    LetterInfo { hebrew: '\u{05DC}', final_form: None,             pictograph: "\u{1F981}", phoenician: '\u{1090B}', name: "Lamed",  meaning: "ox goad, toward, teach" },
    LetterInfo { hebrew: '\u{05DE}', final_form: Some('\u{05DD}'), pictograph: "\u{1F30A}", phoenician: '\u{1090C}', name: "Mem",    meaning: "water, chaos, mighty" },
    LetterInfo { hebrew: '\u{05E0}', final_form: Some('\u{05DF}'), pictograph: "\u{1F41F}", phoenician: '\u{1090D}', name: "Nun",    meaning: "fish, life, activity" },
    LetterInfo { hebrew: '\u{05E1}', final_form: None,             pictograph: "\u{1F335}", phoenician: '\u{1090E}', name: "Samech", meaning: "prop, support, surround" },
    LetterInfo { hebrew: '\u{05E2}', final_form: None,             pictograph: "\u{1F441}\u{FE0F}", phoenician: '\u{1090F}', name: "Ayin",   meaning: "eye, see, perceive" },
    LetterInfo { hebrew: '\u{05E4}', final_form: Some('\u{05E3}'), pictograph: "\u{1F4AC}", phoenician: '\u{10910}', name: "Pe",     meaning: "mouth, speak, edge" },
    LetterInfo { hebrew: '\u{05E6}', final_form: Some('\u{05E5}'), pictograph: "\u{1F3A3}", phoenician: '\u{10911}', name: "Tsade",  meaning: "fish hook, desire, righteous" },
    LetterInfo { hebrew: '\u{05E7}', final_form: None,             pictograph: "\u{1F305}", phoenician: '\u{10912}', name: "Qoph",   meaning: "sun on horizon, follow" },
    LetterInfo { hebrew: '\u{05E8}', final_form: None,             pictograph: "\u{1F464}", phoenician: '\u{10913}', name: "Resh",   meaning: "head, first, chief" },
    LetterInfo { hebrew: '\u{05E9}', final_form: None,             pictograph: "\u{1F9B7}", phoenician: '\u{10914}', name: "Shin",   meaning: "teeth, consume, fire" },
    LetterInfo { hebrew: '\u{05EA}', final_form: None,             pictograph: "\u{271D}\u{FE0F}",  phoenician: '\u{10915}', name: "Tav",    meaning: "mark, covenant, sign" },
];

/// Look up a base Hebrew consonant (handles final forms).
pub fn lookup(ch: char) -> Option<&'static LetterInfo> {
    // Direct match
    if let Some(info) = LETTERS.iter().find(|l| l.hebrew == ch) {
        return Some(info);
    }
    // Final form match
    LETTERS.iter().find(|l| l.final_form == Some(ch))
}
