// enum List<T> {
//     Cons(T, Box<List<T>>),
//     Nil,
// }

enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

impl Iterator for List {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        if let Cons(val, list) = self {
            Some(*val)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::List::{Cons, Nil};

    #[test]
    fn it_works() {
        let list = Cons(1, Box::new(Nil));
    }
}
