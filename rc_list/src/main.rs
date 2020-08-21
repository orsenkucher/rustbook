#[derive(Debug)]
pub enum List<T> {
    Cons(Rc<RefCell<T>>, Rc<List<T>>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

impl<T> List<T> {
    pub fn value(x: T) -> Rc<RefCell<T>> {
        Rc::new(RefCell::new(x))
    }

    pub fn bind(x: Rc<RefCell<T>>, tail: Rc<Self>) -> Rc<Self> {
        Rc::new(Cons(x, tail))
    }

    pub fn new(x: T) -> Rc<Self> {
        Nil.pull(x)
    }

    pub fn branch(x: T, tail: &Rc<Self>) -> Rc<Self> {
        List::bind(Self::value(x), Rc::clone(tail))
    }

    pub fn pull(x: T, tail: Rc<Self>) -> Rc<Self> {
        List::bind(Self::value(x), tail)
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

impl<'a, T> IntoIterator for &'a List<T> {
    type Item = &'a Rc<RefCell<T>>;
    type IntoIter = Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        Iter(self)
    }
}

fn main() {
    let tail = List::new(1).pull(2).pull(3);
    // let tail = List::next(3, List::next(2, List::new(1)));
    // let tail = Rc::new(RefCell::new(tail));
    // let head1 = List::branch(4, &tail);
    // let head2 = List::branch(-4, &tail);
    let tail = Rc::new(tail);
    let value = List::value(4);

    // let head1 = List::branch(4, &tail);
    let head1 = List::branch(4, &tail);
    let head2 = List::branch(-4, &tail);

    // let other = vec![4, 3, 2, 1].into_iter();
    // let hi1 = head1.into_iter().map(|e| e.borrow()).map(|e| *e);
    // let test =

    // other.eq(hi1);
    // hi1.eq(other);

    // let o_vec: Vec<_> = other.collect();
    // let h_vec: Vec<_> = hi1.collect();
    // assert_eq!(o_vec, h_vec);

    // assert!(head1
    //     .into_iter()
    //     .map(|e| e.borrow())
    //     .eq(vec![4, 3, 2, 1].iter()));

    // assert!(head1
    //     .into_iter()
    //     .map(|e| &*e.borrow())
    //     .eq(vec![4, 3, 2, 1].iter()));

    // assert!(head2
    //     .into_iter()
    //     .map(|e| &*e.borrow())
    //     .eq(vec![-4, 3, 2, 1].iter()));

    // println!("{:?}", head1);
    // println!("{:?}", head2);

    let v = vec![1, 2, 3, 4];
    let res: Vec<_> = v.iter().filter(|e| **e % 2 == 0).collect();
    assert_eq!(res, vec![&2, &4]);

    let v = vec![1, 2, 3, 4];
    let res = v.into_iter().filter(|e| *e % 2 == 0).collect::<Vec<_>>();
    assert_eq!(res, vec![2, 4]);

    // let v = head1.into_iter().filter(|e| *e.borrow() % 2 == 0);
    // assert!(v.eq(vec![4, 2].iter()));

    // let v = head2.into_iter().filter(|e| *e.borrow() % 2 == 0);
    // assert!(v.eq(vec![-4, 2].iter()));
}
