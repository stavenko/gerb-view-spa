use wasm_bindgen::prelude::*;

#[wasm_bindgen(inline_js = "export const rand = () => Math.random()")]
extern "C" {
  pub fn rand()->f64;
}
