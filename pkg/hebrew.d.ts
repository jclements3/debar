/* tslint:disable */
/* eslint-disable */

/**
 * Decompose multiple words (space-separated or newline-separated).
 * Returns JSON array of WordBreakdown objects.
 */
export function decompose_text(text: string): string;

/**
 * Decompose a single Hebrew word into its pictograph letters.
 * Returns JSON: { heb, cons, letters: [{p, ph, h, n, m}] }
 */
export function decompose_word(word: string): string;

/**
 * Extract the probable Hebrew root from a word, stripping prefixes and suffixes.
 * Returns JSON: { original, prefixes, root, suffixes }
 */
export function extract_root_json(word: string): string;

/**
 * Generate a pictorial memory narrative for a Hebrew word.
 * Takes a Hebrew word (with or without niqqud), decomposes it,
 * and returns a template narrative string.
 */
export function generate_narrative_wasm(word: string): string;

/**
 * Strip niqqud (vowel points) from Hebrew text.
 */
export function strip_niqqud(text: string): string;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly decompose_text: (a: number, b: number) => [number, number];
    readonly decompose_word: (a: number, b: number) => [number, number];
    readonly extract_root_json: (a: number, b: number) => [number, number];
    readonly generate_narrative_wasm: (a: number, b: number) => [number, number];
    readonly strip_niqqud: (a: number, b: number) => [number, number];
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
    readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
