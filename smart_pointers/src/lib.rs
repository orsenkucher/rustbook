pub enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

pub struct Iter<'a, T>(&'a List<T>);

impl<'a, T> IntoIterator for &'a List<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        Iter(self)
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if let List::Cons(v, ll) = &*self.0 {
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
        let list = Cons(2, Box::new(Cons(1, Box::new(Nil))));
        assert_eq!(list.into_iter().collect::<Vec<_>>(), vec![&2, &1])
    }
}
