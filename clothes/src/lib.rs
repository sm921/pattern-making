use pattern::measurements::{Cm, Measurements};
pub mod pattern;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn f1(str: String) -> String {
    str
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn f2() -> Cm {
    1.0
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn f3() -> Measurements {
    Measurements {
        ..Default::default()
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn f4(arg: Measurements) -> Cm {
    arg.ankle
}
