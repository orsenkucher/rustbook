enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

struct ListIter<'a, T> {
    val: &'a Box<List<T>>,
}

impl<'a, T> IntoIterator for List<T> {
    type Item = T;
    type IntoIter = ListIter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        ListIter {
            val: Box::new(&self),
        }
    }
}

// https://rust-unofficial.github.io/too-many-lists/
impl<T> Iterator for ListIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        // let b = self.val;
        // self.val.

        // let val = &*self.val;
        if let List::Cons(val, list) = &*self.val {
            self.val = list;
            return Some(val);
        }

        // let v: List<Self::Item> = self.val.into();
        None
        // if let List::Cons(val, list) = self.val {
        //     self.val = *list;
        //     Some(val)
        // } else {
        //     None
        // }
    }
}

// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }

// use crate::List::{Cons, Nil};

// impl Iterator for List {
//     type Item = i32;
//     fn next(&mut self) -> Option<Self::Item> {
//         if let Cons(val, list) = self {
//             Some(*val)
//         } else {
//             None
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::List::{Cons, Nil};

    #[test]
    fn it_works() {
        let list = Cons(1, Box::new(Nil));
    }
}
