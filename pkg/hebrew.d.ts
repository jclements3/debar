/* tslint:disable */
/* eslint-disable */
/**
* Decompose a Chinese character into its semantic components.
* Takes a single character string, returns JSON CharBreakdown.
* @param {string} ch
* @returns {string}
*/
export function decompose_hanzi(ch: string): string;
/**
* Extract the probable Hebrew root from a word, stripping prefixes and suffixes.
* Returns JSON: { original, prefixes, root, suffixes }
* @param {string} word
* @returns {string}
*/
export function extract_root_json(word: string): string;
/**
* Decompose multiple words (space-separated or newline-separated).
* Returns JSON array of WordBreakdown objects.
* @param {string} text
* @returns {string}
*/
export function decompose_text(text: string): string;
/**
* Generate a pictorial memory narrative for a Hebrew word.
* Takes a Hebrew word (with or without niqqud), decomposes it,
* and returns a template narrative string.
* @param {string} word
* @returns {string}
*/
export function generate_narrative_wasm(word: string): string;
/**
* Strip niqqud (vowel points) from Hebrew text.
* @param {string} text
* @returns {string}
*/
export function strip_niqqud(text: string): string;
/**
* Generate a narrative memory story for a Chinese character.
* Takes a single character string, returns a mnemonic narrative.
* @param {string} ch
* @returns {string}
*/
export function hanzi_narrative(ch: string): string;
/**
* Decompose a single Hebrew word into its pictograph letters.
* Returns JSON: { heb, cons, letters: [{p, ph, h, n, m}] }
* @param {string} word
* @returns {string}
*/
export function decompose_word(word: string): string;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly decompose_hanzi: (a: number, b: number, c: number) => void;
  readonly decompose_text: (a: number, b: number, c: number) => void;
  readonly decompose_word: (a: number, b: number, c: number) => void;
  readonly extract_root_json: (a: number, b: number, c: number) => void;
  readonly generate_narrative_wasm: (a: number, b: number, c: number) => void;
  readonly hanzi_narrative: (a: number, b: number, c: number) => void;
  readonly strip_niqqud: (a: number, b: number, c: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
