#!/usr/bin/env python3
"""
Generate pictograph memory narratives for top-500 Greek words.

Greek letters descend from Phoenician/Proto-Sinaitic pictographs — the same
ancestors as Hebrew letters. This script generates vivid mnemonic narratives
connecting each Greek word's letter-pictures to its meaning.

Uses Claude API (Sonnet) with few-shot examples. Merges results into
greek_narratives.json.

Usage:
    python3 tools/generate_greek_narratives.py          # generate missing narratives
    python3 tools/generate_greek_narratives.py --dry-run # show what would be generated
"""

import json
import os
import sys
import time

import anthropic

BASE = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
NARRATIVES_FILE = os.path.join(BASE, "greek_narratives.json")
TOP500_FILE = os.path.join(BASE, "tools", "greek_top500_decomposed.json")

# Seed examples — we'll generate these first manually in the system prompt
SEED_NARRATIVES = {
    "θεός": "The coiling serpent (Theta) beholds through a window (Epsilon), watched by the eye (Omicron), consumed by teeth (Sigma) — the one who surrounds all, reveals himself, sees everything, and devours chaos: God.",
    "λόγος": "The goad (Lambda) drives toward the eye (Omicron), lifting the camel (Gamma) to the eye (Omicron) consumed by teeth (Sigma) — the word that directs, carries meaning to sight, and devours confusion: reason, message.",
    "πίστις": "The mouth (Pi) works by hand (Iota), consumed by teeth (Sigma) marked by covenant (Tau), hands working (Iota) through teeth (Sigma) — speech sealed by a sign, labored faithfully to the end: trust, belief.",
    "κύριος": "The open palm (Kappa) hooks (Upsilon) to the head (Rho), hands working (Iota) through the eye (Omicron) consumed by teeth (Sigma) — the yielded hand of the chief who sees and devours all opposition: lord, master.",
    "ἀγάπη": "The ox-strength (Alpha) lifts the camel (Gamma) to ox-strength again (Alpha), speaking (Pi) through the fence (Eta) — primal power that carries, speaks, and encloses: love.",
    "πνεῦμα": "The mouth (Pi) of the fish (Nu), breathing through a window (Epsilon), hooked (Upsilon) to the waters (Mu) of ox-strength (Alpha) — the living breath that speaks from the deep and connects to first power: spirit, wind.",
    "ζωή": "The weapon (Zeta) of the great eye (Omega) enclosed by the fence (Eta) — the blade of completion fenced in and guarded: life.",
    "σταυρός": "Teeth (Sigma) marked by covenant (Tau), ox-strength (Alpha) hooked (Upsilon) to the head (Rho), watched by the eye (Omicron), consumed by teeth (Sigma) — the devouring mark where strength hangs from the chief, seen by all: cross.",
    "ἀλήθεια": "Ox-strength (Alpha) goaded (Lambda) through the fence (Eta), the wheel (Theta) beholds (Epsilon) the hand (Iota) of ox-strength (Alpha) — the power that breaks through enclosure, surrounds and reveals what the hand touches: truth.",
    "χάρις": "The crossroads (Chi) of ox-strength (Alpha), the head (Rho) worked by hand (Iota), consumed by teeth (Sigma) — the choice of the mighty chief whose hand devours debt: grace, favor.",
}


def build_system_prompt():
    """Build system prompt with seed examples."""
    examples = ""
    for grk, narrative in SEED_NARRATIVES.items():
        examples += f"""
Word: {grk}
Narrative: {narrative}
"""

    return f"""You are a Greek pictograph memory narrative writer. Each Greek letter descends from a Phoenician/Proto-Sinaitic pictograph with a concrete meaning. Your job is to write a short memory narrative (1-2 sentences) that connects the pictograph meanings of a word's letters into a vivid scene or image that LANDS ON the actual meaning of the word.

The Greek letter pictograph origins:
- Alpha (Α/α): 🐂 ox, strength, leader (from Aleph)
- Beta (Β/β): 🏠 house, family, inside (from Beth)
- Gamma (Γ/γ): 🐪 camel, lift, carry (from Gimel)
- Delta (Δ/δ): 🚪 door, pathway, enter (from Daleth)
- Epsilon (Ε/ε): 🪟 window, breath, behold (from He)
- Zeta (Ζ/ζ): ⚔️ weapon, cut, divide (from Zayin)
- Eta (Η/η): 🧱 fence, enclosure, separate (from Heth)
- Theta (Θ/θ): 🐍 wheel, surround, coil (from Teth)
- Iota (Ι/ι): 🤲 hand, deed, work (from Yod)
- Kappa (Κ/κ): 🫳 palm, open, tame (from Kaph)
- Lambda (Λ/λ): 🏹 goad, teach, direct (from Lamed)
- Mu (Μ/μ): 🌊 water, chaos, mighty (from Mem)
- Nu (Ν/ν): 🐟 fish, life, activity (from Nun)
- Xi (Ξ/ξ): 🏛️ prop, support, pillar (from Samekh)
- Omicron (Ο/ο): 👁️ eye, see, perceive (from Ayin)
- Pi (Π/π): 👄 mouth, speak, open (from Pe)
- Rho (Ρ/ρ): 👤 head, chief, first (from Resh)
- Sigma (Σ/σ/ς): 🦷 teeth, consume, destroy (from Shin)
- Tau (Τ/τ): ✝️ mark, covenant, sign (from Tav)
- Upsilon (Υ/υ): 🪝 hook, nail, connect (from Vav)
- Phi (Φ/φ): 🔥 flame, spirit, breath-fire
- Chi (Χ/χ): ✖️ cross, choice, divide
- Psi (Ψ/ψ): 🔱 trident, rising, pierce
- Omega (Ω/ω): 👁️ great eye, completion, end (from Ayin big)

Rules:
- Name each letter and its pictograph meaning in parentheses, e.g. "The ox-strength (Alpha)"
- Chain the letters in order using varied connecting phrases
- End with a dash and a phrase that ties the picture to the word's actual meaning
- The narrative must help someone REMEMBER what the word means from the pictures
- Be vivid and concrete, not abstract or vague
- Keep it to 1-2 sentences plus the closing phrase after the dash
- Do NOT include the Greek word or transliteration in the narrative
- For common short words (articles, particles, prepositions), keep the narrative proportionally short

Here are examples of excellent narratives:
{examples}
Match this style exactly. The narrative should feel like the pictures naturally lead to the meaning."""


def main():
    dry_run = "--dry-run" in sys.argv

    # Load or create narratives file
    if os.path.exists(NARRATIVES_FILE):
        with open(NARRATIVES_FILE) as f:
            narratives = json.load(f)
    else:
        narratives = dict(SEED_NARRATIVES)

    with open(TOP500_FILE) as f:
        top500 = json.load(f)

    top500_by_grk = {w["grk"]: w for w in top500}

    # Find words needing narratives
    missing = [w for w in top500 if w["grk"] not in narratives]
    print(f"{len(missing)} of {len(top500)} top-500 words need narratives", file=sys.stderr)

    if not missing:
        print("All top-500 words already have narratives!", file=sys.stderr)
        return

    if dry_run:
        print("\nWords needing narratives:", file=sys.stderr)
        for w in missing:
            print(f"  #{w['rank']:3d} {w['grk']}  ({w['eng']})  {w['letter_summary']}", file=sys.stderr)
        return

    system_prompt = build_system_prompt()
    client = anthropic.Anthropic()

    BATCH_SIZE = 10
    added = 0
    errors = 0

    for batch_start in range(0, len(missing), BATCH_SIZE):
        batch = missing[batch_start:batch_start + BATCH_SIZE]

        words_text = ""
        for w in batch:
            words_text += f"""
---
Word: {w['grk']}
Meaning: {w['eng']}
Letters: {w['letter_summary']}
"""

        user_prompt = f"""Write a pictograph memory narrative for each of these Greek words. Return ONLY a JSON object mapping each Greek word to its narrative string. No markdown fences, no explanation — just the JSON.

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

            for grk, narrative in batch_result.items():
                narratives[grk] = narrative
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
    with open(NARRATIVES_FILE, "w", encoding="utf-8") as f:
        json.dump(narratives, f, ensure_ascii=False, indent=2)

    print(f"\nDone. Added {added} narratives, {errors} errors.", file=sys.stderr)
    print(f"Total narratives: {len(narratives)}", file=sys.stderr)


if __name__ == "__main__":
    main()
