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

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Index {
    num_bands: usize,
    band_width: usize,
    hasher : Arc<Mutex<MinHashIndex<u32, usize>>>
}

#[wasm_bindgen]
impl Index {
    pub fn new(num_bands: usize, band_width: usize) -> Index {
        Index {
            num_bands, band_width,
            hasher : Arc::new(Mutex::new(MinHashIndex::new(num_bands, band_width, 0.5)))
        }
    }

    pub fn size(&self) -> usize {
        return self.hasher.lock().unwrap().size();
    }

    pub fn insert(&mut self, id: usize, doc : String) {
        let minhasher = MinHasher32::new(self.num_bands * self.band_width);
        let sig = minhasher.create_signature(whitespace_split(&doc.to_lowercase()));
        let mut idx = self.hasher.lock().unwrap();
        idx.insert(id, sig);
    }
}

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

/*
#[wasm_bindgen]
pub fn init() -> Index {
    Index::new(42, 3)
}

#[wasm_bindgen]
pub fn add(index : &mut Index, corpus: Array) {
    let (num_bands, band_width) = (42, 3);

    for (i, doc) in corpus.iter().enumerate() {
        let d : String = doc.as_string().unwrap();
        index.insert(i, d);
    }
}
*/