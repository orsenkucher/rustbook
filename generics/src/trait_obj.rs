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

impl Draw for String {
    fn draw(&self) {
        Text::new(self.clone()).draw(); // lol
    }
}

// Object Safety Is Required for Trait Objects
//
// You can only make object-safe traits into trait objects.
// Some complex rules govern all the properties that make a trait object safe,
// but in practice, only two rules are relevant. A trait is object safe if all
// the methods **defined in the trait** have the following properties:
// - The return type isn’t Self.
// - There are no generic type parameters.
//
// pub struct Screen {
//     pub components: Vec<Box<dyn Clone>>,
//     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::clone::Clone` cannot be made into an object
// }
//
fn main() {
    let mut screen = Screen { components: vec![] };
    let text = Text::new(String::from("App"));
    screen.add(text);

    // we impl Draw for String, so can
    screen.add(String::from("Hello"));

    screen.run();
}
