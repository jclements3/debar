#!/usr/bin/env python3
"""
Parse Open Scriptures Hebrew Bible (morphhb) OSIS XML into JSON format.

Usage:
    python tools/parse_morphhb.py          # Process all books
    python tools/parse_morphhb.py Esth     # Process a single book
"""

import json
import os
import re
import sys
import xml.etree.ElementTree as ET

MORPHHB_DIR = "/tmp/morphhb/wlc"
OUTPUT_DIR = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))), "data")

NS = {"osis": "http://www.bibletechnologies.net/2003/OSIS/namespace"}

BOOK_NAMES = {
    "Gen": "Genesis",
    "Exod": "Exodus",
    "Lev": "Leviticus",
    "Num": "Numbers",
    "Deut": "Deuteronomy",
    "Josh": "Joshua",
    "Judg": "Judges",
    "Ruth": "Ruth",
    "1Sam": "1 Samuel",
    "2Sam": "2 Samuel",
    "1Kgs": "1 Kings",
    "2Kgs": "2 Kings",
    "Isa": "Isaiah",
    "Jer": "Jeremiah",
    "Ezek": "Ezekiel",
    "Hos": "Hosea",
    "Joel": "Joel",
    "Amos": "Amos",
    "Obad": "Obadiah",
    "Jonah": "Jonah",
    "Mic": "Micah",
    "Nah": "Nahum",
    "Hab": "Habakkuk",
    "Zeph": "Zephaniah",
    "Hag": "Haggai",
    "Zech": "Zechariah",
    "Mal": "Malachi",
    "Ps": "Psalms",
    "Prov": "Proverbs",
    "Job": "Job",
    "Song": "Song of Songs",
    "Eccl": "Ecclesiastes",
    "Lam": "Lamentations",
    "Esth": "Esther",
    "Dan": "Daniel",
    "Ezra": "Ezra",
    "Neh": "Nehemiah",
    "1Chr": "1 Chronicles",
    "2Chr": "2 Chronicles",
}

# Cantillation marks: U+0591 to U+05AF
CANTILLATION_RE = re.compile("[\u0591-\u05AF]")


def strip_cantillation(text):
    """Remove cantillation marks but keep niqqud vowels (U+05B0-U+05C7)."""
    return CANTILLATION_RE.sub("", text)


def clean_hebrew(text):
    """Strip segment separators and cantillation from Hebrew word text."""
    # Remove the / segment separators
    text = text.replace("/", "")
    # Remove cantillation marks
    text = strip_cantillation(text)
    return text


def clean_lemma(lemma):
    """Strip prefix codes from lemma, keeping only Strong's numbers.

    Examples:
        'b/7225' -> '7225'
        'c/1961' -> '1961'
        '325'    -> '325'
        'c/d/3064' -> '3064'
        '4427 a' -> '4427 a'
    """
    parts = lemma.split("/")
    return parts[-1]


def get_word_text(w_elem):
    """Get the full text content of a <w> element, including tail-less children."""
    # itertext() gets all text content including from sub-elements
    return "".join(w_elem.itertext())


def parse_verse(verse_elem):
    """Parse a verse element, handling ketiv/qere and extracting words."""
    words = []

    # We need to walk the verse children carefully to handle ketiv/qere.
    # Strategy: iterate direct children. Skip ketiv <w> elements.
    # For <note type="variant">, extract the qere <w> from inside.
    # For normal <w> elements, extract directly.

    for child in verse_elem:
        tag = child.tag.replace("{" + NS["osis"] + "}", "")

        if tag == "w":
            # Check if this is a ketiv word (skip it -- qere comes in the note)
            if child.get("type") == "x-ketiv":
                continue

            text = get_word_text(child)
            heb = clean_hebrew(text)
            lemma = clean_lemma(child.get("lemma", ""))
            morph = child.get("morph", "")

            if heb:
                words.append({"heb": heb, "lemma": lemma, "morph": morph})

        elif tag == "note":
            # Check for variant notes containing qere readings
            if child.get("type") == "variant":
                # Find qere reading: <rdg type="x-qere"><w ...>
                for rdg in child.findall("osis:rdg", NS):
                    if rdg.get("type") == "x-qere":
                        for w in rdg.findall("osis:w", NS):
                            text = get_word_text(w)
                            heb = clean_hebrew(text)
                            lemma = clean_lemma(w.get("lemma", ""))
                            morph = w.get("morph", "")

                            if heb:
                                words.append({"heb": heb, "lemma": lemma, "morph": morph})

        # Skip <seg> elements (punctuation) and other notes

    return words


def parse_book(osis_id):
    """Parse a single book XML file and return the JSON structure."""
    xml_path = os.path.join(MORPHHB_DIR, f"{osis_id}.xml")
    if not os.path.exists(xml_path):
        print(f"  WARNING: {xml_path} not found, skipping.")
        return None

    book_name = BOOK_NAMES.get(osis_id, osis_id)
    print(f"  Parsing {book_name} ({osis_id})...")

    tree = ET.parse(xml_path)
    root = tree.getroot()

    chapters = {}

    for chapter_elem in root.iter("{%s}chapter" % NS["osis"]):
        chapter_osis = chapter_elem.get("osisID", "")
        # e.g. "Esth.1" -> chapter number "1"
        chapter_num = chapter_osis.split(".")[-1]

        verses = []
        for verse_elem in chapter_elem.findall("osis:verse", NS):
            verse_osis = verse_elem.get("osisID", "")
            # e.g. "Esth.1.1" -> verse number 1
            verse_num = int(verse_osis.split(".")[-1])

            words = parse_verse(verse_elem)
            ref = f"{book_name} {chapter_num}:{verse_num}"

            verses.append({
                "verse": verse_num,
                "ref": ref,
                "words": words,
            })

        chapters[chapter_num] = verses
        print(f"    Chapter {chapter_num}: {len(verses)} verses")

    return {
        "name": book_name,
        "osisId": osis_id,
        "chapters": chapters,
    }


def main():
    os.makedirs(OUTPUT_DIR, exist_ok=True)

    if len(sys.argv) > 1:
        book_ids = [sys.argv[1]]
    else:
        book_ids = list(BOOK_NAMES.keys())

    for osis_id in book_ids:
        if osis_id not in BOOK_NAMES:
            print(f"Unknown book ID: {osis_id}")
            continue

        result = parse_book(osis_id)
        if result is None:
            continue

        book_name = BOOK_NAMES[osis_id]
        out_path = os.path.join(OUTPUT_DIR, f"{book_name}.json")
        with open(out_path, "w", encoding="utf-8") as f:
            json.dump(result, f, ensure_ascii=False, indent=2)

        total_verses = sum(len(v) for v in result["chapters"].values())
        total_words = sum(
            len(verse["words"])
            for ch in result["chapters"].values()
            for verse in ch
        )
        print(f"  Wrote {out_path} ({len(result['chapters'])} chapters, {total_verses} verses, {total_words} words)")


if __name__ == "__main__":
    main()
