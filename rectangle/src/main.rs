fn main() {
    rect1();
    rect2();
    rect3();
}

fn rect1() {
    let width = 30;
    let height = 50;

    println!(
        "The area of rectangle is {} square pixels",
        area1(height, width)
    );
}

fn area1(height: u32, width: u32) -> u32 {
    height * width
}

fn rect2() {
    let rect = (30, 50);

    println!("The area of rectangle is {} square pixels ", area2(rect));
}

fn area2(dims: (u32, u32)) -> u32 {
    dims.0 * dims.1
}

// But clone is not copy :(
// then is it managed on heap???
#[derive(Debug, Clone)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn rect3() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!("The area of rectangle is {} square pixels.", area3(&rect));
    println!(
        "The area of rectangle is {} square pixels.",
        area35(rect.clone())
    );

    println!("{}", rect.width);
    println!("{:#?}", rect);
}

fn area35(rect: Rectangle) -> u32 {
    rect.width * rect.height
}

fn area3(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

fn test(refint: &u32) {} //lol then is & necessary for rectangle?
