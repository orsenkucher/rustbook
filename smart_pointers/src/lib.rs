// https://rust-unofficial.github.io/too-many-lists/

enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

struct ListIter<'a, T> {
    val: &'a Box<List<T>>,
}

impl<'a, T> IntoIterator for &'a Box<List<T>> {
    type Item = &'a T;
    type IntoIter = ListIter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        ListIter { val: self }
    }
}

impl<'a, T> Iterator for ListIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if let List::Cons(v, ll) = &**self.val {
            self.val = ll;
            Some(v)
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
        let list = Box::new(Cons(2, Box::new(Cons(1, Box::new(Nil)))));
        assert_eq!(list.into_iter().collect::<Vec<_>>(), vec![&2, &1])
    }
}
