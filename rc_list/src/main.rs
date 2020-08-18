use std::rc::Rc;

#[derive(Debug)]
pub enum List<T> {
    Cons(T, Rc<List<T>>),
    Nil,
}

impl<T> List<T> {
    pub fn new(x: T) -> Self {
        Self::Cons(x, Rc::new(Self::Nil))
    }

    pub fn next(x: T, ll: Self) -> Self {
        Self::Cons(x, Rc::new(ll))
    }

    pub fn branch(x: T, ll: &Rc<Self>) -> Self {
        Self::Cons(x, Rc::clone(ll))
    }
}

pub struct Iter<'a, T>(&'a List<T>);

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if let List::Cons(v, ll) = &self.0 {
            self.0 = ll;
            Some(v)
        } else {
            None
        }
    }
}

impl<'a, T> IntoIterator for &'a List<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        Iter(self)
    }
}

fn main() {
    let tail = List::next(3, List::next(2, List::new(1)));
    let tail = Rc::new(tail);
    let head1 = List::branch(4, &tail);
    let head2 = List::branch(-4, &tail);

    assert!(head1.into_iter().eq(vec![4, 3, 2, 1].iter()));
    assert!(head2.into_iter().eq(vec![-4, 3, 2, 1].iter()));

    println!("{:?}", head1);
    println!("{:?}", head2);
}
