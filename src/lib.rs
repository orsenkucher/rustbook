mod mandelbrot;
mod spectrum;
mod utils;

use js_sys::Array;
use log::{debug, info, Level};
use serde::{Deserialize, Serialize};
use std::{cell::RefCell, collections::HashMap, rc::Rc};
use toml_edit::{Decor, Document, TableKeyValue};
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

    fn spectrum(canvas: HtmlCanvasElement, config: Config) -> Result<Chart, JsValue> {
        let map_coord = spectrum::draw(canvas, config).map_err(|err| err.to_string())?;
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
pub struct State {
    logs: Vec<String>,
    files: HashMap<String, File>,
    component: (String, HashMap<String, TableWrapper>),
    document: Option<(String, Rc<RefCell<Document>>)>,
}

const DEFAULT: &str = r#"
# Перша лінія спектру
[[lines]]
name = "Line 1" # Назва лінії
energy = 3.5 # Її енергія
intensity = 600 # Інтенсивність
FWHM = 0.39 # Повна ширина но половині висоти   

# Друга лінія спектру
[[lines]]
name = "Line 2"
energy = 2.5 # МеВ
intensity = 400
FWHM = 0.15 # МеВ

# Третя лінія спектру
[[lines]]
name = "Line 3"
energy = 1.7
intensity = 300
FWHM = 0.34

# Фон
[background]
E1 = 199.5
E2 = 0.001
A = -0.003
B = 4.4

# Енергетичний діапазон 
[range] # (від 0 до Emax)
Emax = 4.5 # Максимальна енергія 
Emin = 0.0 # Початковий зсув
chan_number = 2000 # Кількість каналів спектру
"#;

const DEFAULT2: &str = r#"
# Пошукач піків
[finder]
smoothing = 4.0 # Сгладження
pmax = 17.0 # Maксимальна
pmin = 15.0 # Мінімальна
h1 = 4.0
h2 = 8.0
h3 = 9.0
"#;

#[wasm_bindgen]
impl State {
    pub fn new() -> State {
        info!("New state created");
        let mut component = (String::from("empty"), HashMap::new());
        component
            .1
            .insert(component.0.clone(), TableWrapper::new(Table::new()));
        let files = vec![
            (
                String::from("Default.toml"),
                File::new(String::from(DEFAULT.trim())),
            ),
            (
                String::from("Task2.toml"),
                File::new(String::from(DEFAULT2.trim())),
            ),
        ]
        .into_iter()
        .collect();
        Self {
            logs: vec![String::from("Backend connected")],
            files,
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
        let mut modified = Vec::new();
        files.into_iter().for_each(|(key, value)| {
            self.files
                .entry(key.clone())
                .and_modify(|entry| {
                    modified.push(key);
                    entry.original = value.clone();
                    entry.modified = value.clone();
                })
                .or_insert_with(|| File::new(value));
        });
        self.purge_updated(modified);
        debug!("files: {}", self.files.len());
    }

    fn purge_updated(&mut self, keys: Vec<String>) {
        for key in keys {
            if self.component.1.get(&key).is_some() {
                self.component.1.remove(&key);
            }

            if key == self.component.0 {
                if let Err(err) = self.edit_config(&key) {
                    self.log(err.to_string());
                };
            }
        }
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

    pub fn handle(&mut self, _canvas: HtmlCanvasElement, name: &str) -> Result<String, JsValue> {
        self.edit_config(name).map_err(|err| {
            self.log(err.to_string());
            err.to_string()
        })?;

        let config = self.parse(name).map_err(|err| {
            self.log(err.to_string());
            err.to_string()
        })?;
        info!("{:#?}", config);
        let parsed = serde_json::to_string(&config).map_err(|err| {
            self.log(err.to_string());
            JsValue::from(err.to_string())
        })?;
        self.validate(&config, parsed)
        // Chart::spectrum(canvas, config)
    }

    fn validate(&mut self, config: &Config, parsed: String) -> Result<String, JsValue> {
        for line in &config.lines {
            if line.energy < 0.0 {
                let message = format!("Negative energy in line {}", line.name);
                return self.error(&message);
            }
            if line.energy > config.range.emax {
                let message = format!(
                    "Over limit energy in line {}, max limit: {}",
                    line.name, config.range.emax
                );
                return self.error(&message);
            }
            if line.intensity < 0 {
                let message = format!("Negative intensity in line {}", line.name);
                return self.error(&message);
            }
            if line.fwhm < 0.0 {
                let message = format!("Negative FWHM in line {}", line.name);
                return self.error(&message);
            }
        }
        Ok(parsed)
    }

    fn error(&mut self, message: &str) -> Result<String, JsValue> {
        self.log(String::from(message));
        log::warn!("Editing error: {}", message);
        return Err(message.into());
    }

    pub fn rerender(&mut self, canvas: HtmlCanvasElement) -> Result<String, JsValue> {
        let name = &self.document.as_ref().unwrap().0.clone();
        self.handle(canvas, name)
        // let config = self.parse(name).map_err(|err| err.to_string())?;
        // Chart::spectrum(canvas, config)
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
            self.component.1.insert(
                self.component.0.clone(),
                match traversed {
                    Component::Table(t) => t,
                    _ => unreachable!("root component is always a table"),
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
        let path = vec![String::from(name)];
        Component::Table(TableWrapper::new(Table {
            annotation: Annotation::from(&table.decor),
            title: String::from(name),
            doc: Rc::clone(&doc),
            path: path.clone(),
            components: table
                .iter_kv()
                .map(|kv| Self::traverse_item(kv, path.clone(), Rc::clone(&doc)))
                .collect(),
        }))
    }

    fn traverse_item(
        (title, kv): (&str, &TableKeyValue),
        mut path: Vec<String>,
        doc: Rc<RefCell<Document>>,
    ) -> Component {
        path.push(String::from(title));
        match kv.value() {
            toml_edit::Item::Table(table) => Self::traverse_table(title, table, path, doc),
            toml_edit::Item::ArrayOfTables(tables) => {
                Self::traverse_array_of_tables(title, tables, path, doc)
            }
            toml_edit::Item::Value(value) => {
                Self::traverse_value(title, value, kv.decor().unwrap(), path, doc)
            }
            toml_edit::Item::None => unreachable!("Traversing Item::None"),
        }
    }

    fn traverse_table(
        title: &str,
        table: &toml_edit::Table,
        path: Vec<String>,
        doc: Rc<RefCell<Document>>,
    ) -> Component {
        let decor = &table.decor;
        Component::Table(TableWrapper::new(Table {
            title: String::from(title),
            doc: Rc::clone(&doc),
            path: path.clone(),
            annotation: decor.into(),
            components: table
                .iter_kv()
                .map(|kv| Self::traverse_item(kv, path.clone(), Rc::clone(&doc)))
                .collect(),
        }))
    }

    fn traverse_array_of_tables(
        title: &str,
        tables: &toml_edit::ArrayOfTables,
        path: Vec<String>,
        doc: Rc<RefCell<Document>>,
    ) -> Component {
        Component::ArrayOfTables(TableWrapper::new(Table {
            title: String::from(title),
            doc: Rc::clone(&doc),
            path: path.clone(),
            annotation: Default::default(),
            components: tables
                .iter()
                .enumerate()
                .map(|(idx, table)| {
                    let mut path = path.clone();
                    let index = format!("{}", idx);
                    path.push(index.clone());
                    Self::traverse_table(&index, table, path, Rc::clone(&doc))
                })
                .collect(),
        }))
    }

    fn traverse_value(
        title: &str,
        value: &toml_edit::Value,
        decor: (&Decor, &Decor),
        path: Vec<String>,
        doc: Rc<RefCell<Document>>,
    ) -> Component {
        let value = match value {
            toml_edit::Value::Float(f) => Value::Float(*f.value()),
            toml_edit::Value::Integer(f) => Value::Integer(*f.value()),
            toml_edit::Value::Boolean(f) => Value::Boolean(*f.value()),
            toml_edit::Value::String(f) => Value::String(f.value().clone()),
            toml_edit::Value::DateTime(f) => Value::String(f.value().to_string()),
            toml_edit::Value::InlineTable(f) => Value::String(f.to_string()),
            toml_edit::Value::Array(f) => Value::String(f.to_string()),
        };
        Component::Row(RowWrapper::new(Row {
            key: String::from(title),
            value: vec![value],
            doc,
            path: path.clone(),
            annotation: decor.into(),
        }))
    }

    pub fn evaluate(&mut self) {
        if let Some((name, doc)) = &self.document {
            let file = self.files.get_mut(name).unwrap();
            // info!("Name: {}, Doc: {}", name, doc.borrow());
            file.modified = format!("{}", doc.borrow().to_string_in_original_order());
        }
    }

    fn parse(&self, name: &str) -> Result<Config, Box<dyn std::error::Error>> {
        Ok(toml::from_str(&self.files[name].modified)?)
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    lines: Vec<Line>,
    background: Background,
    range: Range,
}

#[derive(Serialize, Deserialize, Debug)]
struct Line {
    name: String,
    energy: f64,
    intensity: i32,

    #[serde(alias = "FWHM")]
    fwhm: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Background {
    #[serde(alias = "E1")]
    e1: f64,

    #[serde(alias = "E2")]
    e2: f64,

    #[serde(alias = "A")]
    a: f64,

    #[serde(alias = "B")]
    b: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Range {
    #[serde(alias = "Emax")]
    emax: f64,

    #[serde(alias = "Emin")]
    emin: f64,

    chan_number: i32,
}

#[derive(Debug, Clone)]
enum Component {
    Row(RowWrapper),
    Table(TableWrapper),
    ArrayOfTables(TableWrapper),
}

#[wasm_bindgen]
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

#[wasm_bindgen]
#[derive(Debug, Clone, Default)]
pub struct Table {
    title: String,
    annotation: Annotation,
    components: Vec<Component>,
    path: Vec<String>,
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

    pub fn create(&mut self) {
        self.0.borrow_mut().create()
    }

    pub fn remove(&mut self) {
        self.0.borrow_mut().remove()
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

    pub fn create(&mut self) {
        self.create_in_doc();
        self.create_component();
    }

    fn create_component(&mut self) {
        let next = self.components.len().to_string();
        let mut path = self.path.clone();
        path.push(next.clone());

        let component = self.components.last().unwrap().deep_clone(path, true);

        if let Component::Table(table) = &component {
            table.0.borrow_mut().title = next;
        }

        self.components.push(component);
    }

    fn create_in_doc(&self) {
        let root = &mut self.doc.borrow_mut().root;
        let array = self.path[1..]
            .iter()
            .fold(root, |item, key| &mut item[key])
            .as_array_of_tables_mut()
            .unwrap();
        let mut table = array.get(array.len() - 1).unwrap().clone();
        table.decor = Decor::new("\n", "");
        array.append(table);
    }

    fn deep_clone(&self, mut path: Vec<String>, path_handled: bool) -> Table {
        if !path_handled {
            path.push(self.title.clone());
        }

        Self {
            annotation: Default::default(),
            path: path.clone(),
            components: self
                .components
                .iter()
                .map(|component| component.deep_clone(path.clone(), false))
                .collect(),
            ..self.clone()
        }
    }

    pub fn remove(&mut self) {
        self.remove_component();
        self.remove_in_doc();
    }

    fn remove_component(&mut self) {
        if self.components.len() > 1 {
            self.components.pop();
        }
    }

    fn remove_in_doc(&self) {
        let root = &mut self.doc.borrow_mut().root;
        let array = self.path[1..]
            .iter()
            .fold(root, |item, key| &mut item[key])
            .as_array_of_tables_mut()
            .unwrap();
        let len = array.len();
        if len > 1 {
            array.remove(len - 1);
        }
    }
}

impl Component {
    fn deep_clone(&self, path: Vec<String>, path_handled: bool) -> Component {
        match self {
            Component::Table(table) => Component::Table(TableWrapper::new(
                table.0.borrow().deep_clone(path, path_handled),
            )),
            Component::ArrayOfTables(array) => Component::ArrayOfTables(TableWrapper::new(
                array.0.borrow().deep_clone(path, path_handled),
            )),
            Component::Row(row) => Component::Row(RowWrapper::new(row.0.borrow().deep_clone(path))),
        }
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
            Component::Row(_) => String::from("row"),
            Component::Table(_) => String::from("table"),
            Component::ArrayOfTables(_) => String::from("array"),
        })
    }

    #[wasm_bindgen(js_name = nextTable)]
    pub fn next_table(&self) -> Option<TableWrapper> {
        match &self.item {
            Some(Component::Table(t)) | Some(Component::ArrayOfTables(t)) => Some(t.clone()),
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

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct Row {
    key: String,
    value: Vec<Value>,
    annotation: Annotation,
    path: Vec<String>,
    doc: Rc<RefCell<Document>>,
}

#[derive(Debug, Clone, PartialEq)]
enum Value {
    Float(f64),
    Integer(i64),
    Boolean(bool),
    String(String),
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Integer(v) => write!(f, "{}", v),
            Value::String(v) => write!(f, "{}", v),
            Value::Float(v) => write!(f, "{}", v),
            Value::Boolean(v) => write!(f, "{}", v),
        }
    }
}

impl Value {
    fn into(self) -> toml_edit::Value {
        match self {
            Value::Integer(v) => v.into(),
            Value::String(v) => v.into(),
            Value::Float(v) => v.into(),
            Value::Boolean(v) => v.into(),
        }
    }
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

    pub fn original(&self) -> String {
        self.0.borrow().original()
    }

    pub fn modified(&self) -> String {
        self.0.borrow().modified()
    }

    pub fn path(&self) -> String {
        self.0.borrow().path()
    }
}

#[wasm_bindgen]
impl Row {
    pub fn key(&self) -> String {
        self.key.clone()
    }

    pub fn value(&self) -> String {
        self.modified()
    }

    pub fn annotation(&self) -> Annotation {
        self.annotation.clone()
    }

    #[wasm_bindgen(js_name=isModified)]
    pub fn is_modified(&self) -> bool {
        self.value.first() != self.value.last()
    }

    #[wasm_bindgen(js_name=modifyValue)]
    pub fn modify_value(&mut self, value: &str) {
        if value.is_empty() {
            let empty = Value::String(String::new());
            if let Some(Value::String(_)) = self.value.first() {
                self.mutate_doc(empty.clone());
            }
            return self.value.push(empty);
        }

        let value = match self.value.first() {
            Some(Value::Float(_)) => value
                .parse::<f64>()
                .map_err(|err| err.to_string())
                .and_then(|val| {
                    let v = Value::Float(val);
                    if v.to_string() != value {
                        self.value.push(Value::String(String::from(value)));
                        return Err(String::from("Partial float parsing"));
                    }
                    Ok(v)
                }),
            Some(Value::Integer(_)) => value
                .parse()
                .map(|val| Value::Integer(val))
                .map_err(|err| err.to_string()),
            Some(Value::Boolean(_)) => value
                .parse()
                .map(|val| Value::Boolean(val))
                .map_err(|err| err.to_string()),
            Some(Value::String(_)) => Ok(Value::String(String::from(value))),
            None => unreachable!("At least one value is always present"),
        };

        match value {
            Ok(value) => {
                self.value.push(value.clone());
                self.mutate_doc(value);
            }
            Err(err) => log::warn!("Editing error: {}", err),
        }
    }

    pub fn original(&self) -> String {
        self.value.first().unwrap().to_string()
    }

    pub fn modified(&self) -> String {
        self.value.last().unwrap().to_string()
    }

    pub fn path(&self) -> String {
        self.path.iter().fold(String::new(), |acc, next| acc + next)
    }

    fn mutate_doc(&self, value: Value) {
        let root = &mut self.doc.borrow_mut().root;
        let row = self.path[1..].iter().fold(root, |item, key| &mut item[key]);
        row.as_value_mut().unwrap().mutate(value.into());
    }

    fn mutate_doc_without_decor(&self, value: Value) {
        let root = &mut self.doc.borrow_mut().root;
        let row = self.path[1..].iter().fold(root, |item, key| &mut item[key]);
        row.as_value_mut()
            .unwrap()
            .mutate_without_decor(value.into());
    }

    fn deep_clone(&self, path: Vec<String>) -> Row {
        let mut path = path.clone();
        path.push(self.key.clone());
        let default_value = match self.value.first().expect("One value is always present") {
            Value::Float(_) => Value::Float(0.0),
            Value::Integer(_) => Value::Integer(0),
            Value::Boolean(_) => Value::Boolean(false),
            Value::String(_) => Value::String(String::new()),
        };
        let cloned = Self {
            annotation: Default::default(),
            value: vec![default_value.clone()],
            path,
            ..self.clone()
        };
        cloned.mutate_doc_without_decor(default_value);
        cloned
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
