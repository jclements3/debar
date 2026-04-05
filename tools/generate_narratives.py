#!/usr/bin/env python3
"""
Generate pictograph memory narratives for the top 500 Hebrew words.
Each narrative connects letter pictures to the word's meaning.
"""

import json
import unicodedata
import os

# 22 Hebrew letters with their pictograph meanings
LETTERS = {
    '\u05D0': {'name': 'Aleph', 'pic': '🐂', 'meaning': 'strength, leader, first'},
    '\u05D1': {'name': 'Beth', 'pic': '🏠', 'meaning': 'house, inside, family'},
    '\u05D2': {'name': 'Gimel', 'pic': '🐫', 'meaning': 'camel, carry, lift'},
    '\u05D3': {'name': 'Dalet', 'pic': '🚪', 'meaning': 'door, pathway, entry'},
    '\u05D4': {'name': 'He', 'pic': '🙌', 'meaning': 'window, breath, behold'},
    '\u05D5': {'name': 'Vav', 'pic': '🪝', 'meaning': 'tent peg, hook, connect'},
    '\u05D6': {'name': 'Zayin', 'pic': '⚔️', 'meaning': 'weapon, cut, sustain'},
    '\u05D7': {'name': 'Chet', 'pic': '🧱', 'meaning': 'fence, inner room, separate'},
    '\u05D8': {'name': 'Tet', 'pic': '🐍', 'meaning': 'snake, surround, contain'},
    '\u05D9': {'name': 'Yod', 'pic': '🤲', 'meaning': 'hand, deed, work'},
    '\u05DA': {'name': 'Kaph', 'pic': '✋', 'meaning': 'open palm, bend, yield'},  # final
    '\u05DB': {'name': 'Kaph', 'pic': '✋', 'meaning': 'open palm, bend, yield'},
    '\u05DC': {'name': 'Lamed', 'pic': '🦁', 'meaning': 'ox goad, toward, teach'},
    '\u05DD': {'name': 'Mem', 'pic': '🌊', 'meaning': 'water, chaos, mighty'},  # final
    '\u05DE': {'name': 'Mem', 'pic': '🌊', 'meaning': 'water, chaos, mighty'},
    '\u05DF': {'name': 'Nun', 'pic': '🐟', 'meaning': 'fish, life, activity'},  # final
    '\u05E0': {'name': 'Nun', 'pic': '🐟', 'meaning': 'fish, life, activity'},
    '\u05E1': {'name': 'Samech', 'pic': '🌵', 'meaning': 'prop, support, surround'},
    '\u05E2': {'name': 'Ayin', 'pic': '👁️', 'meaning': 'eye, see, perceive'},
    '\u05E3': {'name': 'Pe', 'pic': '💬', 'meaning': 'mouth, speak, edge'},  # final
    '\u05E4': {'name': 'Pe', 'pic': '💬', 'meaning': 'mouth, speak, edge'},
    '\u05E5': {'name': 'Tsade', 'pic': '🎣', 'meaning': 'fish hook, desire, righteous'},  # final
    '\u05E6': {'name': 'Tsade', 'pic': '🎣', 'meaning': 'fish hook, desire, righteous'},
    '\u05E7': {'name': 'Qoph', 'pic': '🌅', 'meaning': 'sun on horizon, follow'},
    '\u05E8': {'name': 'Resh', 'pic': '👤', 'meaning': 'head, first, chief'},
    '\u05E9': {'name': 'Shin', 'pic': '🦷', 'meaning': 'teeth, consume, fire'},
    '\u05EA': {'name': 'Tav', 'pic': '✝️', 'meaning': 'mark, covenant, sign'},
}

def strip_niqqud(text):
    """Remove vowel points and cantillation marks, keep consonants."""
    result = []
    for ch in text:
        if ch in LETTERS:
            result.append(ch)
    return result

def decompose(heb_word):
    """Get letter breakdown for a Hebrew word."""
    consonants = strip_niqqud(heb_word)
    return [LETTERS[c] for c in consonants if c in LETTERS]

def main():
    base = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))

    with open(os.path.join(base, 'www/data/top500.json'), 'r') as f:
        top500 = json.load(f)

    with open(os.path.join(base, 'www/data/strongs.json'), 'r') as f:
        strongs = json.load(f)

    with open(os.path.join(base, 'www/narratives.json'), 'r') as f:
        existing = json.load(f)

    # Build decomposition data for each word
    words = []
    for entry in top500:
        heb = entry['heb']
        eng = entry['eng']
        trans = entry['trans']
        lemma = entry['lemma'].split()[0]  # strip "a" suffix

        # Get fuller definition from Strong's
        strong_def = ''
        strong_usage = ''
        if lemma in strongs:
            strong_def = strongs[lemma].get('def', '')
            strong_usage = strongs[lemma].get('usage', '')

        letters = decompose(heb)
        if not letters:
            continue

        letter_descs = []
        for l in letters:
            letter_descs.append(f"{l['name']} ({l['pic']} {l['meaning'].split(',')[0]})")

        words.append({
            'rank': entry['rank'],
            'heb': heb,
            'eng': eng,
            'trans': trans,
            'lemma': lemma,
            'def': strong_def,
            'usage': strong_usage,
            'letters': letters,
            'letter_summary': ' + '.join(letter_descs),
            'has_narrative': heb in existing,
        })

    # Output for narrative generation
    # Group into batches of 25 for processing
    print(f"Total words: {len(words)}")
    print(f"Already have narratives: {sum(1 for w in words if w['has_narrative'])}")
    print(f"Need narratives: {sum(1 for w in words if not w['has_narrative'])}")
    print()

    # Write the decomposition data for reference
    out_path = os.path.join(base, 'tools/top500_decomposed.json')
    out_data = []
    for w in words:
        out_data.append({
            'rank': w['rank'],
            'heb': w['heb'],
            'eng': w['eng'],
            'trans': w['trans'],
            'def': w['def'],
            'usage': w['usage'],
            'letters': [{'name': l['name'], 'pic': l['pic'], 'meaning': l['meaning']} for l in w['letters']],
            'letter_summary': w['letter_summary'],
        })
    with open(out_path, 'w', encoding='utf-8') as f:
        json.dump(out_data, f, ensure_ascii=False, indent=2)
    print(f"Wrote decomposition data to {out_path}")

if __name__ == '__main__':
    main()
