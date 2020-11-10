mod mandelbrot;
mod utils;

use js_sys::Array;
use log::{debug, info, Level};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use toml_edit::{Decor, Document, TableKeyValue, Value};
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
    #[wasm_bindgen(js_name = writeFile)]
    fn write_file(name: &str, contents: &str);
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

#[wasm_bindgen(start)]
pub fn main() {
    console_log::init_with_level(Level::Debug).expect("error initializing logger");
    utils::set_panic_hook();
    info!("Logging initialized");
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct State {
    logs: Vec<String>,
    files: HashMap<String, File>,
}

#[wasm_bindgen]
impl State {
    pub fn new() -> State {
        info!("New state created");
        Self {
            logs: vec![String::from("Backend connected")],
            files: HashMap::new(),
        }
    }

    pub fn logs(&self) -> Array {
        self.logs.clone().into_iter().map(JsValue::from).collect()
    }

    pub fn log(&mut self, message: String) {
        info!("JS: {}", message);
        self.logs.push(message);
    }

    #[wasm_bindgen(js_name = setFiles)]
    pub fn set_files(&mut self, value: &JsValue) {
        debug!("Received files");
        let files: HashMap<String, String> = value.into_serde().unwrap();
        self.files = files
            .into_iter()
            .map(|(key, value)| (key, File::new(value)))
            .collect();
        debug!("files: {}", self.files.len());
    }

    #[wasm_bindgen(js_name = modFile)]
    pub fn mod_file(&mut self, key: &str, modified: String) {
        if let Some(file) = self.files.get_mut(key) {
            file.modified = modified
        }
    }

    pub fn files(&self) -> JsValue {
        debug!("Retrieved {} files", &self.files.len());
        let map = js_sys::Map::new();
        for (key, value) in &self.files {
            map.set(&key.into(), &value.clone().into());
        }
        map.into()
    }

    pub fn download(&self, name: &str) {
        write_file(name, &self.files[name].modified)
    }

    pub fn handle(&self, canvas: HtmlCanvasElement, name: &str) -> Result<Chart, JsValue> {
        let file = &self.files[name];
        self.edit_config(name, &file.modified).unwrap();
        Chart::mandelbrot(canvas)
    }

    fn edit_config(&self, name: &str, config: &str) -> Result<(), Box<dyn std::error::Error>> {
        let config = config.parse::<Document>()?;

        let traversed = self.traverse_config(name, config);

        info!("{:#?}", traversed);

        // config.iter().for_each(|e| info!("{:?}", &e));

        // let val_raw = config["data"]["name"].as_value_mut().unwrap();
        // let val_decor = val_raw.decor();
        // *val_raw = toml_edit::decorated(
        //     "Orsen2 -> \"orsenkucher2\"".into(),
        //     val_decor.prefix(),
        //     val_decor.suffix(),
        // );

        // config["data"].as_inline_table_mut().map(|t| t.fmt());

        // let result = config.to_string();
        // println!("{}", result);

        // fs::write("package/Duplicate.toml", result)?;

        Ok(())
    }

    fn traverse_config(&self, name: &str, doc: Document) -> Component {
        let doc = doc.as_table();
        Component::Table(Table {
            annotation: Annotation::from(&doc.decor),
            title: String::from(name),
            components: doc.iter_kv().map(Self::traverse_item).collect(),
        })
    }

    fn traverse_item((title, kv): (&str, &TableKeyValue)) -> Component {
        if let Some(table) = kv.value().as_table() {
            let decor = &table.decor;
            Component::Table(Table {
                title: String::from(title),
                annotation: decor.into(),
                components: table.iter_kv().map(Self::traverse_item).collect(),
            })
        } else {
            let value = kv.value().as_value().expect("Now item is value");
            let decor = value.decor();
            let value = match value {
                Value::Integer(f) => f.value().to_string(),
                Value::String(f) => f.value().to_string(),
                Value::Float(f) => f.value().to_string(),
                Value::DateTime(f) => f.value().to_string(),
                Value::Boolean(f) => f.value().to_string(),
                Value::Array(f) => f.to_string(),
                Value::InlineTable(f) => f.to_string(),
            };
            Component::Row(Row {
                key: String::from(title),
                value,
                annotation: decor.into(),
            })
        }
    }
}

#[derive(Debug)]
enum Component {
    Table(Table),
    Row(Row),
}

#[derive(Debug)]
struct Annotation {
    headline: String,
    footnote: String,
}

impl From<&Decor> for Annotation {
    fn from(decor: &Decor) -> Self {
        Self {
            headline: decor.prefix().trim().to_string(),
            footnote: decor.suffix().trim().to_string(),
        }
    }
}

impl From<(&Decor, &Decor)> for Annotation {
    fn from((key, value): (&Decor, &Decor)) -> Self {
        Self {
            headline: key.prefix().trim().to_string(),
            footnote: value.suffix().trim().to_string(),
        }
    }
}

#[derive(Debug)]
struct Table {
    title: String,
    annotation: Annotation,
    components: Vec<Component>,
}

#[derive(Debug)]
struct Row {
    key: String,
    value: String,
    annotation: Annotation,
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Clone)]
pub struct File {
    original: String,
    modified: String,
}

#[wasm_bindgen]
impl File {
    pub fn new(original: String) -> Self {
        let modified = original.clone();
        Self { original, modified }
    }

    pub fn modified(&self) -> String {
        self.modified.clone()
    }

    #[wasm_bindgen(js_name = isModified)]
    pub fn is_modified(&self) -> bool {
        self.original != self.modified
    }
}
