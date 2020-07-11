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

    let area = area(rectangle);

    println!("The area of rectangle is {} square pixels.", area);
}

fn area(rectangle: Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
