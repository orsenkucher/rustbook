mod mandelbrot;
mod utils;

use std::{fs, rc::Rc};

use wasm_bindgen::{prelude::*, JsCast};
// use web_sys::FileReader;
use web_sys::{Event, FileReader, HtmlCanvasElement};

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
        // unsafe { alert("HELLO") }
        // unsafe { alert(canvas) }
        // let file_reader = web_sys::FileReader::new().unwrap();
        let file = fs::read_to_string("myfile.txt");
        match file {
            Ok(buf) => {
                unsafe { alert("buf") };
            }
            Err(e) => {
                // op not supported on this platform
                unsafe { alert(&e.to_string()) };
            }
        }
        // let file_reader = FileReader::new().unwrap();
        // file_reader.read_as_binary_string("myfile.txt").unwrap();
        // file_reader.read(&psd).unwrap();
        // read_file("myfile.txt")
        // file_reader.
        // match unsafe {} {
        //     Ok(buf) => {
        //         unsafe { alert("buf") };
        //     }
        //     Err(e) => {
        //         unsafe { alert("error") };
        //     }
        // }
        mandelbrot::mytest10();
        let map_coord = mandelbrot::draw(canvas).map_err(|err| err.to_string())?;
        // unsafe { alert("HELLO2") }
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
pub fn ondrop(event: web_sys::DragEvent) {
    event.prevent_default();
    event.stop_propagation();

    // let store = Rc::clone(&store_clone);

    let dt = event.data_transfer().unwrap();
    let files = dt.files().unwrap();
    let psd = files.item(0).unwrap();

    let file_reader = web_sys::FileReader::new().unwrap();
    file_reader.read_as_array_buffer(&psd).unwrap();

    let mut onload = Closure::wrap(Box::new(move |event: Event| {
        // alert("got file");
        let file_reader: FileReader = event.target().unwrap().dyn_into().unwrap();
        let psd = file_reader.result().unwrap();
        // alert(format!("{:?}", &psd));
        let psd = js_sys::Uint8Array::new(&psd);

        let mut psd_file = vec![0; psd.length() as usize];
        psd.copy_to(&mut psd_file);

        // unsafe { alert(&psd_file.len().to_string()) }

        let cont = String::from_utf8_lossy(&psd_file[..]);

        unsafe { alert(&cont) }

        // store.borrow_mut().msg(&Msg::ReplacePsd(&psd_file));
    }) as Box<dyn FnMut(_)>);

    file_reader.set_onload(Some(onload.as_ref().unchecked_ref()));
    onload.forget();
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    unsafe { alert("Hello, wasm-react!") }
}

// #[wasm_bindgen]
// extern "C" {
//     type Buffer;
// }

// #[wasm_bindgen(module = "fs")]
// extern "C" {
//     #[wasm_bindgen(js_name = readFileSync, catch)]
//     fn read_file(path: &str) -> Result<Buffer, JsValue>;
// }
