#!/usr/bin/env python3
"""Add 'trans' and 'eng' fields to each word in data/*.json using strongs.json."""

import glob
import json
import os
import re

DATA_DIR = os.path.join(os.path.dirname(__file__), "..", "data")
STRONGS_PATH = os.path.join(DATA_DIR, "strongs.json")


def load_strongs():
    with open(STRONGS_PATH, "r", encoding="utf-8") as f:
        return json.load(f)


def primary_number(lemma):
    """Extract the primary Strong's number from a lemma field.

    Handles cases like "4427 a", "7225", "1961 a", etc.
    Returns the numeric portion as a string, or None.
    """
    if not lemma:
        return None
    m = re.match(r"(\d+)", lemma.strip())
    return m.group(1) if m else None


def annotate_book(book_path, strongs):
    with open(book_path, "r", encoding="utf-8") as f:
        book = json.load(f)

    hits = 0
    misses = 0

    for chap_num, verses in book.get("chapters", {}).items():
        for verse in verses:
            for word in verse.get("words", []):
                num = primary_number(word.get("lemma", ""))
                if num and num in strongs:
                    entry = strongs[num]
                    word["trans"] = entry["xlit"]
                    word["eng"] = entry["def"]
                    hits += 1
                else:
                    # Leave fields empty rather than omitting them
                    word["trans"] = ""
                    word["eng"] = ""
                    misses += 1

    with open(book_path, "w", encoding="utf-8") as f:
        json.dump(book, f, ensure_ascii=False, indent=2)
        f.write("\n")

    return hits, misses


def main():
    strongs = load_strongs()
    print(f"Loaded {len(strongs)} Strong's entries")

    book_files = sorted(glob.glob(os.path.join(DATA_DIR, "*.json")))
    # Exclude strongs.json itself
    book_files = [f for f in book_files if os.path.basename(f) != "strongs.json"]

    total_hits = 0
    total_misses = 0

    for path in book_files:
        name = os.path.basename(path)
        hits, misses = annotate_book(path, strongs)
        total_hits += hits
        total_misses += misses
        print(f"  {name}: {hits} glossed, {misses} unresolved")

    print(f"\nTotal: {total_hits} glossed, {total_misses} unresolved "
          f"({100*total_hits/(total_hits+total_misses):.1f}% coverage)")


if __name__ == "__main__":
    main()
