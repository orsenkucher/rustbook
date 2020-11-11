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
    component: Table,
}

#[wasm_bindgen]
impl State {
    pub fn new() -> State {
        info!("New state created");
        Self {
            logs: vec![String::from("Backend connected")],
            files: HashMap::new(),
            component: Table::new(),
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

    pub fn component(&self) -> Table {
        self.component.clone()
    }

    pub fn handle(&mut self, canvas: HtmlCanvasElement, name: &str) -> Result<Chart, JsValue> {
        self.edit_config(name).unwrap();
        Chart::mandelbrot(canvas)
    }

    fn edit_config(&mut self, name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let config = &self.files[name].modified;
        let config = config.parse::<Document>()?;

        let traversed = self.traverse_config(name, config);

        info!("{:#?}", traversed);

        if let Component::Table(table) = traversed {
            self.component = table;
        }

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

#[derive(Debug, Serialize, Deserialize, Clone)]
enum Component {
    Table(Table),
    Row(Row),
}

#[wasm_bindgen]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Annotation {
    headline: String,
    footnote: String,
}

#[wasm_bindgen]
impl Annotation {
    pub fn headline(&self) -> String {
        self.headline.clone()
    }

    pub fn footnote(&self) -> String {
        self.footnote.clone()
    }
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

#[wasm_bindgen]
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Table {
    title: String,
    annotation: Annotation,
    components: Vec<Component>,
}

impl Table {
    fn new() -> Self {
        // Default::default()
        Self {
            title: "empty".to_string(),
            annotation: Default::default(),
            components: Default::default(),
        }
    }
}

#[wasm_bindgen]
impl Table {
    pub fn title(&self) -> String {
        self.title.clone()
    }

    pub fn annotation(&self) -> Annotation {
        self.annotation.clone()
    }

    // pub fn components(&self) -> Vec<(Option<Table>, Option<Row>)> {
    //     self.components
    //         .iter()
    //         .map(|component| match component {
    //             Component::Table(t) => (Some(t), None),
    //             Component::Row(r) => (None, Some(r)),
    //         })
    //         .collect()
    // }

    pub fn components(&self) -> ComponentIter {
        ComponentIter::new(&self.components)
    }
}

#[wasm_bindgen]
pub struct ComponentIter {
    iter: Box<dyn Iterator<Item = Component>>,
    item: Option<Component>,
}

impl ComponentIter {
    fn new(components: &Vec<Component>) -> Self {
        let iter = components.clone().into_iter();
        Self {
            iter: Box::new(iter),
            item: None,
        }
    }
}

#[wasm_bindgen]
impl ComponentIter {
    pub fn next(&mut self) -> Option<String> {
        self.item = self.iter.next();
        self.item.as_ref().map(|it| match it {
            Component::Table(_) => String::from("table"),
            Component::Row(_) => String::from("row"),
        })
    }

    #[wasm_bindgen(js_name = nextTable)]
    pub fn next_table(&self) -> Option<Table> {
        match &self.item {
            Some(Component::Table(t)) => Some(t.clone()),
            _ => None,
        }
    }

    #[wasm_bindgen(js_name = nextRow)]
    pub fn next_row(&self) -> Option<Row> {
        match &self.item {
            Some(Component::Row(r)) => Some(r.clone()),
            _ => None,
        }
    }
}

#[wasm_bindgen]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Row {
    key: String,
    value: String,
    annotation: Annotation,
}

#[wasm_bindgen]
impl Row {
    pub fn key(&self) -> String {
        self.key.clone()
    }

    pub fn value(&self) -> String {
        self.value.clone()
    }

    pub fn annotation(&self) -> Annotation {
        self.annotation.clone()
    }
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
