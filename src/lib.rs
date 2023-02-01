mod utils;

use std::borrow::BorrowMut;
use gaoya::minhash::{MinHashIndex, MinHasher32, MinHasher};
use gaoya::clustering::clustering_serial::{Clusterer, ClusterPoint, ClusterPointInner};
use gaoya::text::whitespace_split;
use std::collections::{HashMap, HashSet};
use std::ops::DerefMut;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use js_sys::{Array, Uint32Array};
use wasm_bindgen::prelude::*;
use fnv::FnvBuildHasher;
use gaoya::clustering::QueryIndex;
use wasm_bindgen::JsObject;

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

fn create_uint32(res : &Vec<usize>) -> Uint32Array {
    let t : Vec<u32> = res.iter().map(|x| *x as u32).collect();
    js_sys::Uint32Array::from(t.as_slice())
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

    pub fn cluster(&self) -> js_sys::Array {
        let result= self._cluster();

        let a = js_sys::Array::new();

        for t in result.iter().map(|x| create_uint32(x)) {
            a.push(&t);
        }

        return a;
    }

    fn _cluster(&self) -> Vec<Vec<usize>> {
        let mut key_to_cluster: HashMap<usize, usize> = HashMap::new();
        let mut clusters = vec![];

        for (key, signature) in self.hasher.get_id_signature_map() {
            if key_to_cluster.contains_key(key) {
                continue;
            }
            let result : Vec<usize> =
                self.hasher.query(signature).into_iter()
                    .filter(|node| !key_to_cluster.contains_key(node))
                    .cloned().collect();

            if result.len() > 1 {
                for key in &result {
                    key_to_cluster.insert(key.clone(), clusters.len());
                }
                clusters.push(result)
            }
        }
        clusters.sort_by(|a, b| b.len().partial_cmp(&a.len()).unwrap());
        return clusters;
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
