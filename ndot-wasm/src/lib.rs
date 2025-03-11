mod utils;

use ndot::make_svg_from_dot as make_svg_from_dot_rust;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(getter_with_clone)]
pub struct DotResult {
    pub svg: Option<String>,
    pub error: Option<String>,
}

#[wasm_bindgen]
pub fn make_svg_from_dot(dot: String) -> DotResult {
    let r = make_svg_from_dot_rust(dot);
    match r {
        Ok(svg) => DotResult {
            svg: Some(svg),
            error: None,
        },
        Err(e) => DotResult {
            svg: None,
            error: Some(e),
        },
    }
}
