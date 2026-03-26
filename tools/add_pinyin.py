#!/usr/bin/env python3
"""Add pinyin readings and English definitions to Chinese Bible JSON files.

Uses the Unicode Unihan database to map Chinese characters to their
Mandarin pinyin readings and short English definitions.

Usage:
    python add_pinyin.py              # process all books
    python add_pinyin.py Genesis      # process a single book
"""

import json
import os
import sys
import urllib.request
import zipfile

UNIHAN_URL = "https://www.unicode.org/Public/UCD/latest/ucd/Unihan.zip"
UNIHAN_ZIP_PATH = "/tmp/Unihan.zip"
UNIHAN_READINGS_PATH = "/tmp/Unihan_Readings.txt"

DATA_DIR = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))), "data", "chinese")


def download_unihan():
    """Download and extract Unihan_Readings.txt if not already cached."""
    if os.path.exists(UNIHAN_READINGS_PATH):
        print(f"Using cached {UNIHAN_READINGS_PATH}")
        return

    if not os.path.exists(UNIHAN_ZIP_PATH):
        print(f"Downloading {UNIHAN_URL} ...")
        urllib.request.urlretrieve(UNIHAN_URL, UNIHAN_ZIP_PATH)
        print("Download complete.")

    print("Extracting Unihan_Readings.txt ...")
    with zipfile.ZipFile(UNIHAN_ZIP_PATH, "r") as zf:
        zf.extract("Unihan_Readings.txt", "/tmp")
    print("Extraction complete.")


def parse_unihan():
    """Parse Unihan_Readings.txt and return two dicts: pinyin and definition.

    Returns:
        (pinyin_map, definition_map) where keys are single characters and
        values are the pinyin string or short English definition.
    """
    pinyin_map = {}
    definition_map = {}

    with open(UNIHAN_READINGS_PATH, "r", encoding="utf-8") as f:
        for line in f:
            line = line.strip()
            if not line or line.startswith("#"):
                continue

            parts = line.split("\t")
            if len(parts) < 3:
                continue

            codepoint_str, field, value = parts[0], parts[1], parts[2]

            # Parse U+XXXX to a character
            if not codepoint_str.startswith("U+"):
                continue
            codepoint = int(codepoint_str[2:], 16)
            char = chr(codepoint)

            if field == "kMandarin":
                # kMandarin may have multiple readings separated by spaces;
                # take the first one (the most common reading).
                pinyin_map[char] = value.split()[0]
            elif field == "kDefinition":
                # Take the first definition before the semicolon.
                short_def = value.split(";")[0].strip()
                definition_map[char] = short_def

    return pinyin_map, definition_map


def process_book(filepath, pinyin_map, definition_map):
    """Update a single book JSON file with pinyin and definitions.

    Returns:
        (total_chars, updated_chars) count tuple.
    """
    with open(filepath, "r", encoding="utf-8") as f:
        book = json.load(f)

    total = 0
    updated = 0

    chapters = book.get("chapters", {})
    for ch_key in chapters:
        for verse in chapters[ch_key]:
            for char_entry in verse.get("chars", []):
                ch = char_entry.get("ch", "")
                if not ch:
                    continue

                total += 1

                # Only fill in empty fields
                if char_entry.get("pinyin") == "":
                    py = pinyin_map.get(ch, "")
                    if py:
                        char_entry["pinyin"] = py
                        updated += 1

                if char_entry.get("eng") == "":
                    defn = definition_map.get(ch, "")
                    if defn:
                        char_entry["eng"] = defn

    with open(filepath, "w", encoding="utf-8") as f:
        json.dump(book, f, ensure_ascii=False)

    return total, updated


def main():
    book_filter = sys.argv[1] if len(sys.argv) > 1 else None

    # Step 1: Download and extract Unihan data
    download_unihan()

    # Step 2: Parse the readings file
    print("Parsing Unihan_Readings.txt ...")
    pinyin_map, definition_map = parse_unihan()
    print(f"Loaded {len(pinyin_map)} pinyin entries, {len(definition_map)} definitions.")

    # Step 3: Find and process book files
    if not os.path.isdir(DATA_DIR):
        print(f"Data directory not found: {DATA_DIR}")
        sys.exit(1)

    json_files = sorted(
        f for f in os.listdir(DATA_DIR) if f.endswith(".json")
    )

    if book_filter:
        # Match by filename stem (case-insensitive)
        target = book_filter.lower()
        json_files = [
            f for f in json_files
            if os.path.splitext(f)[0].lower() == target
        ]
        if not json_files:
            print(f"No JSON file found for book: {book_filter}")
            sys.exit(1)

    if not json_files:
        print("No JSON files found in", DATA_DIR)
        sys.exit(1)

    grand_total = 0
    grand_updated = 0

    for filename in json_files:
        filepath = os.path.join(DATA_DIR, filename)
        total, updated = process_book(filepath, pinyin_map, definition_map)
        grand_total += total
        grand_updated += updated
        print(f"  {filename}: {updated}/{total} characters updated")

    print(f"Done. {grand_updated}/{grand_total} characters updated across {len(json_files)} file(s).")


if __name__ == "__main__":
    main()
