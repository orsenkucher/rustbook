#[derive(Debug)]
pub enum List<T> {
    Cons(Rc<RefCell<T>>, Rc<List<T>>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::{cell::RefCell, rc::Rc};

impl<T> List<T> {
    pub fn new(x: T) -> Self {
        Nil.prepend(x)
    }

    pub fn prepend(self, x: T) -> Self {
        Cons(Self::value(x), Rc::new(self))
    }

    pub fn cons(self, x: Rc<RefCell<T>>) -> Self {
        Cons(x, Rc::new(self))
    }

    pub fn value(x: T) -> Rc<RefCell<T>> {
        Rc::new(RefCell::new(x))
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter(self)
    }
}

impl<T> IntoIterator for List<T> {
    type Item = Rc<RefCell<T>>;
    type IntoIter = IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIter(Rc::new(self))
    }
}

impl<'a, T> IntoIterator for &'a List<T> {
    type Item = Rc<RefCell<T>>;
    type IntoIter = Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        Iter(self)
    }
}

pub struct IntoIter<T>(Rc<List<T>>);

impl<T> Iterator for IntoIter<T> {
    type Item = Rc<RefCell<T>>;
    fn next(&mut self) -> Option<Self::Item> {
        if let Cons(h, t) = &*self.0 {
            let h = Rc::clone(h);
            let t = Rc::clone(t);
            self.0 = t;
            Some(h)
        } else {
            None
        }
    }
}

pub struct Iter<'a, T>(&'a List<T>);

impl<T> Iterator for Iter<'_, T> {
    type Item = Rc<RefCell<T>>;
    fn next(&mut self) -> Option<Self::Item> {
        if let Cons(h, t) = self.0 {
            self.0 = t;
            Some(Rc::clone(h))
        } else {
            None
        }
    }
}

fn main() {
    let value = List::value(4);
    let tail = List::new(1).prepend(2).prepend(3).cons(Rc::clone(&value));
    let tail = Rc::new(tail);
    let list1 = Cons(List::value(5), Rc::clone(&tail));
    let value2 = List::value(-5);
    let list2 = Cons(Rc::clone(&value2), Rc::clone(&tail));

    *value.borrow_mut() *= 5;
    *value2.borrow_mut() *= 10;

    println!("{:?}", list1);
    println!("{:?}", list2);

    assert!(list1.iter().map(|e| *e.borrow()).eq(vec![5, 20, 3, 2, 1]));

    assert!(list2.iter().map(|e| *e.borrow()).eq(vec![-50, 20, 3, 2, 1]));

    assert!(list1
        .into_iter()
        .map(|e| *e.borrow())
        .filter(|e| *e % 2 == 0)
        .eq(vec![20, 2]));

    assert!(list2
        .into_iter()
        .map(|e| *e.borrow())
        .filter(|e| *e % 2 == 0)
        .eq(vec![-50, 20, 2]));
}
