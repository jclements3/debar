# Handoff Note — Lab Computer Session (2026-04-05)

## What was done this session

### 1. Septuagint (LXX) Greek Bible app added
- New file: `www/greek.html` — Septuagint reader with Greek word cards, Phoenician ancestor letters, pictograph breakdowns, KJV English translation
- Data: `www/data/greek/genesis.json` — All 50 chapters of Genesis (LXX Greek + KJV English), fetched from bolls.life and bible-api.com APIs
- Book list: `www/greek_books.js` — Currently only Genesis; more OT books can be added by running `generate_greek_data.py` with additional books
- Uses `--greek-blue: #1a4a7a` accent color and GFS Didot font for Greek text
- Card added to `www/index.html` between Chinese Bible and Aramaic Targum
- LXX text is lemmatized (dictionary forms) — fine for vocabulary study, not inflected

### 2. Hebrew word font sizes increased
- `.w-hebrew` bumped from `1.4rem` to `2.5rem` in `www/hebrew.html`
- `.w-trans` (transliteration) bumped from `0.55rem` to `0.85rem`
- `.w-eng` (English gloss) bumped from `0.75rem` to `1.1rem`
- Same changes applied to root-level `hebrew.html`

### 3. 531 pictograph memory narratives generated
- `www/narratives.json` now has 531 entries (was 69)
- Covers the top 500 most frequent Hebrew words from `www/data/top500.json`
- Each narrative connects Proto-Sinaitic letter pictures to the word's meaning as a 1-2 sentence mnemonic
- Generated in 5 parallel batches, merged with existing narratives
- Batch files in `tools/narratives_batch_[1-5].json` (can be deleted)
- Decomposition data in `tools/top500_decomposed.json` (useful reference)

## Key files modified
- `www/index.html` — Added Greek card + GFS Didot font
- `www/hebrew.html` — Font size increases
- `www/narratives.json` — 531 narratives (was 69)
- `www/greek.html` — NEW: Septuagint reader
- `www/greek_books.js` — NEW: LXX book list
- `www/data/greek/genesis.json` — NEW: LXX + KJV data

## Important: www/ vs root files
Capacitor builds from `www/` directory. Root-level HTML files are for direct browser testing. Changes must be made to BOTH or synced. The root copies of `index.html`, `hebrew.html`, `narratives.json` were updated, but `greek.html` and `greek_books.js` were only copied — if you edit, edit `www/` first then copy.

## To add more LXX books
Edit `generate_greek_data.py` — add books to `LXX_BOOKS` and `CHAPTER_COUNTS` dicts, then run it. Watch for API rate limits (bolls.life throttles after ~40 rapid requests). The script has 0.3s delays but may need 1.5s for large batches. Also update `www/greek_books.js` with new entries.

## Build commands
```bash
npx cap sync android
cd android && ./gradlew assembleDebug
adb install -r android/app/build/outputs/apk/debug/app-debug.apk
adb shell pm clear com.debar.pictographbible  # clear WebView cache
adb shell am start -n com.debar.pictographbible/.MainActivity
```

## Not yet committed
All changes are uncommitted. Review with `git diff` and `git status` before committing.
