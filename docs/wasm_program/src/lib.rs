use wasm_bindgen::prelude::*;
use web_sys::console;
mod util;

#[wasm_bindgen(getter_with_clone)]
pub struct Ret {
    pub err: String,
    pub vis: String,
}

#[wasm_bindgen]
pub fn vis(_input: String, _output: String) -> Ret {
    let input = util::parse_input(&_input);
    let output = util::parse_output(&_output);
    let (err, vis) = util::vis(&input, &output);
    Ret {
        err: err.to_string(),
        vis: vis.to_string(),
    }
}
