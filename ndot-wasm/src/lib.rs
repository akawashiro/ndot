mod utils;

use ndot::make_svg_from_dot as make_svg_from_dot_rust;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn make_svg_from_dot(dot: String) -> Result<String, String> {
    make_svg_from_dot_rust(dot)
}
