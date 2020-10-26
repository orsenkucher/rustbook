mod mandelbrot;
mod utils;

use js_sys::Array;
use std::collections::HashMap;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::HtmlCanvasElement;

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
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn ondrop(event: web_sys::DragEvent) {
    event.prevent_default(); // stop browser default behavior
    event.stop_propagation(); // stop propagation to parents

    let dt = event.data_transfer().unwrap();
    let files = dt.files().unwrap();
    let psd = files.item(0).unwrap();

    let file_reader = web_sys::FileReader::new().unwrap();
    file_reader.read_as_array_buffer(&psd).unwrap();

    let onload = Closure::wrap(Box::new(move |event: web_sys::Event| {
        let file_reader: web_sys::FileReader = event.target().unwrap().dyn_into().unwrap();
        let psd = file_reader.result().unwrap();
        let psd = js_sys::Uint8Array::new(&psd);

        let mut psd_file = vec![0; psd.length() as usize];
        psd.copy_to(&mut psd_file);

        let cont = String::from_utf8_lossy(&psd_file[..]);
        alert(&cont);
        write_file("my_file.txt", &cont);
    }) as Box<dyn FnMut(_)>);

    file_reader.set_onload(Some(onload.as_ref().unchecked_ref()));
    onload.forget();
}

#[wasm_bindgen(module = "/www/src/saver.js")]
extern "C" {
    #[wasm_bindgen]
    fn write_file(name: &str, contents: &str);
}

#[wasm_bindgen(module = "/www/src/logger.js")]
extern "C" {
    #[wasm_bindgen(js_name = logDebug)]
    fn log_debug(message: &str);
}

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
        let map_coord = mandelbrot::draw(canvas).map_err(|err| err.to_string())?;
        Ok(Chart {
            convert: Box::new(map_coord),
        })
    }

    /// This function can be used to convert screen coordinates to
    /// chart coordinates.
    pub fn coord(&self, x: i32, y: i32) -> Option<Point> {
        (self.convert)((x, y)).map(|(x, y)| Point { x, y })
    }
}

#[wasm_bindgen]
pub struct State {
    logs: Vec<String>,
    files: HashMap<String, String>,
}

#[wasm_bindgen]
impl State {
    pub fn new() -> State {
        Self {
            logs: (1..=5).map(|i| format!("{}", i)).collect(),
            files: HashMap::new(),
        }
    }

    pub fn logs(&self) -> Array {
        self.logs.clone().into_iter().map(JsValue::from).collect()
    }

    pub fn log(&mut self, message: String) {
        self.logs.push(message)
    }

    #[wasm_bindgen]
    pub fn set_files(&mut self, value: JsValue) -> Result<(), JsValue> {
        log_debug("Hello1");
        let value: HashMap<String, String> = serde_wasm_bindgen::from_value(value)?;
        log_debug("Hello2");
        log_debug(&format!("files: {}", value.len()));
        self.log(format!("files: {}", value.len()));
        self.files = value;
        Ok(())
    }

    #[wasm_bindgen]
    pub fn files(&self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.files).map_err(|err| err.into())
    }
}
