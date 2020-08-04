use std::fmt::Display;

pub struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        // Point { x, y }
        Self { x, y } // also possible haha
    }
}

impl<T: Display + PartialOrd> Point<T> {
    fn cmp_display(&self) {
        if self.x > self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

trait ToMyString {
    fn to_string(&self) -> String;
}

// Implementations of a trait on any type that satisfies
// the trait bounds are called blanket implementations.
impl<T: Display> ToMyString for T {
    fn to_string(&self) -> String {
        todo!()
    }
}
