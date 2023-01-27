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
