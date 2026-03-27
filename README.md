# Pictograph Bible

A Hebrew pictograph study toolkit connecting Proto-Sinaitic/Phoenician letter pictures
to biblical vocabulary and scripture text.

## The Core Idea

Every Hebrew root is a 3-letter pictograph that tells a story.
🌊✋🦁 → מלכ → "the force that directs the masses and holds power" → מֶלֶךְ (melek, king)

## Files

### Study Apps (open in any browser)
- **genesis1_pictograph.html** — Genesis 1 word-by-word pictograph reader
  - Every word broken into picture → Phoenician → Hebrew letter
  - Click any word for full letter breakdown + pictorial memory narrative
  - Toggle pictures, Phoenician, transliteration, English on/off
  - Works fully offline

- **hebrew_app.html** — Full Hebrew reference app
  - 22-letter alphabet table (picture → Phoenician → Hebrew → meaning)
  - 12 root families with vocabulary (pics → root → combined meaning → words)
  - Flashcard mode (3 decks: Letters, Roots, Vocabulary)
  - Anki CSV download for all 3 decks

### Reference Document
- **hebrew_reference.docx** — Printable Word document
  - All 22 letters table
  - 12 root families with vocabulary and memory hooks

### Source Data
- **gen1_data.js** — Genesis 1 Hebrew word/letter data (Node.js module)
- **narratives.json** — 69 pre-generated pictorial memory narratives (keyed by Hebrew word)
- **hebrew_ref.js** — Word document generator script (requires: npm install docx)

## Android Build

Build a signed APK for sideloading onto an Android tablet:

```bash
# Prerequisites (Ubuntu)
export ANDROID_HOME=~/Android/Sdk
export JAVA_HOME=/usr/lib/jvm/java-21-openjdk-amd64
export PATH="$ANDROID_HOME/cmdline-tools/latest/bin:$ANDROID_HOME/platform-tools:$PATH"

# Sync web assets and build release APK
npm run build:release
# outputs: android/app/build/outputs/apk/release/app-release.apk

# Install on connected device
adb install android/app/build/outputs/apk/release/app-release.apk
```

Other build commands:
- `npm run build:debug` — debug APK with dev tools
- `npm run sync` — sync web assets to Android project without building

Target: Android 16 (API 35), min SDK 22. Signing keystore: `android/debar-release.keystore`.

### Installing on a tablet without USB

Transfer the APK file (17 MB) via Google Drive, email, or any file manager, then tap to install. First time sideloading: go to **Settings > Security** (or **Apps > Special app access**) and enable **Install unknown apps** for your file manager or browser.

## Rebuild the Word Doc
```bash
npm install docx
node hebrew_ref.js
# outputs: hebrew_reference.docx
```

## Next Steps / Roadmap
- Expand to Genesis 2–3
- Add all 30 high-yield root families
- Red-letter NT words (Yeshua's words in pictograph)
- 30-day study schedule
- Psalm 23 pictograph reader

## Letter Quick Reference
| Picture | Hebrew | Name | Core Meaning |
|---------|--------|------|-------------|
| 🐂 | א | Aleph | strength, leader, first |
| 🏠 | ב | Beth | house, inside, family |
| 🐪 | ג | Gimel | camel, carry, lift |
| 🚪 | ד | Dalet | door, pathway, entry |
| 👐 | ה | He | window, breath, behold |
| 🪝 | ו | Vav | tent peg, hook, connect |
| ⚔️ | ז | Zayin | weapon, cut, sustain |
| 🧱 | ח | Chet | fence, inner room, separate |
| 🐍 | ט | Tet | snake, surround, contain |
| 🤲 | י | Yod | hand, deed, work |
| ✋ | כ | Kaph | open palm, bend, yield |
| 🦁 | ל | Lamed | ox goad, toward, teach |
| 🌊 | מ | Mem | water, chaos, mighty |
| 🐟 | נ | Nun | fish, life, activity |
| 🌵 | ס | Samech | prop, support, surround |
| 👁️ | ע | Ayin | eye, see, perceive |
| 💬 | פ | Pe | mouth, speak, edge |
| 🎣 | צ | Tsade | fish hook, desire, righteous |
| 🌅 | ק | Qoph | sun on horizon, follow |
| 👤 | ר | Resh | head, first, chief |
| 🦷 | ש | Shin | teeth, consume, fire |
| ✝️ | ת | Tav | mark, covenant, sign |
