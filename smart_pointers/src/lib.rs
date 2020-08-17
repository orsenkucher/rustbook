// todo: https://rust-unofficial.github.io/too-many-lists/

enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

struct ListIter<'a, T>(&'a Box<List<T>>);

impl<'a, T> std::ops::Deref for ListIter<'a, T> {
    type Target = &'a Box<List<T>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, T> IntoIterator for &'a Box<List<T>> {
    type Item = &'a T;
    type IntoIter = ListIter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        ListIter(self)
    }
}

impl<'a, T> Iterator for ListIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if let List::Cons(v, ll) = self as &List<_> {
            self.0 = ll;
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
