#!/usr/bin/env python3
"""Fetch LXX + KJV data from APIs and generate JSON for the Septuagint reader app."""

import json
import re
import time
import urllib.request

# Greek letter -> Phoenician ancestor mapping
GREEK_LETTERS = {
    # lowercase
    'α': {'p': '🐂', 'ph': '𐤀', 'g': 'α', 'n': 'Alpha', 'm': 'ox'},
    'β': {'p': '🏠', 'ph': '𐤁', 'g': 'β', 'n': 'Beta', 'm': 'house'},
    'γ': {'p': '🐫', 'ph': '𐤂', 'g': 'γ', 'n': 'Gamma', 'm': 'camel'},
    'δ': {'p': '🚪', 'ph': '𐤃', 'g': 'δ', 'n': 'Delta', 'm': 'door'},
    'ε': {'p': '🙋', 'ph': '𐤄', 'g': 'ε', 'n': 'Epsilon', 'm': 'window'},
    'ζ': {'p': '⚔️', 'ph': '𐤆', 'g': 'ζ', 'n': 'Zeta', 'm': 'weapon'},
    'η': {'p': '🧱', 'ph': '𐤇', 'g': 'η', 'n': 'Eta', 'm': 'fence'},
    'θ': {'p': '🐍', 'ph': '𐤈', 'g': 'θ', 'n': 'Theta', 'm': 'serpent'},
    'ι': {'p': '✋', 'ph': '𐤉', 'g': 'ι', 'n': 'Iota', 'm': 'hand'},
    'κ': {'p': '🤲', 'ph': '𐤊', 'g': 'κ', 'n': 'Kappa', 'm': 'palm'},
    'λ': {'p': '🪝', 'ph': '𐤋', 'g': 'λ', 'n': 'Lambda', 'm': 'goad'},
    'μ': {'p': '🌊', 'ph': '𐤌', 'g': 'μ', 'n': 'Mu', 'm': 'water'},
    'ν': {'p': '🐟', 'ph': '𐤍', 'g': 'ν', 'n': 'Nu', 'm': 'fish'},
    'ξ': {'p': '🏛️', 'ph': '𐤎', 'g': 'ξ', 'n': 'Xi', 'm': 'pillar'},
    'ο': {'p': '👁️', 'ph': '𐤏', 'g': 'ο', 'n': 'Omicron', 'm': 'eye'},
    'π': {'p': '👄', 'ph': '𐤐', 'g': 'π', 'n': 'Pi', 'm': 'mouth'},
    'ρ': {'p': '👤', 'ph': '𐤓', 'g': 'ρ', 'n': 'Rho', 'm': 'head'},
    'σ': {'p': '🦷', 'ph': '𐤔', 'g': 'σ', 'n': 'Sigma', 'm': 'teeth'},
    'ς': {'p': '🦷', 'ph': '𐤔', 'g': 'ς', 'n': 'Sigma', 'm': 'teeth'},  # final sigma
    'τ': {'p': '✝️', 'ph': '𐤕', 'g': 'τ', 'n': 'Tau', 'm': 'mark'},
    'υ': {'p': '🪛', 'ph': '𐤅', 'g': 'υ', 'n': 'Upsilon', 'm': 'hook'},
    'φ': {'p': '🎯', 'ph': '𐤒', 'g': 'φ', 'n': 'Phi', 'm': 'needle eye'},
    'χ': {'p': '❌', 'ph': '',  'g': 'χ', 'n': 'Chi', 'm': 'crossed sticks'},
    'ψ': {'p': '🔱', 'ph': '',  'g': 'ψ', 'n': 'Psi', 'm': 'trident'},
    'ω': {'p': '👁️', 'ph': '𐤏', 'g': 'ω', 'n': 'Omega', 'm': 'eye'},
}

# Strip diacritics to find base letter
import unicodedata

def base_letter(ch):
    """Strip combining marks/diacritics from a Greek character to get its base letter."""
    decomposed = unicodedata.normalize('NFD', ch)
    base = ''.join(c for c in decomposed if unicodedata.category(c) != 'Mn')
    return base.lower()

def decompose_word(word):
    """Break a Greek word into its letter components with pictograph data."""
    letters = []
    for ch in word:
        b = base_letter(ch)
        if b in GREEK_LETTERS:
            info = GREEK_LETTERS[b].copy()
            info['g'] = ch  # preserve original character with diacritics
            letters.append(info)
    return letters

def fetch_json(url):
    """Fetch JSON from a URL."""
    req = urllib.request.Request(url, headers={'User-Agent': 'Debar-Bible-App/1.0'})
    with urllib.request.urlopen(req, timeout=30) as resp:
        return json.loads(resp.read().decode('utf-8'))

def fetch_lxx_chapter(book_num, chapter):
    """Fetch LXX Greek text for a chapter."""
    url = f'https://bolls.life/get-text/LXX/{book_num}/{chapter}/'
    return fetch_json(url)

def fetch_kjv_chapter(book_name, chapter):
    """Fetch KJV English text for a chapter."""
    url = f'https://bible-api.com/{book_name}+{chapter}?translation=kjv'
    return fetch_json(url)

# LXX book numbers (bolls.life uses sequential numbering)
# Standard Protestant OT ordering
LXX_BOOKS = {
    'Genesis': 1,
    'Exodus': 2,
    'Leviticus': 3,
    'Numbers': 4,
    'Deuteronomy': 5,
}

# Chapter counts for initial books
CHAPTER_COUNTS = {
    'Genesis': 50,
    'Exodus': 40,
    'Leviticus': 27,
    'Numbers': 36,
    'Deuteronomy': 34,
}

def build_chapter_data(book_name, book_num, chapter):
    """Build chapter data combining LXX and KJV."""
    print(f"  Fetching {book_name} {chapter}...")

    lxx = fetch_lxx_chapter(book_num, chapter)
    time.sleep(0.3)  # be nice to the API

    kjv = fetch_kjv_chapter(book_name, chapter)
    time.sleep(0.3)

    # Build KJV lookup by verse number
    kjv_by_verse = {}
    for v in kjv.get('verses', []):
        text = v['text'].strip().replace('\n', ' ')
        text = re.sub(r'\s+', ' ', text)
        kjv_by_verse[v['verse']] = text

    verses = []
    for lxx_verse in lxx:
        vnum = lxx_verse['verse']
        greek_text = lxx_verse['text'].strip()
        eng_text = kjv_by_verse.get(vnum, '')

        # Split Greek into words
        greek_words = greek_text.split()

        words = []
        for gw in greek_words:
            letters = decompose_word(gw)
            if letters:
                words.append({
                    'gr': gw,
                    'eng': '',  # individual word glosses not available from this API
                    'letters': letters
                })

        verses.append({
            'verse': vnum,
            'greek': greek_text,
            'kjv': eng_text,
            'words': words
        })

    return verses

def main():
    import os

    out_dir = os.path.join(os.path.dirname(__file__), 'www', 'data', 'greek')
    os.makedirs(out_dir, exist_ok=True)

    # Start with Genesis (all 50 chapters)
    for book_name in ['Genesis']:
        book_num = LXX_BOOKS[book_name]
        num_chapters = CHAPTER_COUNTS[book_name]

        print(f"Processing {book_name} ({num_chapters} chapters)...")

        chapters = {}
        for ch in range(1, num_chapters + 1):
            try:
                chapters[str(ch)] = build_chapter_data(book_name, book_num, ch)
            except Exception as e:
                print(f"  ERROR on chapter {ch}: {e}")
                continue

        book_data = {
            'name': book_name,
            'chapters': chapters
        }

        fname = book_name.lower().replace(' ', '_') + '.json'
        out_path = os.path.join(out_dir, fname)
        with open(out_path, 'w', encoding='utf-8') as f:
            json.dump(book_data, f, ensure_ascii=False, indent=None)

        print(f"  Wrote {out_path}")
        print(f"  Size: {os.path.getsize(out_path)} bytes")

if __name__ == '__main__':
    main()
