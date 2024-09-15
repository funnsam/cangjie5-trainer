use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn chars_len() -> usize { cangjie_dict::CHARS.len() }

#[wasm_bindgen]
pub fn id_char(c: usize) -> char { cangjie_dict::CHARS[c].0 }

#[wasm_bindgen]
pub fn id_codes(c: usize) -> Vec<String> {
    cangjie_dict::CHARS[c].1.iter().map(|s| s.to_string()).collect()
}
