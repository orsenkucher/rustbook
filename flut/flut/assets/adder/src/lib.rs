mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() -> i32 {
    // alert("Hello, adder!");
    // String::from("Hello from Rust!")
    1234
}

// #[wasm_bindgen]
// pub fn add() -> String {
//     String::from("Hello from Rust!")
// }

// #[wasm_bindgen]
// pub fn test_add() -> String {
//   String::from("Hello from Rust")
// }
