#!/usr/bin/env python3
"""Add KJV English translations to Hebrew Bible verse data.

Downloads KJV text from github.com/aruljohn/Bible-kjv and adds an "english"
field to each verse in our data/*.json files.

Usage:
    python add_kjv.py              # process all books
    python add_kjv.py Genesis      # process one book
    python add_kjv.py "1 Samuel"   # quote names with spaces
"""

import json
import os
import sys
import urllib.request
import urllib.error

DATA_DIR = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))), "data")
CACHE_DIR = "/tmp/kjv"
BASE_URL = "https://raw.githubusercontent.com/aruljohn/Bible-kjv/master"

# Map our book names -> KJV repo filenames (without .json).
# The KJV repo uses names with spaces removed for filenames.
# Our "Song of Songs" is KJV's "Song of Solomon".
OUR_TO_KJV = {
    "Genesis": "Genesis",
    "Exodus": "Exodus",
    "Leviticus": "Leviticus",
    "Numbers": "Numbers",
    "Deuteronomy": "Deuteronomy",
    "Joshua": "Joshua",
    "Judges": "Judges",
    "Ruth": "Ruth",
    "1 Samuel": "1Samuel",
    "2 Samuel": "2Samuel",
    "1 Kings": "1Kings",
    "2 Kings": "2Kings",
    "1 Chronicles": "1Chronicles",
    "2 Chronicles": "2Chronicles",
    "Ezra": "Ezra",
    "Nehemiah": "Nehemiah",
    "Esther": "Esther",
    "Job": "Job",
    "Psalms": "Psalms",
    "Proverbs": "Proverbs",
    "Ecclesiastes": "Ecclesiastes",
    "Song of Songs": "SongofSolomon",
    "Isaiah": "Isaiah",
    "Jeremiah": "Jeremiah",
    "Lamentations": "Lamentations",
    "Ezekiel": "Ezekiel",
    "Daniel": "Daniel",
    "Hosea": "Hosea",
    "Joel": "Joel",
    "Amos": "Amos",
    "Obadiah": "Obadiah",
    "Jonah": "Jonah",
    "Micah": "Micah",
    "Nahum": "Nahum",
    "Habakkuk": "Habakkuk",
    "Zephaniah": "Zephaniah",
    "Haggai": "Haggai",
    "Zechariah": "Zechariah",
    "Malachi": "Malachi",
}


def download_kjv(kjv_filename):
    """Download a KJV book JSON, caching in /tmp/kjv/."""
    os.makedirs(CACHE_DIR, exist_ok=True)
    cache_path = os.path.join(CACHE_DIR, kjv_filename + ".json")

    if os.path.exists(cache_path):
        with open(cache_path, "r", encoding="utf-8") as f:
            return json.load(f)

    url = "{}/{}.json".format(BASE_URL, kjv_filename)
    print("  Downloading {}".format(url))
    try:
        req = urllib.request.Request(url, headers={"User-Agent": "Mozilla/5.0"})
        with urllib.request.urlopen(req, timeout=30) as resp:
            raw = resp.read()
    except urllib.error.HTTPError as e:
        print("  ERROR: HTTP {} fetching {}".format(e.code, url))
        return None

    with open(cache_path, "wb") as f:
        f.write(raw)

    return json.loads(raw.decode("utf-8"))


def build_verse_lookup(kjv_data):
    """Build a dict of (chapter_str, verse_str) -> text from KJV data."""
    lookup = {}
    for ch in kjv_data.get("chapters", []):
        ch_num = str(ch["chapter"])
        for v in ch.get("verses", []):
            v_num = str(v["verse"])
            lookup[(ch_num, v_num)] = v["text"]
    return lookup


def process_book(book_name):
    """Add KJV English text to a single book's data file."""
    data_path = os.path.join(DATA_DIR, book_name + ".json")
    if not os.path.exists(data_path):
        print("  SKIP: {} not found".format(data_path))
        return False

    kjv_filename = OUR_TO_KJV.get(book_name)
    if kjv_filename is None:
        print("  SKIP: no KJV mapping for '{}'".format(book_name))
        return False

    kjv_data = download_kjv(kjv_filename)
    if kjv_data is None:
        return False

    lookup = build_verse_lookup(kjv_data)

    with open(data_path, "r", encoding="utf-8") as f:
        book = json.load(f)

    matched = 0
    missed = 0
    for ch_num, verses in book.get("chapters", {}).items():
        for verse in verses:
            v_num = str(verse["verse"])
            text = lookup.get((str(ch_num), v_num))
            if text:
                verse["english"] = text
                matched += 1
            else:
                missed += 1

    with open(data_path, "w", encoding="utf-8") as f:
        json.dump(book, f, ensure_ascii=False, indent=2)
        f.write("\n")

    print("  {} -- matched: {}, missed: {}".format(book_name, matched, missed))
    return True


def get_all_books():
    """Return list of book names from data directory (excluding non-book files)."""
    books = []
    for fname in sorted(os.listdir(DATA_DIR)):
        if fname.endswith(".json") and fname != "strongs.json":
            books.append(fname[:-5])  # strip .json
    return books


def main():
    if len(sys.argv) > 1:
        books = [" ".join(sys.argv[1:])]
    else:
        books = get_all_books()

    print("Processing {} book(s)...".format(len(books)))
    success = 0
    for book_name in books:
        print("[{}]".format(book_name))
        if process_book(book_name):
            success += 1

    print("\nDone. {}/{} books processed.".format(success, len(books)))


if __name__ == "__main__":
    main()
