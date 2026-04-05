#!/usr/bin/env python3
"""
Generate pictograph memory narratives for top-500 Hebrew words missing them.

Uses Claude API (Sonnet) with few-shot examples from existing narratives.json.
Merges results back into narratives.json.

Usage:
    python3 tools/generate_narratives.py          # generate missing narratives
    python3 tools/generate_narratives.py --dry-run # show what would be generated
"""

import json
import os
import sys
import time

import anthropic

BASE = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
NARRATIVES_FILE = os.path.join(BASE, "narratives.json")
TOP500_FILE = os.path.join(BASE, "tools/top500_decomposed.json")

# Few-shot example words (must exist in narratives.json)
FEW_SHOT_KEYS = [
    "בָּרָא",  # create — short, concrete
    "אָדָם",  # man — concrete
    "טוֹב",   # good — abstract
    "בֵּן",   # son — short
    "מֶלֶךְ", # king — concrete
    "אוֹר",   # light — abstract
    "הָיָה",  # to be — very abstract
    "עָשָׂה",  # to do — action
    "לֹא",    # not — particle
    "נֶפֶשׁ",  # soul — abstract
]


def build_system_prompt(narratives, top500_by_heb):
    """Build system prompt with few-shot examples from existing narratives."""
    examples = ""
    for heb in FEW_SHOT_KEYS:
        if heb not in narratives:
            continue
        entry = top500_by_heb.get(heb)
        if not entry:
            continue
        eng = entry.get("def") or entry.get("eng", "?")
        examples += f"""
Word: {heb}
Meaning: {eng}
Letters: {entry['letter_summary']}
Narrative: {narratives[heb]}
"""

    return f"""You are a Hebrew pictograph memory narrative writer. Each Hebrew letter was originally a pictograph with a concrete meaning. Your job is to write a short memory narrative (1-2 sentences) that connects the pictograph meanings of a word's letters into a vivid scene or image that LANDS ON the actual meaning of the word.

Rules:
- Name each letter and its pictograph meaning in parentheses, e.g. "The house (Beth)"
- Chain the letters in order using varied connecting phrases
- End with a dash and a phrase that ties the picture to the word's actual meaning
- The narrative must help someone REMEMBER what the word means from the pictures
- Be vivid and concrete, not abstract or vague
- Keep it to 1-2 sentences plus the closing phrase after the dash
- Do NOT include the Hebrew word or transliteration in the narrative

Here are examples of excellent narratives:
{examples}
Match this style exactly. The narrative should feel like the pictures naturally lead to the meaning."""


def main():
    dry_run = "--dry-run" in sys.argv

    with open(NARRATIVES_FILE) as f:
        narratives = json.load(f)

    with open(TOP500_FILE) as f:
        top500 = json.load(f)

    top500_by_heb = {w["heb"]: w for w in top500}

    # Find words needing narratives
    missing = [w for w in top500 if w["heb"] not in narratives]
    print(f"{len(missing)} of {len(top500)} top-500 words need narratives", file=sys.stderr)

    if not missing:
        print("All top-500 words already have narratives!", file=sys.stderr)
        return

    if dry_run:
        print("\nWords needing narratives:", file=sys.stderr)
        for w in missing:
            eng = w.get("def") or w.get("eng", "?")
            print(f"  #{w['rank']:3d} {w['heb']}  ({eng})  {w['letter_summary']}", file=sys.stderr)
        return

    system_prompt = build_system_prompt(narratives, top500_by_heb)
    client = anthropic.Anthropic()

    BATCH_SIZE = 10
    added = 0
    errors = 0

    for batch_start in range(0, len(missing), BATCH_SIZE):
        batch = missing[batch_start:batch_start + BATCH_SIZE]

        words_text = ""
        for w in batch:
            eng = w.get("def") or w.get("eng", "?")
            words_text += f"""
---
Word: {w['heb']}
Meaning: {eng}
Usage: {w.get('usage', '')}
Letters: {w['letter_summary']}
"""

        user_prompt = f"""Write a pictograph memory narrative for each of these Hebrew words. Return ONLY a JSON object mapping each Hebrew word to its narrative string. No markdown fences, no explanation — just the JSON.

{words_text}"""

        label = f"{batch_start + 1}-{min(batch_start + BATCH_SIZE, len(missing))}/{len(missing)}"
        print(f"[{label}] Generating...", end="", flush=True, file=sys.stderr)

        try:
            message = client.messages.create(
                model="claude-sonnet-4-20250514",
                max_tokens=4096,
                system=system_prompt,
                messages=[{"role": "user", "content": user_prompt}],
            )

            text = message.content[0].text.strip()
            # Strip markdown fences if present
            if text.startswith("```"):
                text = text.split("\n", 1)[1]
                if text.endswith("```"):
                    text = text[:text.rfind("```")]

            batch_result = json.loads(text)

            for heb, narrative in batch_result.items():
                narratives[heb] = narrative
                added += 1

            tokens = f"{message.usage.input_tokens}+{message.usage.output_tokens}"
            print(f" done ({tokens} tokens, {len(batch_result)} words)", file=sys.stderr)

        except anthropic.RateLimitError:
            print(" rate limited, waiting 60s...", file=sys.stderr)
            time.sleep(60)
            continue
        except json.JSONDecodeError as e:
            print(f" JSON parse error: {e}", file=sys.stderr)
            print(f"  Response: {text[:200]}...", file=sys.stderr)
            errors += 1
            continue
        except Exception as e:
            print(f" ERROR: {e}", file=sys.stderr)
            errors += 1
            continue

        time.sleep(1)

    # Save updated narratives
    with open(NARRATIVES_FILE, "w") as f:
        json.dump(narratives, f, ensure_ascii=False, indent=2)

    print(f"\nDone. Added {added} narratives, {errors} errors.", file=sys.stderr)
    print(f"Total narratives: {len(narratives)}", file=sys.stderr)


if __name__ == "__main__":
    main()
