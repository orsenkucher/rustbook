// #![feature(associated_type_defaults)]

#[derive(Debug)]
pub enum List<T> {
    Cons(Rc<RefCell<T>>, Rc<List<T>>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::{ops::Deref, rc::Rc};

impl<T> List<T> {
    pub fn new(x: T) -> Self {
        Nil.pull(x)
    }

    pub fn pull(self, x: T) -> Self {
        Cons(Self::value(x), Rc::new(self))
    }

    pub fn value(x: T) -> Rc<RefCell<T>> {
        Rc::new(RefCell::new(x))
    }
}

impl<'a, T> IntoIterator for &'a List<T> {
    type Item = &'a Rc<RefCell<T>>;
    type IntoIter = Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        Iter(self)
    }
}

#[derive(Debug)]
pub struct RcList<T>(Rc<List<T>>);

impl<T> RcList<T> {
    pub fn new(list: List<T>) -> Self {
        Self(Rc::new(list))
    }

    pub fn bind(self, x: Rc<RefCell<T>>) -> Self {
        Self::new(Cons(x, self.0))
    }

    pub fn branch(&self, x: Rc<RefCell<T>>) -> Self {
        Self::clone(&self).bind(x)
    }
}

impl<T> Clone for RcList<T> {
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }
}

impl<T> Deref for RcList<T> {
    type Target = Rc<List<T>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct Iter<'a, T>(&'a List<T>);

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a Rc<RefCell<T>>;
    fn next(&mut self) -> Option<Self::Item> {
        if let Cons(v, ll) = self.0 {
            self.0 = ll;
            Some(v)
        } else {
            None
        }
    }
}

fn main() {
    let value = List::value(4);
    let tail = List::new(1).pull(2).pull(3);
    let tail = RcList::new(tail).bind(Rc::clone(&value));
    let list1 = tail.branch(List::value(5));
    let value2 = List::value(-5);
    let list2 = tail.branch(Rc::clone(&value2));

    *value.borrow_mut() *= 5;
    *value2.borrow_mut() *= 10;

    println!("{:?}", list1);
    println!("{:?}", list2);

    assert!(list1
        .into_iter()
        .map(|e| *e.borrow())
        .eq(vec![5, 20, 3, 2, 1]));

    assert!(list2
        .into_iter()
        .map(|e| *e.borrow())
        .eq(vec![-50, 20, 3, 2, 1]));

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
