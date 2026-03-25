const DB_NAME = "debar";
const DB_VERSION = 1;

function wrap(req) {
  return new Promise((resolve, reject) => {
    req.onsuccess = () => resolve(req.result);
    req.onerror = () => reject(req.error);
  });
}

export async function openDB() {
  return new Promise((resolve, reject) => {
    const req = indexedDB.open(DB_NAME, DB_VERSION);
    req.onupgradeneeded = () => {
      const db = req.result;
      if (!db.objectStoreNames.contains("books")) {
        db.createObjectStore("books", { keyPath: "name" });
      }
      if (!db.objectStoreNames.contains("verses")) {
        db.createObjectStore("verses", { keyPath: ["book", "chapter", "verse"] });
      }
      if (!db.objectStoreNames.contains("narratives")) {
        db.createObjectStore("narratives", { keyPath: "heb" });
      }
    };
    req.onsuccess = () => resolve(req.result);
    req.onerror = () => reject(req.error);
  });
}

export async function putBook(db, book) {
  const tx = db.transaction("books", "readwrite");
  tx.objectStore("books").put(book);
  return new Promise((resolve, reject) => {
    tx.oncomplete = () => resolve();
    tx.onerror = () => reject(tx.error);
  });
}

export async function getBook(db, name) {
  return wrap(db.transaction("books").objectStore("books").get(name));
}

export async function getAllBooks(db) {
  return wrap(db.transaction("books").objectStore("books").getAll());
}

export async function putVerses(db, verses) {
  const tx = db.transaction("verses", "readwrite");
  const store = tx.objectStore("verses");
  for (const v of verses) {
    store.put(v);
  }
  return new Promise((resolve, reject) => {
    tx.oncomplete = () => resolve();
    tx.onerror = () => reject(tx.error);
  });
}

export async function getChapter(db, book, chapter) {
  return new Promise((resolve, reject) => {
    const tx = db.transaction("verses");
    const store = tx.objectStore("verses");
    const results = [];
    const req = store.openCursor();
    req.onsuccess = () => {
      const cursor = req.result;
      if (cursor) {
        const v = cursor.value;
        if (v.book === book && v.chapter === chapter) {
          results.push(v);
        }
        cursor.continue();
      } else {
        results.sort((a, b) => a.verse - b.verse);
        resolve(results);
      }
    };
    req.onerror = () => reject(req.error);
  });
}

export async function putNarrative(db, heb, narrative, source) {
  const tx = db.transaction("narratives", "readwrite");
  tx.objectStore("narratives").put({ heb, narrative, source });
  return new Promise((resolve, reject) => {
    tx.oncomplete = () => resolve();
    tx.onerror = () => reject(tx.error);
  });
}

export async function getNarrative(db, heb) {
  return wrap(db.transaction("narratives").objectStore("narratives").get(heb));
}

export async function importBookJSON(db, bookData) {
  const { name, hebrewName, chapters } = bookData;
  const totalChapters = Object.keys(chapters).length;
  await putBook(db, { name, hebrewName, totalChapters });

  const verses = [];
  for (const [ch, chVerses] of Object.entries(chapters)) {
    const chNum = Number(ch);
    for (const v of chVerses) {
      verses.push({ ...v, book: name, chapter: chNum });
    }
  }
  await putVerses(db, verses);
}

export async function importNarrativesJSON(db, narrativesObj) {
  const tx = db.transaction("narratives", "readwrite");
  const store = tx.objectStore("narratives");
  for (const [heb, val] of Object.entries(narrativesObj)) {
    const record = typeof val === "string"
      ? { heb, narrative: val, source: "manual" }
      : { heb, narrative: val.narrative, source: val.source || "manual" };
    store.put(record);
  }
  return new Promise((resolve, reject) => {
    tx.oncomplete = () => resolve();
    tx.onerror = () => reject(tx.error);
  });
}
