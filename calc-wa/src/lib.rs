use wasm_bindgen::prelude::*;


#[wasm_bindgen]
pub fn mult_by2(num: usize) -> usize {
    num * 2
}
