const {
  Document, Packer, Paragraph, TextRun, Table, TableRow, TableCell,
  HeadingLevel, AlignmentType, BorderStyle, WidthType, ShadingType,
  PageBreak, LevelFormat
} = require('docx');
const fs = require('fs');

const border = { style: BorderStyle.SINGLE, size: 1, color: "CCCCCC" };
const borders = { top: border, bottom: border, left: border, right: border };
const headerBorder = { style: BorderStyle.SINGLE, size: 1, color: "8B6914" };
const headerBorders = { top: headerBorder, bottom: headerBorder, left: headerBorder, right: headerBorder };

function cell(text, width, isHeader = false, shade = null) {
  return new TableCell({
    borders: isHeader ? headerBorders : borders,
    width: { size: width, type: WidthType.DXA },
    shading: shade ? { fill: shade, type: ShadingType.CLEAR } : undefined,
    margins: { top: 80, bottom: 80, left: 120, right: 120 },
    children: [new Paragraph({
      children: [new TextRun({ text, bold: isHeader, size: isHeader ? 22 : 20, font: "Arial" })]
    })]
  });
}

function heading(text, level) {
  return new Paragraph({
    heading: level,
    children: [new TextRun({ text, font: "Arial" })]
  });
}

function para(text, opts = {}) {
  return new Paragraph({
    children: [new TextRun({ text, font: "Arial", size: 22, ...opts })]
  });
}

function spacer() {
  return new Paragraph({ children: [new TextRun("")] });
}

// ── 22 LETTERS ──────────────────────────────────────────────────────────────
const letters = [
  ["🐂","𐤀","א","Aleph","Silent/glottal","Ox head","Strength, leader, first"],
  ["🏠","𐤁","ב","Beth","B / V","House / tent","Inside, container, family"],
  ["🐪","𐤂","ג","Gimel","G","Camel","To carry, lift, benefit"],
  ["🚪","𐤃","ד","Dalet","D","Door","Pathway, entry, to move"],
  ["👐","𐤄","ה","He","H","Window / breath","Behold! Reveal, breath of God"],
  ["🪝","𐤅","ו","Vav","V / W","Tent peg / hook","To connect, to secure, and"],
  ["⚔️","𐤆","ז","Zayin","Z","Weapon / sword","To cut, to arm, sustenance"],
  ["🧱","𐤇","ח","Chet","Ch (guttural)","Fence / inner room","Separation, protect, inner"],
  ["🐍","𐤈","ט","Tet","T","Coiled snake / basket","To surround, contain, good"],
  ["🤲","𐤉","י","Yod","Y","Closed hand / arm","Deed, work, divine action"],
  ["✋","𐤊","כ","Kaph","K / Kh","Open palm","To bend, yield, open power"],
  ["🦁","𐤋","ל","Lamed","L","Ox goad","To teach, urge, toward"],
  ["🌊","𐤌","מ","Mem","M","Water","Chaos, the masses, mighty"],
  ["🐟","𐤍","נ","Nun","N","Fish / sprout","Life, activity, heir"],
  ["🌵","𐤎","ס","Samech","S","Prop / thorn","To support, surround, hate"],
  ["👁️","𐤏","ע","Ayin","Silent (guttural)","Eye","To see, perceive, presence"],
  ["💬","𐤐","פ","Pe","P / Ph","Mouth","To speak, word, edge"],
  ["🎣","𐤑","צ","Tsade","Ts","Fish hook","To hunt, desire, righteous"],
  ["🌅","𐤒","ק","Qoph","Q","Sun on horizon / back of head","Last, behind, to follow"],
  ["👤","𐤓","ר","Resh","R","Head of a man","First, highest, beginning"],
  ["🦷","𐤔","ש","Shin","Sh / S","Teeth / flames","To consume, press, fire"],
  ["✝️","𐤕","ת","Tav","T / Th","Crossed sticks / mark","Sign, covenant, seal"],
];

// ── VOCABULARY BY ROOT COMBINATIONS ────────────────────────────────────────
const roots = [
  { pics:"🐂🏠", root:"אב", meaning:"the strong one of the house", words:[
    { heb:"אָב", trans:"av", eng:"father", memory:"The ox (strength) of the house — the strong one inside the tent" },
    { heb:"אַבְרָהָם", trans:"Avraham", eng:"Abraham", memory:"Father of many — strength lifting many houses" },
  ]},
  { pics:"🐂🦁", root:"אל", meaning:"raw strength guided with authority", words:[
    { heb:"אֵל", trans:"El", eng:"God / mighty one", memory:"Ox goad drives the ox — strength under supreme authority" },
    { heb:"אֱלֹהִים", trans:"Elohim", eng:"God (plural of majesty)", memory:"The ultimate strength that urges all creation" },
  ]},
  { pics:"🏠🤲✝️", root:"בית", meaning:"the house where God's work is sealed", words:[
    { heb:"בַּיִת", trans:"bayit", eng:"house / temple", memory:"House, working hand, covenant mark — where deeds are sealed inside" },
    { heb:"בֵּית לֶחֶם", trans:"Beit Lechem", eng:"Bethlehem", memory:"House of bread — where sustenance lives" },
  ]},
  { pics:"🚪🏠👤", root:"דבר", meaning:"the chief thing that enters the dwelling", words:[
    { heb:"דָּבָר", trans:"davar", eng:"word / thing", memory:"The chief (head) thing entering (door) the house — a word crossing the threshold" },
    { heb:"דִּבֵּר", trans:"dibber", eng:"he spoke", memory:"He sent a word through the door" },
    { heb:"מִדְבָּר", trans:"midbar", eng:"wilderness / desert", memory:"Where words go to die — empty place beyond the door" },
    { heb:"דְּבִיר", trans:"devir", eng:"Holy of Holies", memory:"Innermost room where the Chief Word dwells" },
  ]},
  { pics:"🌊✋🦁", root:"מלכ", meaning:"force that directs the masses and holds power", words:[
    { heb:"מֶלֶךְ", trans:"melek", eng:"king", memory:"Water (masses) bent by the open palm, directed by the goad" },
    { heb:"מַלְכָּה", trans:"malkah", eng:"queen", memory:"She who holds royal power" },
    { heb:"מַלְכוּת", trans:"malkuth", eng:"kingdom", memory:"The realm where the open hand rules the waters" },
    { heb:"מָלַךְ", trans:"malakh", eng:"to reign", memory:"To direct the chaos with authority" },
  ]},
  { pics:"🤲🦷👁️", root:"ישע", meaning:"a hand pressing through into open sight", words:[
    { heb:"יֵשׁוּעַ", trans:"Yeshua", eng:"Jesus / Joshua", memory:"Working hand pressing through (teeth) into open sight — rescue!" },
    { heb:"יְשׁוּעָה", trans:"yeshuah", eng:"salvation", memory:"The hand breaking through pressure into open space" },
    { heb:"יְשַׁעְיָהוּ", trans:"Yeshayahu", eng:"Isaiah", memory:"YHWH is salvation — God's hand presses through" },
  ]},
  { pics:"🦷🦁🌊", root:"שלמ", meaning:"consuming the chaos — nothing left to disturb", words:[
    { heb:"שָׁלוֹם", trans:"shalom", eng:"peace / wholeness", memory:"Teeth consume, goad directs, water calms — complete peace" },
    { heb:"שְׁלֹמֹה", trans:"Shelomoh", eng:"Solomon", memory:"The man of complete peace" },
    { heb:"יְרוּשָׁלַיִם", trans:"Yerushalayim", eng:"Jerusalem", memory:"Foundation of peace — where shalom is laid down" },
  ]},
  { pics:"👤🪝🧱", root:"רוח", meaning:"breath hooked through the boundary", words:[
    { heb:"רוּחַ", trans:"ruach", eng:"spirit / wind / breath", memory:"The head connected (hook) through the fence — breath crossing a boundary" },
    { heb:"רוּחַ אֱלֹהִים", trans:"ruach Elohim", eng:"Spirit of God", memory:"God's breath hooked through the veil — Genesis 1:2" },
  ]},
  { pics:"🐂🚪🌊", root:"אדמ", meaning:"strength entering the waters", words:[
    { heb:"אָדָם", trans:"adam", eng:"man / humanity", memory:"Ox strength enters the doorway of the waters — formed from red clay" },
    { heb:"אֲדָמָה", trans:"adamah", eng:"ground / earth", memory:"The watery red clay — where man came from" },
  ]},
  { pics:"🦁🏠", root:"לב", meaning:"what the goad drives from inside the house", words:[
    { heb:"לֵב", trans:"lev", eng:"heart", memory:"The goad inside the house — what drives from within" },
    { heb:"לֵבָב", trans:"levav", eng:"heart (emphatic)", memory:"The innermost inner drive — doubly inside" },
  ]},
  { pics:"🐟💬🦷", root:"נפש", meaning:"living breath that consumes — the whole person", words:[
    { heb:"נֶפֶשׁ", trans:"nefesh", eng:"soul / living being", memory:"Fish (life) mouth (breath) teeth (consuming) — the animated whole person" },
    { heb:"נְשָׁמָה", trans:"neshamah", eng:"breath of life", memory:"The breath that makes the fish swim upward" },
  ]},
  { pics:"🌅🚪🦷", root:"קדש", meaning:"fire at the threshold — burning at the boundary", words:[
    { heb:"קֹדֶשׁ", trans:"qodesh", eng:"holy / set apart", memory:"Sun crosses the door into fire — burning boundary between common and holy" },
    { heb:"קָדוֹשׁ", trans:"qadosh", eng:"holy one", memory:"The one who stands where the sun crosses — separate, other" },
    { heb:"מִקְדָּשׁ", trans:"miqdash", eng:"sanctuary / temple", memory:"The place where fire meets the door" },
  ]},
];

// ── BUILD DOCUMENT ───────────────────────────────────────────────────────────
const children = [];

// Title
children.push(new Paragraph({
  heading: HeadingLevel.HEADING_1,
  children: [new TextRun({ text: "Hebrew Pictorial Roots: Letter & Vocabulary Reference", font: "Arial", bold: true, size: 36 })]
}));
children.push(spacer());
children.push(para("Every Hebrew root is a three-letter pictograph that tells a story. Learn the pictures — learn the words.", { italics: true }));
children.push(spacer());

// ── SECTION 1: 22 LETTERS TABLE ─────────────────────────────────────────────
children.push(heading("The 22 Letters: Picture → Sound → Meaning", HeadingLevel.HEADING_1));
children.push(spacer());

// Table header
const colWidths = [700, 700, 700, 1200, 1400, 1800, 2860];
const headers = ["Picture","Phoenician","Hebrew","Name","Sound","Image","Core Meaning"];

const headerRow = new TableRow({
  children: headers.map((h, i) => new TableCell({
    borders: headerBorders,
    width: { size: colWidths[i], type: WidthType.DXA },
    shading: { fill: "8B6914", type: ShadingType.CLEAR },
    margins: { top: 80, bottom: 80, left: 120, right: 120 },
    children: [new Paragraph({
      children: [new TextRun({ text: h, bold: true, color: "FFFFFF", size: 20, font: "Arial" })]
    })]
  }))
});

const letterRows = letters.map((l, idx) => new TableRow({
  children: l.map((val, i) => new TableCell({
    borders,
    width: { size: colWidths[i], type: WidthType.DXA },
    shading: { fill: idx % 2 === 0 ? "FEFBF0" : "FFFFFF", type: ShadingType.CLEAR },
    margins: { top: 80, bottom: 80, left: 120, right: 120 },
    children: [new Paragraph({
      children: [new TextRun({
        text: val,
        font: i === 2 ? "Arial" : "Arial",
        size: i === 2 ? 28 : 20,
        bold: i === 2
      })]
    })]
  }))
}));

children.push(new Table({
  width: { size: 9360, type: WidthType.DXA },
  columnWidths: colWidths,
  rows: [headerRow, ...letterRows]
}));

children.push(spacer());
children.push(spacer());

// ── SECTION 2: VOCABULARY BY ROOT ───────────────────────────────────────────
children.push(new Paragraph({ children: [new PageBreak()] }));
children.push(heading("Vocabulary Through Picture Combinations", HeadingLevel.HEADING_1));
children.push(spacer());
children.push(para("Each root below shows the pictures that build it, then the words that flow from those pictures.", { italics: true }));
children.push(spacer());

roots.forEach(root => {
  // Root header
  children.push(new Paragraph({
    heading: HeadingLevel.HEADING_2,
    children: [new TextRun({ text: `${root.pics}  →  ${root.root}  →  ${root.meaning}`, font: "Arial", bold: true, size: 26 })]
  }));

  // Word table
  const wColWidths = [1400, 1400, 1400, 5160];
  const wHeaders = ["Hebrew", "Transliteration", "English", "Memory Hook (what the pictures show)"];

  const wHeaderRow = new TableRow({
    children: wHeaders.map((h, i) => new TableCell({
      borders: headerBorders,
      width: { size: wColWidths[i], type: WidthType.DXA },
      shading: { fill: "4A3728", type: ShadingType.CLEAR },
      margins: { top: 60, bottom: 60, left: 120, right: 120 },
      children: [new Paragraph({
        children: [new TextRun({ text: h, bold: true, color: "FFFFFF", size: 18, font: "Arial" })]
      })]
    }))
  });

  const wRows = root.words.map((w, idx) => new TableRow({
    children: [w.heb, w.trans, w.eng, w.memory].map((val, i) => new TableCell({
      borders,
      width: { size: wColWidths[i], type: WidthType.DXA },
      shading: { fill: idx % 2 === 0 ? "FDF6E8" : "FFFFFF", type: ShadingType.CLEAR },
      margins: { top: 60, bottom: 60, left: 120, right: 120 },
      children: [new Paragraph({
        children: [new TextRun({
          text: val,
          font: "Arial",
          size: i === 0 ? 26 : 20,
          bold: i === 0,
          italics: i === 3
        })]
      })]
    }))
  }));

  children.push(new Table({
    width: { size: 9360, type: WidthType.DXA },
    columnWidths: wColWidths,
    rows: [wHeaderRow, ...wRows]
  }));
  children.push(spacer());
});

// ── BUILD & SAVE ─────────────────────────────────────────────────────────────
const doc = new Document({
  styles: {
    default: { document: { run: { font: "Arial", size: 22 } } },
    paragraphStyles: [
      { id: "Heading1", name: "Heading 1", basedOn: "Normal", next: "Normal", quickFormat: true,
        run: { size: 32, bold: true, font: "Arial", color: "4A3728" },
        paragraph: { spacing: { before: 300, after: 150 }, outlineLevel: 0 } },
      { id: "Heading2", name: "Heading 2", basedOn: "Normal", next: "Normal", quickFormat: true,
        run: { size: 26, bold: true, font: "Arial", color: "8B6914" },
        paragraph: { spacing: { before: 240, after: 120 }, outlineLevel: 1 } },
    ]
  },
  sections: [{
    properties: {
      page: {
        size: { width: 12240, height: 15840 },
        margin: { top: 1080, right: 1080, bottom: 1080, left: 1080 }
      }
    },
    children
  }]
});

Packer.toBuffer(doc).then(buf => {
  fs.writeFileSync("/home/claude/hebrew_reference.docx", buf);
  console.log("Done");
});
