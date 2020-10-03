mod mandelbrot;
mod utils;

use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// Type alias for the result of a drawing function.
pub type DrawResult<T> = Result<T, Box<dyn std::error::Error>>;

/// Type used on the JS side to convert screen coordinates to chart
/// coordinates.
#[wasm_bindgen]
pub struct Chart {
    convert: Box<dyn Fn((i32, i32)) -> Option<(f64, f64)>>,
}

/// Result of screen to chart coordinates conversion.
#[wasm_bindgen]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[wasm_bindgen]
impl Chart {
    /// Draw Mandelbrot set on the provided canvas element.
    /// Return `Chart` struct suitable for coordinate conversion.
    pub fn mandelbrot(canvas: HtmlCanvasElement) -> Result<Chart, JsValue> {
        unsafe { alert("HELLO") }
        // unsafe { alert(canvas) }
        mandelbrot::mytest10();
        let map_coord = mandelbrot::draw(canvas).map_err(|err| err.to_string())?;
        unsafe { alert("HELLO2") }
        Ok(Chart {
            convert: Box::new(map_coord),
        })
    }

    pub fn testfn(canvas: HtmlCanvasElement) {
        unsafe { alert("HELLO my test") }
    }

    /// This function can be used to convert screen coordinates to
    /// chart coordinates.
    pub fn coord(&self, x: i32, y: i32) -> Option<Point> {
        (self.convert)((x, y)).map(|(x, y)| Point { x, y })
    }
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    unsafe { alert("Hello, wasm-react!") }
}
