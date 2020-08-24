pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        self.components.iter().for_each(|c| c.draw());
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!(
            "Button of {}x{} size: {}",
            self.width, self.height, self.label
        );
    }
}

impl Button {
    pub fn on_click() {}
}


