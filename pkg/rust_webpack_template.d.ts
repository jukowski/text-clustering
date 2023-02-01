/* tslint:disable */
/* eslint-disable */
/**
*/
export class Index {
  free(): void;
/**
* @param {number} num_bands
* @param {number} band_width
* @param {number} threshold
* @returns {Index}
*/
  static new(num_bands: number, band_width: number, threshold: number): Index;
/**
* @returns {Array<any>}
*/
  cluster(): Array<any>;
/**
* @returns {number}
*/
  size(): number;
/**
* @param {string} doc
* @returns {Uint32Array}
*/
  query(doc: string): Uint32Array;
/**
* @param {number} id
* @param {string} doc
*/
  insert(id: number, doc: string): void;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_index_free: (a: number) => void;
  readonly index_new: (a: number, b: number, c: number) => number;
  readonly index_cluster: (a: number) => number;
  readonly index_size: (a: number) => number;
  readonly index_query: (a: number, b: number, c: number, d: number) => void;
  readonly index_insert: (a: number, b: number, c: number, d: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
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
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
