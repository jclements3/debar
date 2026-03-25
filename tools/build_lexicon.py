#!/usr/bin/env python3
"""Download OpenScriptures Strong's Hebrew dictionary and build data/strongs.json."""

import json
import os
import urllib.request
import xml.etree.ElementTree as ET

URL = "https://raw.githubusercontent.com/openscriptures/HebrewLexicon/master/HebrewStrong.xml"
CACHE_PATH = "/tmp/HebrewStrong.xml"
OUTPUT_PATH = os.path.join(os.path.dirname(__file__), "..", "data", "strongs.json")


def download_xml():
    if os.path.exists(CACHE_PATH):
        print(f"Using cached XML: {CACHE_PATH}")
    else:
        print(f"Downloading {URL} ...")
        urllib.request.urlretrieve(URL, CACHE_PATH)
        print(f"Saved to {CACHE_PATH}")
    return CACHE_PATH


def get_text(elem):
    """Recursively extract all text content from an element."""
    parts = []
    if elem.text:
        parts.append(elem.text)
    for child in elem:
        parts.append(get_text(child))
        if child.tail:
            parts.append(child.tail)
    return "".join(parts).strip()


def parse_xml(path):
    tree = ET.parse(path)
    root = tree.getroot()
    lexicon = {}

    # Detect namespace from root tag (e.g. {http://...}lexicon)
    ns = ""
    if root.tag.startswith("{"):
        ns = root.tag.split("}")[0] + "}"

    def tag(name):
        return f"{ns}{name}"

    for entry in root.iter(tag("entry")):
        entry_id = entry.get("id", "")
        if not entry_id.startswith("H"):
            continue
        num = entry_id[1:]  # strip the "H" prefix

        # Extract word and transliteration from <w> element
        w_elem = entry.find(tag("w"))
        word = ""
        xlit = ""
        if w_elem is not None:
            word = w_elem.text.strip() if w_elem.text else ""
            xlit = w_elem.get("xlit", "")

        # Extract definition from <meaning><def>
        definition = ""
        meaning_elem = entry.find(tag("meaning"))
        if meaning_elem is not None:
            def_elem = meaning_elem.find(tag("def"))
            if def_elem is not None:
                definition = get_text(def_elem)

        # Extract usage string
        usage = ""
        usage_elem = entry.find(tag("usage"))
        if usage_elem is not None:
            usage = get_text(usage_elem)

        lexicon[num] = {
            "word": word,
            "xlit": xlit,
            "def": definition,
            "usage": usage,
        }

    return lexicon


def main():
    xml_path = download_xml()
    lexicon = parse_xml(xml_path)
    print(f"Parsed {len(lexicon)} entries")

    out = os.path.normpath(OUTPUT_PATH)
    os.makedirs(os.path.dirname(out), exist_ok=True)

    with open(out, "w", encoding="utf-8") as f:
        json.dump(lexicon, f, ensure_ascii=False, indent=1, sort_keys=True)
        f.write("\n")

    print(f"Wrote {out}")

    # Quick sanity check
    for key in ("1", "430", "1254", "7225"):
        if key in lexicon:
            e = lexicon[key]
            print(f"  H{key}: {e['word']} ({e['xlit']}) = {e['def']}")


if __name__ == "__main__":
    main()
