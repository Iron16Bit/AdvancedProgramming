//Es_1:

use std::cell::RefCell;
use std::fmt::{Debug, Display, Formatter};
use std::ops::Deref;
use std::rc::Rc;

struct BinIter {
    n: u128,
    l: usize,
    counter: usize,
}

impl BinIter {
    fn new(n: u128, l: usize) -> Self {
        return BinIter{n, l, counter: 0};
    }
}

impl Iterator for BinIter{
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if self.counter < self.l {
            self.counter += 1;
            let val = self.n % 2;
            self.n = (self.n - val) / 2;
            return Some(if val == 1 {true} else {false})
        }
        None
    }
}

pub fn test1() {
    for n in BinIter::new(4641312598359562305508665788689819792, 128) {
        print!("{}", if n {1} else {0})
    }
    println!();

    for n in BinIter::new(15, 4) {
        print!("{}", if n {1} else {0})
    }
    println!();

    for n in BinIter::new(375, 9) {
        print!("{}", if n {"*"} else {"_"})
    }
    println!();

    for n in BinIter::new(8978540, 16) {
        print!("{}", if n {"*"} else {"_"})
    }
}

//Es_2
struct Node<T> {
    element: T,
    prev: Option<Rc<RefCell<Node<T>>>>,
    next: Option<Rc<RefCell<Node<T>>>>
}

struct List<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
    size: usize
}

impl<T: PartialEq> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        return self.element == other.element
    }
}

impl<T: Debug> Display for Node<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", &self.element)
    }
}

impl<T: PartialEq> PartialEq for List<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.size != other.size {
            return false
        } else if self.size > 0{
            let mut iter = self.size.clone();
            let mut l1 = self.head.as_ref().unwrap().clone().borrow();
            let mut l2 = other.head.as_ref().unwrap().clone().borrow();
            while iter > 0 {
                if l1.element != l2.element {
                    return false
                }
                l1 = l1.next.as_ref().unwrap().clone().borrow();
                l2 = l2.next.as_ref().unwrap().clone().borrow();
                iter -= 1;
            }
            if l1.element != l2.element {
                return false
            } else {
                return true
            }
        } else {
            return false
        }
    }
}

impl<T: Debug> Display for List<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut l = self.head.unwrap().borrow_mut();
        let mut iter = self.size.clone();
        let mut str = String::new();

        while iter > 0 {
            iter -= 1;
            str += &format!("{}", l);
            l = l.next.as_ref().unwrap().borrow_mut();
        }

        str += &format!("{}", l);
        write!(f, "{}", str)
    }
}

impl<T: Debug> List<T> {
    fn new() -> Self {
        return List{head: None, tail: None, size: 0}
    }

    fn print_list(&self) {
        println!("{}", self)
    }

    fn push(&mut self, element: T) {
        let n = Node{element, prev: None, next: self.head.clone()};
    }
}