mod mandelbrot;
mod utils;

use js_sys::Array;
use log::{debug, info, Level};
use serde::{Deserialize, Serialize};
use std::{cell::RefCell, collections::HashMap, rc::Rc};
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
// #[derive(Serialize, Deserialize)]
pub struct State {
    logs: Vec<String>,
    files: HashMap<String, File>,
    component: (String, HashMap<String, TableWrapper>),
    document: Option<(String, Rc<RefCell<Document>>)>,
}

#[wasm_bindgen]
impl State {
    pub fn new() -> State {
        info!("New state created");
        let mut component = (String::from("empty"), HashMap::new());
        component
            .1
            .insert(component.0.clone(), TableWrapper::new(Table::new()));
        Self {
            logs: vec![String::from("Backend connected")],
            files: HashMap::new(),
            component,
            document: None,
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
        files.into_iter().for_each(|(key, value)| {
            self.files
                .entry(key)
                .and_modify(|entry| entry.modified = value.clone())
                .or_insert_with(|| File::new(value));
        });
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

    pub fn component(&self) -> TableWrapper {
        self.component.1[&self.component.0].clone()
    }

    pub fn handle(&mut self, canvas: HtmlCanvasElement, name: &str) -> Result<Chart, JsValue> {
        self.edit_config(name).map_err(|err| {
            self.log(err.to_string());
            err.to_string()
        })?;
        Chart::mandelbrot(canvas)
    }

    fn edit_config(&mut self, name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let config = &self.files[name].modified;
        let doc = config.parse::<Document>()?;

        self.component.0 = String::from(name);

        let vacant = self.component.1.get(&self.component.0).is_none();
        if vacant {
            let doc = Rc::new(RefCell::new(doc));
            self.document = Some((String::from(name), Rc::clone(&doc)));
            let traversed = self.traverse_doc(name, doc);
            info!("{:#?}", traversed);
            self.component.1.insert(
                self.component.0.clone(),
                match traversed {
                    Component::Table(t) => t,
                    _ => panic!("root component is always a table"),
                },
            );
        } else {
            let c = &self.component.1[&self.component.0];
            self.document = Some((String::from(name), Rc::clone(&c.0.borrow().doc)));
        }

        Ok(())
    }

    fn traverse_doc(&self, name: &str, doc: Rc<RefCell<Document>>) -> Component {
        let doc_ref = doc.borrow();
        let table = doc_ref.as_table();
        Component::Table(TableWrapper::new(Table {
            annotation: Annotation::from(&table.decor),
            title: String::from(name),
            doc: Rc::clone(&doc),
            components: table
                .iter_kv()
                .map(|kv| Self::traverse_item(kv, vec![], Rc::clone(&doc)))
                .collect(),
        }))
    }

    fn traverse_item(
        (title, kv): (&str, &TableKeyValue),
        mut path: Vec<String>,
        doc: Rc<RefCell<Document>>,
    ) -> Component {
        path.push(String::from(title));
        if let Some(table) = kv.value().as_table() {
            let decor = &table.decor;
            Component::Table(TableWrapper::new(Table {
                title: String::from(title),
                doc: Rc::clone(&doc),
                annotation: decor.into(),
                components: table
                    .iter_kv()
                    .map(|kv| Self::traverse_item(kv, path.clone(), Rc::clone(&doc)))
                    .collect(),
            }))
        } else {
            let value = kv.value().as_value().expect("Now item is value");
            let decor = kv.decor().unwrap();
            let value = match value {
                Value::Integer(f) => f.value().to_string(),
                Value::String(f) => f.value().to_string(),
                Value::Float(f) => f.value().to_string(),
                Value::DateTime(f) => f.value().to_string(),
                Value::Boolean(f) => f.value().to_string(),
                Value::Array(f) => f.to_string(),
                Value::InlineTable(f) => f.to_string(),
            };
            Component::Row(RowWrapper::new(Row {
                key: String::from(title),
                value: vec![value],
                doc,
                path: path.clone(),
                annotation: decor.into(),
            }))
        }
    }

    pub fn evaluate(&mut self) {
        if let Some((name, doc)) = &self.document {
            let file = self.files.get_mut(name).unwrap();
            info!("Name: {}, Doc: {}", name, doc.borrow());
            file.modified = format!("{}", doc.borrow().to_string_in_original_order());
        }
    }
}

//  Serialize, Deserialize,
#[derive(Debug, Clone)]
enum Component {
    Table(TableWrapper),
    Row(RowWrapper),
}

#[wasm_bindgen] //  Serialize, Deserialize,
#[derive(Debug, Default, Clone)]
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

#[wasm_bindgen] //  Serialize, Deserialize,
#[derive(Debug, Clone, Default)]
pub struct Table {
    title: String,
    annotation: Annotation,
    components: Vec<Component>,
    doc: Rc<RefCell<Document>>,
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct TableWrapper(Rc<RefCell<Table>>);

#[wasm_bindgen]
impl TableWrapper {
    fn new(table: Table) -> Self {
        Self(Rc::new(RefCell::new(table)))
    }

    pub fn title(&self) -> String {
        self.0.borrow().title()
    }

    pub fn annotation(&self) -> Annotation {
        self.0.borrow().annotation()
    }

    pub fn components(&self) -> ComponentIter {
        self.0.borrow().components()
    }
}

impl Table {
    fn new() -> Self {
        Self {
            title: "empty".to_string(),
            ..Default::default()
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
    pub fn next_table(&self) -> Option<TableWrapper> {
        match &self.item {
            Some(Component::Table(t)) => Some(t.clone()),
            _ => None,
        }
    }

    #[wasm_bindgen(js_name = nextRow)]
    pub fn next_row(&self) -> Option<RowWrapper> {
        match &self.item {
            Some(Component::Row(r)) => Some(r.clone()),
            _ => None,
        }
    }
}

#[wasm_bindgen] // Serialize, Deserialize,
#[derive(Debug)]
pub struct Row {
    key: String,
    value: Vec<String>,
    annotation: Annotation,
    path: Vec<String>,
    doc: Rc<RefCell<Document>>,
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct RowWrapper(Rc<RefCell<Row>>);

#[wasm_bindgen]
impl RowWrapper {
    fn new(row: Row) -> Self {
        Self(Rc::new(RefCell::new(row)))
    }

    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }

    pub fn key(&self) -> String {
        self.0.borrow().key()
    }

    pub fn value(&self) -> String {
        self.0.borrow().value()
    }

    pub fn annotation(&self) -> Annotation {
        self.0.borrow().annotation()
    }

    #[wasm_bindgen(js_name=isModified)]
    pub fn is_modified(&self) -> bool {
        self.0.borrow().is_modified()
    }

    #[wasm_bindgen(js_name=modifyValue)]
    pub fn modify_value(&mut self, value: &str) {
        self.0.borrow_mut().modify_value(value);
    }
}

#[wasm_bindgen]
impl Row {
    pub fn key(&self) -> String {
        self.key.clone()
    }

    pub fn value(&self) -> String {
        self.value.last().unwrap().clone()
    }

    pub fn annotation(&self) -> Annotation {
        self.annotation.clone()
    }

    #[wasm_bindgen(js_name=isModified)]
    pub fn is_modified(&self) -> bool {
        info!("values: {:?}", self.value);
        info!("res: {:?}", self.value.first() != self.value.last());
        self.value.first() != self.value.last()
    }

    #[wasm_bindgen(js_name=modifyValue)]
    pub fn modify_value(&mut self, value: &str) {
        self.value.push(String::from(value));
        self.mutate_doc(value);
    }

    fn mutate_doc(&self, value: &str) {
        let root = &mut self.doc.borrow_mut().root;
        let row = self.path.iter().fold(root, |item, key| &mut item[key]);
        row.as_value_mut().unwrap().mutate(value.into());
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
        let prep = |s: &str| s.replace("\r", "").replace("\n", "");
        prep(&self.original) != prep(&self.modified)
    }
}
