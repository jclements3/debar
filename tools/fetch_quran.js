#!/usr/bin/env node
// Fetch all 114 surahs from alquran.cloud API and write them as JSON data files
// matching the debar project structure.
//
// Usage: node tools/fetch_quran.js
//
// Output: data/quran/<SurahName>.json per surah

const fs = require('fs');
const path = require('path');
const https = require('https');

const OUT_DIR = path.join(__dirname, '..', 'data', 'quran');

function sleep(ms) { return new Promise(r => setTimeout(r, ms)); }

function fetchJSON(url, retries = 3) {
  return new Promise((resolve, reject) => {
    https.get(url, res => {
      let body = '';
      res.on('data', chunk => body += chunk);
      res.on('end', async () => {
        try {
          const data = JSON.parse(body);
          if (data.code === 429 || res.statusCode === 429) {
            if (retries > 0) {
              console.log(`  Rate limited, waiting 3s and retrying... (${retries} left)`);
              await sleep(3000);
              resolve(fetchJSON(url, retries - 1));
            } else {
              reject(new Error('Rate limited after retries: ' + url));
            }
          } else {
            resolve(data);
          }
        }
        catch(e) {
          if (retries > 0) {
            await sleep(2000);
            resolve(fetchJSON(url, retries - 1));
          } else {
            reject(new Error('JSON parse error: ' + e.message));
          }
        }
      });
      res.on('error', reject);
    }).on('error', async (err) => {
      if (retries > 0) {
        await sleep(2000);
        resolve(fetchJSON(url, retries - 1));
      } else {
        reject(err);
      }
    });
  });
}

// Arabic letter -> Proto-Sinaitic pictograph mapping
const ARABIC_LETTERS = {
  'ا': { name: 'Alif', proto: 'Ox head', emoji: '\uD83D\uDC02', skeleton: 'ا' },
  'أ': { name: 'Alif', proto: 'Ox head', emoji: '\uD83D\uDC02', skeleton: 'ا' },
  'إ': { name: 'Alif', proto: 'Ox head', emoji: '\uD83D\uDC02', skeleton: 'ا' },
  'آ': { name: 'Alif', proto: 'Ox head', emoji: '\uD83D\uDC02', skeleton: 'ا' },
  'ٱ': { name: 'Alif', proto: 'Ox head', emoji: '\uD83D\uDC02', skeleton: 'ا' },
  'ب': { name: 'Ba', proto: 'House', emoji: '\uD83C\uDFE0', skeleton: '\u066E' },
  'ت': { name: 'Ta', proto: 'Mark/sign', emoji: '\u2716\uFE0F', skeleton: '\u066E' },
  'ث': { name: 'Tha', proto: 'Mark/sign', emoji: '\u2716\uFE0F', skeleton: '\u066E' },
  'ج': { name: 'Jim', proto: 'Camel', emoji: '\uD83D\uDC2A', skeleton: 'ح' },
  'ح': { name: 'Ha', proto: 'Courtyard/fence', emoji: '\uD83E\uDDF1', skeleton: 'ح' },
  'خ': { name: 'Kha', proto: 'Courtyard/fence', emoji: '\uD83E\uDDF1', skeleton: 'ح' },
  'د': { name: 'Dal', proto: 'Door', emoji: '\uD83D\uDEAA', skeleton: 'د' },
  'ذ': { name: 'Dhal', proto: 'Door', emoji: '\uD83D\uDEAA', skeleton: 'د' },
  'ر': { name: 'Ra', proto: 'Head', emoji: '\uD83D\uDDE3\uFE0F', skeleton: 'ر' },
  'ز': { name: 'Zay', proto: 'Weapon/sword', emoji: '\u2694\uFE0F', skeleton: 'ر' },
  'س': { name: 'Sin', proto: 'Teeth', emoji: '\uD83E\uDDB7', skeleton: 'س' },
  'ش': { name: 'Shin', proto: 'Teeth', emoji: '\uD83E\uDDB7', skeleton: 'س' },
  'ص': { name: 'Sad', proto: 'Plant/papyrus', emoji: '\uD83C\uDF3F', skeleton: 'ص' },
  'ض': { name: 'Dad', proto: 'Plant/papyrus', emoji: '\uD83C\uDF3F', skeleton: 'ص' },
  'ط': { name: 'Taa', proto: 'Basket/wheel', emoji: '\u2638\uFE0F', skeleton: 'ط' },
  'ظ': { name: 'Dhaa', proto: 'Basket/wheel', emoji: '\u2638\uFE0F', skeleton: 'ط' },
  'ع': { name: 'Ayn', proto: 'Eye', emoji: '\uD83D\uDC41\uFE0F', skeleton: 'ع' },
  'غ': { name: 'Ghayn', proto: 'Eye', emoji: '\uD83D\uDC41\uFE0F', skeleton: 'ع' },
  'ف': { name: 'Fa', proto: 'Mouth', emoji: '\uD83D\uDC44', skeleton: '\u06A1' },
  'ق': { name: 'Qaf', proto: 'Back of head', emoji: '\uD83E\uDDE0', skeleton: '\u06A1' },
  'ك': { name: 'Kaf', proto: 'Palm of hand', emoji: '\u270B', skeleton: 'ك' },
  'ل': { name: 'Lam', proto: 'Goad/staff', emoji: '\uD83E\uDE84', skeleton: 'ل' },
  'م': { name: 'Mim', proto: 'Water', emoji: '\uD83C\uDF0A', skeleton: 'م' },
  'ن': { name: 'Nun', proto: 'Fish/serpent', emoji: '\uD83D\uDC1F', skeleton: '\u066E' },
  'ه': { name: 'Ha', proto: 'Man calling', emoji: '\uD83D\uDE4B', skeleton: 'ه' },
  'ة': { name: 'Ta Marbuta', proto: 'Mark/sign', emoji: '\u2716\uFE0F', skeleton: 'ه' },
  'و': { name: 'Waw', proto: 'Hook/peg', emoji: '\uD83E\uDE9D', skeleton: 'و' },
  'ي': { name: 'Ya', proto: 'Arm/hand', emoji: '\uD83D\uDCAA', skeleton: '\u066E' },
  'ى': { name: 'Alif Maqsura', proto: 'Arm/hand', emoji: '\uD83D\uDCAA', skeleton: '\u066E' },
  'ئ': { name: 'Ya', proto: 'Arm/hand', emoji: '\uD83D\uDCAA', skeleton: '\u066E' },
  'ؤ': { name: 'Waw', proto: 'Hook/peg', emoji: '\uD83E\uDE9D', skeleton: 'و' },
};

// Strip tashkeel (vowel diacritics) from Arabic text
function stripTashkeel(text) {
  return text.replace(/[\u064B-\u065F\u0670\u06D6-\u06ED]/g, '');
}

// Decompose an Arabic word into its root letters with pictograph data
function decomposeWord(word) {
  const clean = stripTashkeel(word);
  const letters = [];
  for (const ch of clean) {
    const info = ARABIC_LETTERS[ch];
    if (info) {
      letters.push({
        ar: ch,
        p: info.emoji,
        name: info.name,
        proto: info.proto,
        skeleton: info.skeleton,
      });
    }
  }
  return letters;
}

// Surah metadata (English names, Arabic names, revelation type)
async function fetchSurahList() {
  const resp = await fetchJSON('https://api.alquran.cloud/v1/surah');
  if (resp.code !== 200) throw new Error('API error fetching surah list');
  return resp.data;
}

async function fetchSurah(num) {
  // Fetch three editions in parallel: simple-clean (no vowels), English translation, transliteration
  const [arResp, enResp, trResp] = await Promise.all([
    fetchJSON(`https://api.alquran.cloud/v1/surah/${num}/quran-simple-clean`),
    fetchJSON(`https://api.alquran.cloud/v1/surah/${num}/en.sahih`),
    fetchJSON(`https://api.alquran.cloud/v1/surah/${num}/en.transliteration`),
  ]);
  if (arResp.code !== 200) throw new Error(`API error for surah ${num} arabic`);
  if (enResp.code !== 200) throw new Error(`API error for surah ${num} english`);
  return { ar: arResp.data, en: enResp.data, tr: trResp.data };
}

function buildSurahJSON(surahMeta, arData, enData, trData) {
  const ayahs = arData.ayahs.map((arAyah, i) => {
    const enAyah = enData.ayahs[i];
    const trAyah = trData.ayahs[i];

    // Split Arabic text into words
    const arWords = arAyah.text.split(/\s+/).filter(w => w.length > 0);
    const words = arWords.map(w => {
      const letters = decomposeWord(w);
      return {
        ar: w,
        letters: letters,
      };
    });

    return {
      verse: arAyah.numberInSurah,
      ref: `${surahMeta.englishName} ${arAyah.numberInSurah}`,
      arabic: arAyah.text,
      english: enAyah ? enAyah.text : '',
      transliteration: trAyah ? trAyah.text : '',
      words: words,
    };
  });

  return {
    number: surahMeta.number,
    name: surahMeta.englishName,
    arabicName: surahMeta.name,
    englishNameTranslation: surahMeta.englishNameTranslation,
    revelationType: surahMeta.revelationType,
    ayahCount: surahMeta.numberOfAyahs,
    chapters: { "1": ayahs },
  };
}

async function main() {
  fs.mkdirSync(OUT_DIR, { recursive: true });

  console.log('Fetching surah list...');
  const surahList = await fetchSurahList();
  console.log(`Found ${surahList.length} surahs`);

  // Also generate quran_books.js
  const booksEntries = surahList.map(s => ({
    number: s.number,
    name: s.englishName,
    arabicName: s.name,
    translation: s.englishNameTranslation,
    ayahs: s.numberOfAyahs,
    type: s.revelationType,
  }));

  const booksJS = `export const QURAN_SURAHS = ${JSON.stringify(booksEntries, null, 2)};\n`;
  fs.writeFileSync(path.join(__dirname, '..', 'quran_books.js'), booksJS);
  console.log('Wrote quran_books.js');

  // Fetch surahs in batches of 3, with delay between batches
  for (let i = 0; i < surahList.length; i += 3) {
    const batch = surahList.slice(i, i + 3);

    // Skip already-fetched surahs
    const toFetch = batch.filter(s => {
      const filename = s.englishName.replace(/[^a-zA-Z0-9-]/g, '-') + '.json';
      return !fs.existsSync(path.join(OUT_DIR, filename));
    });
    if (toFetch.length === 0) {
      for (const s of batch) console.log(`  [${s.number}/114] ${s.englishName} (cached)`);
      continue;
    }

    const results = await Promise.all(toFetch.map(async s => {
      const { ar, en, tr } = await fetchSurah(s.number);
      return { meta: s, ar, en, tr };
    }));
    await sleep(1000);

    for (const { meta, ar, en, tr } of results) {
      const data = buildSurahJSON(meta, ar, en, tr);
      const filename = meta.englishName.replace(/[^a-zA-Z0-9-]/g, '-') + '.json';
      fs.writeFileSync(path.join(OUT_DIR, filename), JSON.stringify(data, null, 2));
      console.log(`  [${meta.number}/114] ${meta.englishName} -> ${filename}`);
    }
  }

  console.log('\nDone! All surahs written to data/quran/');
}

main().catch(err => {
  console.error('Fatal error:', err);
  process.exit(1);
});
