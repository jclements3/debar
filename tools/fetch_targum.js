#!/usr/bin/env node
// Fetch Targum Onkelos (Aramaic Torah) from Sefaria API.
// Output: data/aramaic/<BookName>.json per book, plus aramaic_books.js
//
// Usage: node tools/fetch_targum.js

const fs = require('fs');
const path = require('path');
const https = require('https');

const OUT_DIR = path.join(__dirname, '..', 'data', 'aramaic');

function sleep(ms) { return new Promise(r => setTimeout(r, ms)); }

function fetchJSON(url, retries = 3) {
  return new Promise((resolve, reject) => {
    https.get(url, res => {
      let body = '';
      res.on('data', chunk => body += chunk);
      res.on('end', async () => {
        try {
          const data = JSON.parse(body);
          resolve(data);
        } catch(e) {
          if (retries > 0) { await sleep(2000); resolve(fetchJSON(url, retries - 1)); }
          else reject(new Error('JSON parse error: ' + e.message));
        }
      });
      res.on('error', reject);
    }).on('error', async (err) => {
      if (retries > 0) { await sleep(2000); resolve(fetchJSON(url, retries - 1)); }
      else reject(err);
    });
  });
}

const BOOKS = [
  { name: 'Genesis', api: 'Onkelos_Genesis', aramaic: 'בראשית', aramaicName: 'Bereshit', chapters: 50 },
  { name: 'Exodus', api: 'Onkelos_Exodus', aramaic: 'שמות', aramaicName: 'Shemot', chapters: 40 },
  { name: 'Leviticus', api: 'Onkelos_Leviticus', aramaic: 'ויקרא', aramaicName: 'Vayikra', chapters: 27 },
  { name: 'Numbers', api: 'Onkelos_Numbers', aramaic: 'במדבר', aramaicName: 'Bamidbar', chapters: 36 },
  { name: 'Deuteronomy', api: 'Onkelos_Deuteronomy', aramaic: 'דברים', aramaicName: 'Devarim', chapters: 34 },
];

// Strip HTML tags from Sefaria text
function stripHTML(text) {
  return text.replace(/<[^>]+>/g, '');
}

// Strip niqqud/tashkeel to get consonantal text
function stripNiqqud(text) {
  return text.replace(/[\u0591-\u05C7]/g, '');
}

// Aramaic uses the same 22 Hebrew letters — same Proto-Sinaitic pictographs
const LETTER_INFO = {
  'א': { name: 'Aleph', proto: 'Ox head', emoji: '🐂' },
  'ב': { name: 'Bet', proto: 'House', emoji: '🏠' },
  'ג': { name: 'Gimel', proto: 'Camel', emoji: '🐪' },
  'ד': { name: 'Dalet', proto: 'Door', emoji: '🚪' },
  'ה': { name: 'He', proto: 'Man calling', emoji: '🙋' },
  'ו': { name: 'Vav', proto: 'Hook/peg', emoji: '🪝' },
  'ז': { name: 'Zayin', proto: 'Weapon', emoji: '⚔️' },
  'ח': { name: 'Chet', proto: 'Courtyard', emoji: '🧱' },
  'ט': { name: 'Tet', proto: 'Basket', emoji: '☸️' },
  'י': { name: 'Yod', proto: 'Hand', emoji: '💪' },
  'כ': { name: 'Kaf', proto: 'Palm', emoji: '✋' },
  'ך': { name: 'Kaf', proto: 'Palm', emoji: '✋' },
  'ל': { name: 'Lamed', proto: 'Goad', emoji: '🪄' },
  'מ': { name: 'Mem', proto: 'Water', emoji: '🌊' },
  'ם': { name: 'Mem', proto: 'Water', emoji: '🌊' },
  'נ': { name: 'Nun', proto: 'Fish', emoji: '🐟' },
  'ן': { name: 'Nun', proto: 'Fish', emoji: '🐟' },
  'ס': { name: 'Samekh', proto: 'Support', emoji: '🏛️' },
  'ע': { name: 'Ayin', proto: 'Eye', emoji: '👁️' },
  'פ': { name: 'Pe', proto: 'Mouth', emoji: '👄' },
  'ף': { name: 'Pe', proto: 'Mouth', emoji: '👄' },
  'צ': { name: 'Tsade', proto: 'Hunt', emoji: '🏹' },
  'ץ': { name: 'Tsade', proto: 'Hunt', emoji: '🏹' },
  'ק': { name: 'Qof', proto: 'Back of head', emoji: '🧠' },
  'ר': { name: 'Resh', proto: 'Head', emoji: '🗣️' },
  'ש': { name: 'Shin', proto: 'Teeth', emoji: '🦷' },
  'ת': { name: 'Tav', proto: 'Mark/sign', emoji: '✖️' },
};

function decomposeWord(word) {
  const consonants = stripNiqqud(stripHTML(word));
  const letters = [];
  for (const ch of consonants) {
    const info = LETTER_INFO[ch];
    if (info) {
      letters.push({
        h: ch,
        p: info.emoji,
        n: info.name,
        m: info.proto,
      });
    }
  }
  return letters;
}

async function fetchChapter(apiName, chapter) {
  const baseUrl = `https://www.sefaria.org/api/v3/texts/${apiName}.${chapter}`;
  const [arResp, enResp] = await Promise.all([
    fetchJSON(baseUrl),
    fetchJSON(baseUrl + '?version=english'),
  ]);
  return { ar: arResp, en: enResp };
}

function buildChapterVerses(bookName, chapter, arData, enData) {
  const arTexts = arData.versions && arData.versions.length > 0
    ? arData.versions[0].text : [];
  const enTexts = enData.versions && enData.versions.length > 0
    ? enData.versions[0].text : [];

  return arTexts.map((arText, i) => {
    const cleanAr = stripHTML(arText || '');
    const arWords = cleanAr.split(/\s+/).filter(w => w.length > 0 && w !== ':');

    const words = arWords.map(w => ({
      ar: w,
      letters: decomposeWord(w),
    }));

    return {
      verse: i + 1,
      ref: `${bookName} ${chapter}:${i + 1}`,
      aramaic: cleanAr,
      english: enTexts[i] ? stripHTML(enTexts[i]) : '',
      words: words,
    };
  });
}

async function main() {
  fs.mkdirSync(OUT_DIR, { recursive: true });

  for (const book of BOOKS) {
    const outFile = path.join(OUT_DIR, book.name + '.json');
    if (fs.existsSync(outFile)) {
      console.log(`[SKIP] ${book.name} (already exists)`);
      continue;
    }

    console.log(`Fetching ${book.name} (${book.chapters} chapters)...`);
    const chapters = {};

    for (let ch = 1; ch <= book.chapters; ch++) {
      try {
        const { ar, en } = await fetchChapter(book.api, ch);
        const verses = buildChapterVerses(book.name, ch, ar, en);
        chapters[String(ch)] = verses;
        process.stdout.write(`  ch.${ch}/${book.chapters}\r`);
        await sleep(500); // Be nice to the API
      } catch(e) {
        console.warn(`  Warning: failed ch.${ch} of ${book.name}: ${e.message}`);
        // Try once more after a longer wait
        await sleep(3000);
        try {
          const { ar, en } = await fetchChapter(book.api, ch);
          const verses = buildChapterVerses(book.name, ch, ar, en);
          chapters[String(ch)] = verses;
        } catch(e2) {
          console.warn(`  Skipped ch.${ch} of ${book.name}: ${e2.message}`);
          chapters[String(ch)] = [];
        }
      }
    }

    const bookData = {
      name: book.name,
      aramaicName: book.aramaic,
      translitName: book.aramaicName,
      chapters: chapters,
    };

    fs.writeFileSync(outFile, JSON.stringify(bookData, null, 2));
    console.log(`  Wrote ${book.name}.json`);
  }

  // Generate aramaic_books.js
  const booksJS = `export const TARGUM_BOOKS = ${JSON.stringify(BOOKS.map(b => ({
    name: b.name,
    aramaic: b.aramaic,
    aramaicName: b.aramaicName,
    chapters: b.chapters,
  })), null, 2)};\n`;
  fs.writeFileSync(path.join(__dirname, '..', 'aramaic_books.js'), booksJS);
  console.log('Wrote aramaic_books.js');

  console.log('\nDone!');
}

main().catch(err => {
  console.error('Fatal error:', err);
  process.exit(1);
});
