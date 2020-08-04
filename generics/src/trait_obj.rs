pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    fn run(&self) {
        for c in self.components.iter() {
            c.draw();
        }
    }

    fn add(&mut self, c: impl Draw + 'static) {
        let boxed = Box::new(c);
        self.components.push(boxed);
    }
}

struct Text {
    contents: String,
}

impl Text {
    fn new(contents: String) -> Text {
        Text { contents }
    }
}

impl Draw for Text {
    fn draw(&self) {
        todo!()
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
    }
}

fn main() {
    let mut screen = Screen { components: vec![] };
    let text = Text::new(String::from("App"));
    screen.add(text);

    screen.run();
}
