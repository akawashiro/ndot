mod utils;

use ndot::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn make_svg_from_dot(dot: String) -> Result<String, String> {
    ndot::make_svg_from_dot(dot)
}
