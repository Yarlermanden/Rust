use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(n1: i32, n2: i32) -> i32 {
    n1+n2
}

pub fn add_5(n1: i32) -> i32 {
    n1+5
}
