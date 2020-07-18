// is this struct on heap?
// is struct without Clone+Copy on heap?
// if no, is borrowing not only for heap?
#[derive(Debug, Clone, Copy)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rectangle = Rectangle {
        width: 30,
        height: 50,
    };

    let area = rectangle.area();

    println!("The area of rectangle is {} square pixels.", area);

    let sq = Rectangle::square(12);
    println!("sq area: {}", sq.area());
    println!("sq and hold: {}", sq.can_hold(&Rectangle::square(11)));
}

impl Rectangle {
    // can also be &self
    // and maybe &mut self
    fn area(self) -> u32 {
        self.width * self.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
