fn main() {
    println!("Hello, world!");

    another_function(20, 4);
    println!("Another function. {}", five());
}

fn another_function(x: i32, _: i32) {
    println!("Another function. {}", x);

    let y = {
        let x = 3;
        x + 1
    };

    println!("Y is: {}", y);
}

fn five() -> i32 {
    5
}
