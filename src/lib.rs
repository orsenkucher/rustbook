mod utils;

use wasm_bindgen::{prelude::*, JsCast};

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
    event.prevent_default();
    event.stop_propagation();

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
