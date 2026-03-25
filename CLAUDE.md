# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Pictograph Bible — a Hebrew pictograph study toolkit connecting Proto-Sinaitic/Phoenician letter pictures to biblical vocabulary and Genesis 1 scripture. All apps are static HTML/JS that run offline in a browser with no build step.

## Architecture

- **genesis1.html** — Genesis 1 word-by-word pictograph reader. Imports `gen1_data.js` and `narratives.json` at runtime. Self-contained CSS/JS in a single HTML file.
- **hebrew_app.html** — Full reference app (alphabet table, root families, flashcards, Anki export). Self-contained single-file app.
- **gen1_data.js** — CommonJS module (`const genesis1 = [...]`) exporting verse-by-verse word data. Each word has Hebrew text, transliteration, English gloss, and letter-by-letter breakdown (emoji pictograph, Phoenician glyph, Hebrew letter, letter name).
- **narratives.json** — 69 pictorial memory narratives keyed by Hebrew word. These are human-readable stories connecting letter pictures to word meanings.
- **hebrew_ref.js** — Node.js script that generates `hebrew_reference.docx` using the `docx` npm package.

## Commands

```bash
# Rebuild the Word document (only command in the project)
npm install docx
node hebrew_ref.js
# outputs: hebrew_reference.docx
```

## Data Conventions

- Letter data uses a consistent structure: `{p: "emoji", ph: "Phoenician", h: "Hebrew", n: "name"}`
- In `gen1_data.js`, the short keys are: `p` = pictograph emoji, `ph` = Phoenician Unicode char, `h` = Hebrew letter, `n` = letter name
- Both HTML apps share the same parchment-gold CSS theme (CSS variables defined in `:root`)
- Hebrew text uses Unicode with niqqud (vowel points)
- Phoenician glyphs use Unicode block U+10900–U+1091F

## Roadmap

Planned expansions: Genesis 2–3, additional root families (30 total), red-letter NT pictographs, Psalm 23 reader, 30-day study schedule.
