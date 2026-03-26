#!/usr/bin/env python3
"""Parse the Chinese Union Version (CUV) Bible into per-book JSON files."""

import json, os, sys, urllib.request

CUV_URL = "https://raw.githubusercontent.com/MaatheusGois/bible/main/versions/zh/cuv.json"
CUV_CACHE = "/tmp/cuv.json"
DATA_DIR = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))), "data", "chinese")

CHINESE_NAMES = [
    "創世記","出埃及記","利未記","民數記","申命記","約書亞記","士師記","路得記",
    "撒母耳記上","撒母耳記下","列王紀上","列王紀下","歷代志上","歷代志下",
    "以斯拉記","尼希米記","以斯帖記","約伯記","詩篇","箴言","傳道書","雅歌",
    "以賽亞書","耶利米書","耶利米哀歌","以西結書","但以理書","何西阿書",
    "約珥書","阿摩司書","俄巴底亞書","約拿書","彌迦書","那鴻書","哈巴谷書",
    "西番雅書","哈該書","撒迦利亞書","瑪拉基書",
    "馬太福音","馬可福音","路加福音","約翰福音","使徒行傳","羅馬書",
    "哥林多前書","哥林多後書","加拉太書","以弗所書","腓立比書","歌羅西書",
    "帖撒羅尼迦前書","帖撒羅尼迦後書","提摩太前書","提摩太後書","提多書",
    "腓利門書","希伯來書","雅各書","彼得前書","彼得後書",
    "約翰壹書","約翰貳書","約翰參書","猶大書","啟示錄",
]

PUNCTUATION = set("。，、；：？！「」『』（）——…【】《》·''""．\u3000 \t\n\r.,;:?!\"'()-")

def is_cjk(ch):
    cp = ord(ch)
    return (0x4E00 <= cp <= 0x9FFF) or (0x3400 <= cp <= 0x4DBF) or (0xF900 <= cp <= 0xFAFF)

def main():
    os.makedirs(DATA_DIR, exist_ok=True)

    if not os.path.exists(CUV_CACHE):
        print(f"Downloading CUV from {CUV_URL} ...")
        req = urllib.request.Request(CUV_URL, headers={"User-Agent": "Mozilla/5.0"})
        with urllib.request.urlopen(req) as resp:
            raw = resp.read().decode("utf-8-sig")
        with open(CUV_CACHE, "w", encoding="utf-8") as f:
            f.write(raw)
    else:
        print(f"Using cached {CUV_CACHE}")

    with open(CUV_CACHE, "r", encoding="utf-8-sig") as f:
        data = json.load(f)

    print(f"Source: {len(data)} books")

    filter_book = " ".join(sys.argv[1:]) if len(sys.argv) > 1 else None
    total_b = total_ch = total_v = total_c = 0

    for i, book in enumerate(data[:66]):
        eng_name = book["name"]
        if filter_book and eng_name != filter_book:
            continue

        chi_name = CHINESE_NAMES[i] if i < len(CHINESE_NAMES) else ""
        section = "Old Testament" if i < 39 else "New Testament"
        chapters = {}
        bk_v = bk_c = 0

        for ch_idx, verse_list in enumerate(book["chapters"]):
            ch_num = str(ch_idx + 1)
            verses = []
            for v_idx, text in enumerate(verse_list):
                v_num = v_idx + 1
                # Characters: strip spaces/punctuation, keep CJK
                chars = [{"ch": c, "pinyin": "", "eng": ""} for c in text if is_cjk(c)]
                verses.append({
                    "verse": v_num,
                    "ref": f"{eng_name} {ch_num}:{v_num}",
                    "chinese": text.replace(" ", ""),
                    "english": "",
                    "chars": chars,
                })
                bk_c += len(chars)
            bk_v += len(verses)
            chapters[ch_num] = verses

        result = {"name": eng_name, "chineseName": chi_name, "section": section, "chapters": chapters}
        out = os.path.join(DATA_DIR, f"{eng_name}.json")
        with open(out, "w", encoding="utf-8") as f:
            json.dump(result, f, ensure_ascii=False)

        print(f"  {eng_name}: {len(chapters)} ch, {bk_v} v, {bk_c} chars")
        total_b += 1; total_ch += len(chapters); total_v += bk_v; total_c += bk_c

    print(f"\nTotal: {total_b} books, {total_ch} chapters, {total_v} verses, {total_c} characters")

if __name__ == "__main__":
    main()
