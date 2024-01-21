use std::borrow::{Borrow, BorrowMut};
use std::f32::consts::E;
use std::{fmt::Debug, ops::Add};

//Ex_1
mod odd_module {
    pub(crate) const CONSTANT: i32 = 123;
}

mod even_module {
    pub(crate) const CONSTANT: i32 = 246;
}

mod getter_function {
    use super::even_module;
    use super::odd_module;
    pub fn get_constant(value: u32) -> i32 {
        if value % 2 == 0 {
            return even_module::CONSTANT;
        } else {
            return odd_module::CONSTANT;
        }
    }
}

//Ex_2:
trait CloneAndDouble {
    fn clone_and_double(&self) -> Self;
}

impl<T: Clone + Add<Output = T>> CloneAndDouble for T {
    fn clone_and_double(&self) -> Self {
        return self.clone() + self.clone();
    }
}

//Ex_3
trait Unknown {
    fn serialize(&self) -> String;
}

impl Unknown for i32 {
    fn serialize(&self) -> String {
        i32::to_string(self)
    }
}

impl Unknown for String {
    fn serialize(&self) -> String {
        self.clone()
    }
}

impl<T: Debug> Unknown for Vec<T> {
    fn serialize(&self) -> String {
        let mut ret_val = String::new();
        for item in self.iter() {
            ret_val.push_str(format!("{:?}", item).as_str());
        }
        return ret_val;
    }
}

fn get_vec() -> Vec<Box<dyn Unknown>> {
    vec![]
}

fn print_vec(vec: &Vec<Box<dyn Unknown>>) {
    for item in vec.iter() {
        println!("{}", item.serialize());
    }
}

//Ex_4:
use std::cell::RefCell;
use std::fmt::{write, Display, Formatter};
use std::ops::Deref;
use std::rc::Rc;

struct BinIter {
    n: u128,
    l: usize,
    counter: usize,
}

impl BinIter {
    fn new(n: u128, l: usize) -> Self {
        return BinIter { n, l, counter: 0 };
    }
}

impl Iterator for BinIter {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if self.counter < self.l {
            self.counter += 1;
            let val = self.n % 2;
            self.n = (self.n - val) / 2;
            return Some(if val == 1 { true } else { false });
        }
        None
    }
}

pub fn test1() {
    for n in BinIter::new(4641312598359562305508665788689819792, 128) {
        print!("{}", if n { 1 } else { 0 })
    }
    println!();

    for n in BinIter::new(15, 4) {
        print!("{}", if n { 1 } else { 0 })
    }
    println!();

    for n in BinIter::new(375, 9) {
        print!("{}", if n { "*" } else { "_" })
    }
    println!();

    for n in BinIter::new(8978540, 16) {
        print!("{}", if n { "*" } else { "_" })
    }
}

// Ex_5
#[derive(Clone)]
struct Node<T> {
    element: T,
    prev: Option<Rc<RefCell<Node<T>>>>,
    next: Option<Rc<RefCell<Node<T>>>>,
}

struct List<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
    size: usize,
}

impl<T> PartialEq for Node<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.element == other.element
    }
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.element)
    }
}

// impl<T> PartialEq for List<T>
// where T: PartialEq {
//     fn eq(&self, other: &Self) -> bool {
//         let mut self_iter = self.head.as_ref().map(|node| Rc::clone(node));
//         let mut other_iter = other.head.as_ref().map(|node| Rc::clone(node));

//         while let (Some(node1), Some(node2)) = (self_iter.clone(), other_iter.clone()) {
//             if node1.borrow_mut().element != node2.borrow_mut().element {
//                 return false;
//             }
//             self_iter = node1.borrow_mut().next.as_ref().map(|node| Rc::clone(node));
//             other_iter = node2.borrow_mut().next.as_ref().map(|node| Rc::clone(node));
//         }

//         self_iter.is_none() && other_iter.is_none()
//     }
// }

// impl<T: PartialEq + Display + Debug + Clone + Copy> PartialEq for List<T> {
//     fn eq(&self, other: &Self) -> bool {
//         if self.size != other.size {
//             return false;
//         }
//         for (node1, node2) in self.head.iter_mut().zip(other.head.iter()) {
//             if *node1.borrow_mut() != *node2.borrow() {
//                 return false;
//             }
//         }
//         return true;
//     }
// }

impl<T> Debug for List<T>
where
    T: Display + Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut head = self.head.as_ref();
        if head.is_some() {
            let mut ret = String::new();

            while let Some(head) = head.unwrap().as_ref().borrow_mut().next.as_ref() {
                ret.push_str(format!("{}", head.as_ref().borrow_mut()).as_str());
            }

            write!(f, "List of size {}: {}", self.size, ret)
        } else {
            write!(f, "Empty List")
        }
    }
}

impl<T> List<T>
where
    T: Display + Debug,
{
    fn new() -> Self {
        return List {
            size: 0,
            head: None,
            tail: None,
        };
    }

    fn print_list(&self) {
        println!("{:?}", self)
    }

    fn push(&mut self, element: T) {
        match self.size {
            0 => {
                let new_node = Rc::new(RefCell::new(Node {
                    element,
                    prev: None,
                    next: None,
                }));
                self.head = Some(Rc::clone(&new_node));
            }
            1 => {
                let new_node = Rc::new(RefCell::new(Node {
                    element,
                    next: self.head.take(),
                    prev: self.head.take(),
                }));
                self.head = Some(Rc::clone(&new_node));

                new_node
                    .as_ref()
                    .borrow_mut()
                    .next
                    .as_ref()
                    .unwrap()
                    .as_ref()
                    .borrow_mut()
                    .next = Some(Rc::clone(&new_node));
                new_node
                    .as_ref()
                    .borrow_mut()
                    .next
                    .as_ref()
                    .unwrap()
                    .as_ref()
                    .borrow_mut()
                    .prev = Some(Rc::clone(&new_node));
            }
            _ => {
                let new_node = Rc::new(RefCell::new(Node {
                    element,
                    next: self.head.take(),
                    prev: None,
                }));
                self.head = Some(Rc::clone(&new_node));

                new_node.as_ref().borrow_mut().prev = Some(Rc::clone(
                    &new_node
                        .as_ref()
                        .borrow_mut()
                        .next
                        .as_ref()
                        .unwrap()
                        .as_ref()
                        .borrow_mut()
                        .prev
                        .as_ref()
                        .unwrap(),
                ));
                new_node
                    .as_ref()
                    .borrow_mut()
                    .next
                    .as_ref()
                    .unwrap()
                    .as_ref()
                    .borrow_mut()
                    .prev = Some(Rc::clone(&new_node));

                new_node
                    .as_ref()
                    .borrow_mut()
                    .prev
                    .as_ref()
                    .unwrap()
                    .as_ref()
                    .borrow_mut()
                    .next = Some(Rc::clone(&new_node));
            }
        }
        self.size += 1;
    }
}

pub fn list_test() {
    let mut list: List<i32> = List::new();
    list.print_list();
    println!("CIAOOOO");
    list.push(1);
    println!("CIAOOOO");
    list.push(2);
    println!("CIAOOOO");
    list.print_list();
    list.push(3);
    println!("CIAOOOO");
    list.print_list();
}
