#!/usr/bin/env node
// Build a cross-language root comparison by reducing every word
// in the Hebrew Torah, Aramaic Targum, and Quran to its Proto-Sinaitic
// pictograph sequence, then grouping by picture pattern to find cognates.
//
// Usage: node tools/build_roots.js
// Output: data/roots_comparison.json

const fs = require('fs');
const path = require('path');

// ── Proto-Sinaitic pictograph normalization ────────────────────────────
// Every Hebrew, Aramaic, and Arabic consonant maps to the same
// Proto-Sinaitic ancestor picture. This table normalizes all three
// scripts to a common pictograph alphabet.

const PROTO = {
  // name, emoji, description
  'Ox':       { e: '\uD83D\uDC02', d: 'Strength, leader, first' },
  'House':    { e: '\uD83C\uDFE0', d: 'Family, inside, shelter' },
  'Camel':    { e: '\uD83D\uDC2A', d: 'Carry, journey, lift' },
  'Door':     { e: '\uD83D\uDEAA', d: 'Entry, path, move' },
  'Window':   { e: '\uD83D\uDE4B', d: 'Behold, reveal, breath' },
  'Hook':     { e: '\uD83E\uDE9D', d: 'Connect, add, secure' },
  'Weapon':   { e: '\u2694\uFE0F', d: 'Cut, nourish, plough' },
  'Wall':     { e: '\uD83E\uDDF1', d: 'Separate, protect, outside' },
  'Basket':   { e: '\u2638\uFE0F', d: 'Surround, contain, mud' },
  'Hand':     { e: '\uD83D\uDCAA', d: 'Work, make, throw' },
  'Palm':     { e: '\u270B',       d: 'Open, tame, cover' },
  'Goad':     { e: '\uD83E\uDE84', d: 'Teach, yoke, toward' },
  'Water':    { e: '\uD83C\uDF0A', d: 'Chaos, mighty, blood' },
  'Fish':     { e: '\uD83D\uDC1F', d: 'Continue, heir, life' },
  'Prop':     { e: '\uD83C\uDFDB\uFE0F', d: 'Support, twist, turn' },
  'Eye':      { e: '\uD83D\uDC41\uFE0F', d: 'See, know, experience' },
  'Mouth':    { e: '\uD83D\uDC44', d: 'Speak, blow, edge' },
  'Hunt':     { e: '\uD83C\uDFF9', d: 'Catch, desire, wait' },
  'Horizon':  { e: '\uD83E\uDDE0', d: 'Behind, last, least' },
  'Head':     { e: '\uD83D\uDDE3\uFE0F', d: 'First, top, beginning' },
  'Teeth':    { e: '\uD83E\uDDB7', d: 'Consume, destroy, sharp' },
  'Mark':     { e: '\u2716\uFE0F', d: 'Sign, covenant, seal' },
};

// Hebrew/Aramaic consonant -> Proto-Sinaitic name
const HEB_MAP = {
  '\u05D0': 'Ox',      // א
  '\u05D1': 'House',   // ב
  '\u05D2': 'Camel',   // ג
  '\u05D3': 'Door',    // ד
  '\u05D4': 'Window',  // ה
  '\u05D5': 'Hook',    // ו
  '\u05D6': 'Weapon',  // ז
  '\u05D7': 'Wall',    // ח
  '\u05D8': 'Basket',  // ט
  '\u05D9': 'Hand',    // י
  '\u05DB': 'Palm',    // כ
  '\u05DA': 'Palm',    // ך
  '\u05DC': 'Goad',    // ל
  '\u05DE': 'Water',   // מ
  '\u05DD': 'Water',   // ם
  '\u05E0': 'Fish',    // נ
  '\u05DF': 'Fish',    // ן
  '\u05E1': 'Prop',    // ס
  '\u05E2': 'Eye',     // ע
  '\u05E4': 'Mouth',   // פ
  '\u05E3': 'Mouth',   // ף
  '\u05E6': 'Hunt',    // צ
  '\u05E5': 'Hunt',    // ץ
  '\u05E7': 'Horizon', // ק
  '\u05E8': 'Head',    // ר
  '\u05E9': 'Teeth',   // ש
  '\u05EA': 'Mark',    // ת
};

// Arabic consonant -> Proto-Sinaitic name
const AR_MAP = {
  '\u0627': 'Ox',      // ا
  '\u0623': 'Ox',      // أ
  '\u0625': 'Ox',      // إ
  '\u0622': 'Ox',      // آ
  '\u0671': 'Ox',      // ٱ
  '\u0628': 'House',   // ب
  '\u062A': 'Mark',    // ت  (cognate of tav)
  '\u062B': 'Mark',    // ث  (variant of tav)
  '\u062C': 'Camel',   // ج
  '\u062D': 'Wall',    // ح
  '\u062E': 'Wall',    // خ
  '\u062F': 'Door',    // د
  '\u0630': 'Door',    // ذ
  '\u0631': 'Head',    // ر
  '\u0632': 'Weapon',  // ز
  '\u0633': 'Teeth',   // س  (sin = shin)
  '\u0634': 'Teeth',   // ش
  '\u0635': 'Hunt',    // ص
  '\u0636': 'Hunt',    // ض
  '\u0637': 'Basket',  // ط
  '\u0638': 'Basket',  // ظ
  '\u0639': 'Eye',     // ع
  '\u063A': 'Eye',     // غ
  '\u0641': 'Mouth',   // ف
  '\u0642': 'Horizon', // ق
  '\u0643': 'Palm',    // ك
  '\u0644': 'Goad',    // ل
  '\u0645': 'Water',   // م
  '\u0646': 'Fish',    // ن
  '\u0647': 'Window',  // ه
  '\u0629': 'Window',  // ة  (ta marbuta, cognate of he)
  '\u0648': 'Hook',    // و
  '\u0624': 'Hook',    // ؤ
  '\u064A': 'Hand',    // ي
  '\u0649': 'Hand',    // ى
  '\u0626': 'Hand',    // ئ
};

// ── Text stripping ────────────────────────────────────────────────────

function stripHebNiqqud(text) {
  return text.replace(/[\u0591-\u05C7]/g, '');
}

function stripArTashkeel(text) {
  return text.replace(/[\u064B-\u065F\u0670\u06D6-\u06ED\uFEFF]/g, '');
}

// Common Hebrew/Aramaic prefixes (single-letter function words)
const HEB_PREFIXES = new Set(['\u05D1', '\u05D4', '\u05D5', '\u05DC', '\u05DE', '\u05DB', '\u05E9']);
// Common Hebrew suffixes
const HEB_SUFFIXES = ['\u05D9\u05DD', '\u05D5\u05EA', '\u05D9\u05DF'];

function extractHebRoot(word) {
  let consonants = stripHebNiqqud(word);
  // Get only Hebrew consonants
  let chars = [];
  for (const ch of consonants) {
    if (HEB_MAP[ch]) chars.push(ch);
  }
  // Strip single-char prefix if word is long enough
  if (chars.length > 3 && HEB_PREFIXES.has(chars[0])) {
    chars = chars.slice(1);
  }
  // Strip the definite article הַ if followed by doubled consonant
  if (chars.length > 3 && chars[0] === '\u05D4') {
    chars = chars.slice(1);
  }
  // Strip common suffixes
  if (chars.length > 3) {
    const last2 = chars.slice(-2).join('');
    if (HEB_SUFFIXES.includes(last2)) {
      chars = chars.slice(0, -2);
    }
  }
  // Strip final ה or ם as suffix markers
  if (chars.length > 3) {
    const last = chars[chars.length - 1];
    if (last === '\u05D4' || last === '\u05DD' || last === '\u05DF') {
      chars = chars.slice(0, -1);
    }
  }
  return chars;
}

// Common Arabic prefixes
const AR_PREFIXES_2 = ['\u0627\u0644']; // ال (al-)
const AR_PREFIXES_1 = new Set(['\u0628', '\u0641', '\u0648', '\u0644', '\u0643']); // ب ف و ل ك

function extractArRoot(word) {
  let consonants = stripArTashkeel(word);
  let chars = [];
  for (const ch of consonants) {
    if (AR_MAP[ch]) chars.push(ch);
  }
  // Strip ال prefix
  if (chars.length > 3 && chars[0] === '\u0627' && chars[1] === '\u0644') {
    chars = chars.slice(2);
  }
  // Strip single prefix
  if (chars.length > 3 && AR_PREFIXES_1.has(chars[0])) {
    chars = chars.slice(1);
  }
  // Strip common suffixes: ون ين ات ة
  if (chars.length > 3) {
    const last = chars[chars.length - 1];
    if (last === '\u0629' || last === '\u0627') { // ة ا
      chars = chars.slice(0, -1);
    }
    const last2 = chars.slice(-2).join('');
    if (['\u0648\u0646', '\u064A\u0646', '\u0627\u062A'].includes(last2)) {
      chars = chars.slice(0, -2);
    }
  }
  return chars;
}

// Convert consonant array to pictograph key
function toPictoKey(chars, map) {
  const pics = [];
  for (const ch of chars) {
    const p = map[ch];
    if (p) pics.push(p);
  }
  return pics;
}

// ── Corpus reading ────────────────────────────────────────────────────

function readHebrewCorpus() {
  const dataDir = path.join(__dirname, '..', 'data');
  const words = new Map(); // consonants -> { root, count, examples: [{word, eng, ref}] }

  const TORAH = ['Genesis', 'Exodus', 'Leviticus', 'Numbers', 'Deuteronomy'];
  for (const book of TORAH) {
    const file = path.join(dataDir, book + '.json');
    if (!fs.existsSync(file)) continue;
    const data = JSON.parse(fs.readFileSync(file, 'utf8'));
    for (const [ch, verses] of Object.entries(data.chapters)) {
      for (const v of verses) {
        for (const w of v.words) {
          const root = extractHebRoot(w.heb);
          if (root.length < 2 || root.length > 4) continue;
          const key = root.join('');
          if (!words.has(key)) {
            words.set(key, { root, count: 0, examples: [], lang: 'hebrew' });
          }
          const entry = words.get(key);
          entry.count++;
          if (entry.examples.length < 3) {
            entry.examples.push({ word: w.heb, eng: w.eng || '', ref: v.ref });
          }
        }
      }
    }
  }
  return words;
}

function readAramaicCorpus() {
  const dataDir = path.join(__dirname, '..', 'data', 'aramaic');
  const words = new Map();

  const BOOKS = ['Genesis', 'Exodus', 'Leviticus', 'Numbers', 'Deuteronomy'];
  for (const book of BOOKS) {
    const file = path.join(dataDir, book + '.json');
    if (!fs.existsSync(file)) continue;
    const data = JSON.parse(fs.readFileSync(file, 'utf8'));
    for (const [ch, verses] of Object.entries(data.chapters)) {
      for (const v of verses) {
        for (const w of v.words) {
          const root = extractHebRoot(w.ar); // Aramaic uses same script
          if (root.length < 2 || root.length > 4) continue;
          const key = root.join('');
          if (!words.has(key)) {
            words.set(key, { root, count: 0, examples: [], lang: 'aramaic' });
          }
          const entry = words.get(key);
          entry.count++;
          if (entry.examples.length < 3) {
            entry.examples.push({ word: w.ar, eng: '', ref: v.ref });
          }
        }
      }
    }
  }
  return words;
}

function readArabicCorpus() {
  const dataDir = path.join(__dirname, '..', 'data', 'quran');
  const words = new Map();
  const files = fs.readdirSync(dataDir).filter(f => f.endsWith('.json'));

  for (const file of files) {
    const data = JSON.parse(fs.readFileSync(path.join(dataDir, file), 'utf8'));
    const ayahs = data.chapters['1'] || [];
    for (const v of ayahs) {
      for (const w of v.words) {
        const root = extractArRoot(w.ar);
        if (root.length < 2 || root.length > 4) continue;
        const key = root.join('');
        if (!words.has(key)) {
          words.set(key, { root, count: 0, examples: [], lang: 'arabic' });
        }
        const entry = words.get(key);
        entry.count++;
        if (entry.examples.length < 3) {
          entry.examples.push({ word: w.ar, eng: '', ref: v.ref });
        }
      }
    }
  }
  return words;
}

// ── Build comparison ──────────────────────────────────────────────────

function main() {
  console.log('Reading Hebrew Torah...');
  const hebrew = readHebrewCorpus();
  console.log(`  ${hebrew.size} unique Hebrew roots`);

  console.log('Reading Aramaic Targum...');
  const aramaic = readAramaicCorpus();
  console.log(`  ${aramaic.size} unique Aramaic roots`);

  console.log('Reading Arabic Quran...');
  const arabic = readArabicCorpus();
  console.log(`  ${arabic.size} unique Arabic roots`);

  // Normalize all roots to pictograph keys
  const pictoGroups = new Map(); // pictoKey -> { pics, hebrew: [], aramaic: [], arabic: [] }

  function addToGroup(wordsMap, map, langName) {
    for (const [key, entry] of wordsMap) {
      const pics = toPictoKey(entry.root, map);
      if (pics.length < 2) continue;
      const picKey = pics.join('+');

      if (!pictoGroups.has(picKey)) {
        pictoGroups.set(picKey, {
          pics: pics,
          emojis: pics.map(p => PROTO[p].e),
          meanings: pics.map(p => PROTO[p].d),
          hebrew: [],
          aramaic: [],
          arabic: [],
        });
      }
      const group = pictoGroups.get(picKey);
      group[langName].push({
        root: entry.root.join(''),
        count: entry.count,
        examples: entry.examples,
      });
    }
  }

  addToGroup(hebrew, HEB_MAP, 'hebrew');
  addToGroup(aramaic, HEB_MAP, 'aramaic');
  addToGroup(arabic, AR_MAP, 'arabic');

  // Filter to groups that have entries in at least 2 languages
  const crossLingual = [];
  for (const [picKey, group] of pictoGroups) {
    const langCount = (group.hebrew.length > 0 ? 1 : 0) +
                      (group.aramaic.length > 0 ? 1 : 0) +
                      (group.arabic.length > 0 ? 1 : 0);
    if (langCount >= 2) {
      // Sort each language's entries by frequency
      group.hebrew.sort((a, b) => b.count - a.count);
      group.aramaic.sort((a, b) => b.count - a.count);
      group.arabic.sort((a, b) => b.count - a.count);
      // Total frequency across all languages
      const totalFreq = group.hebrew.reduce((s, e) => s + e.count, 0) +
                        group.aramaic.reduce((s, e) => s + e.count, 0) +
                        group.arabic.reduce((s, e) => s + e.count, 0);
      crossLingual.push({
        key: picKey,
        ...group,
        totalFreq,
        langCount,
      });
    }
  }

  // Sort by number of languages matched (3 > 2), then by total frequency
  crossLingual.sort((a, b) => {
    if (b.langCount !== a.langCount) return b.langCount - a.langCount;
    return b.totalFreq - a.totalFreq;
  });

  // Also collect single-language groups for stats
  let totalGroups = pictoGroups.size;
  let triLingual = crossLingual.filter(g => g.langCount === 3).length;
  let biLingual = crossLingual.filter(g => g.langCount === 2).length;

  const output = {
    stats: {
      totalPictoGroups: totalGroups,
      crossLingual: crossLingual.length,
      triLingual,
      biLingual,
      hebrewRoots: hebrew.size,
      aramaicRoots: aramaic.size,
      arabicRoots: arabic.size,
    },
    proto: PROTO,
    groups: crossLingual,
  };

  const outFile = path.join(__dirname, '..', 'data', 'roots_comparison.json');
  fs.writeFileSync(outFile, JSON.stringify(output, null, 2));
  console.log(`\nWrote ${outFile}`);
  console.log(`Stats: ${totalGroups} total pictograph groups`);
  console.log(`  ${triLingual} tri-lingual (Hebrew + Aramaic + Arabic)`);
  console.log(`  ${biLingual} bi-lingual`);
  console.log(`\nTop 10 tri-lingual matches:`);

  const top = crossLingual.filter(g => g.langCount === 3).slice(0, 10);
  for (const g of top) {
    const pics = g.emojis.join(' ');
    const heb = g.hebrew[0] ? g.hebrew[0].root : '-';
    const arm = g.aramaic[0] ? g.aramaic[0].root : '-';
    const arb = g.arabic[0] ? g.arabic[0].root : '-';
    const eng = g.hebrew[0]?.examples[0]?.eng || '';
    console.log(`  ${pics}  Heb:${heb}  Arm:${arm}  Ar:${arb}  "${eng}"`);
  }
}

main();
