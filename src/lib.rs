mod utils;

use std::borrow::BorrowMut;
use gaoya::minhash::{MinHashIndex, MinHasher32, MinHasher} ;
use gaoya::text::whitespace_split;
use std::collections::HashSet;
use std::ops::DerefMut;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use js_sys::Array;
use wasm_bindgen::prelude::*;
use fnv::FnvBuildHasher;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Index {
    num_bands: usize,
    band_width: usize,
    min_hasher: MinHasher32<FnvBuildHasher>,
    hasher : MinHashIndex<u32, usize>
}

#[wasm_bindgen]
impl Index {
    pub fn new(num_bands: usize, band_width: usize, threshold: f64) -> Index {
        Index {
            num_bands, band_width,
            min_hasher: MinHasher32::new(num_bands * band_width),
            hasher : MinHashIndex::new(num_bands, band_width, threshold)
        }
    }

    pub fn size(&self) -> usize {
        return self.hasher.size();
    }

    pub fn query(&self, doc : String) -> Vec<usize> {
        let sig = self.min_hasher.create_signature(whitespace_split(&doc.to_lowercase()));
        let result = self.hasher.query(&sig);
        return result.into_iter().cloned().collect();
    }

    pub fn insert(&mut self, id: usize, doc : String) {
        let sig = self.min_hasher.create_signature(whitespace_split(&doc.to_lowercase()));
        self.hasher.insert(id, sig);
    }
}
