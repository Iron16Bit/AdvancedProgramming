use std::{
    borrow::{Borrow, BorrowMut},
    cell::RefCell,
    fmt::{Debug, Formatter},
    rc::Rc,
};

type NodeRef<T> = Rc<RefCell<Node<T>>>;
struct Node<T> {
    inner_value: T,
    adjacent: Vec<NodeRef<T>>,
}

impl<T> Debug for Node<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "iv: {:?}, adj: {}",
            self.inner_value,
            self.adjacent.len()
        )
    }
}

#[derive(Debug)]
struct Graph<T> {
    nodes: Vec<NodeRef<T>>,
}

pub trait SameBool {
    fn samebool(&self, other: &Self) -> bool;
}

#[derive(Debug)]
pub struct Content {
    pub i: i32,
    pub b: bool,
}

impl Content {
    pub fn new_with(i: i32, b: bool) -> Content {
        Content { i, b }
    }
}

impl PartialEq for Content {
    fn eq(&self, other: &Self) -> bool {
        self.i == other.i
    }
}

impl PartialOrd for Content {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.i.partial_cmp(&other.i) {
            ord => return ord,
        }
    }
}

impl SameBool for Content {
    fn samebool(&self, other: &Self) -> bool {
        if self.b == other.b {
            return true;
        }
        return false;
    }
}

impl<T> Graph<T> {
    fn new() -> Self {
        return Graph { nodes: Vec::new() };
    }

    fn add_node(&mut self, n: T)
    where
        T: SameBool + PartialOrd,
    {
        let new_node = Rc::new(RefCell::new(Node {
            inner_value: n,
            adjacent: Vec::new(),
        }));
        let tmp = Rc::clone(&new_node);
        let mut tmp_node = tmp.as_ref().borrow_mut();

        for node in &self.nodes {
            let actual_node = Rc::clone(node);
            let mut inner_node = actual_node.as_ref().borrow_mut();
            
            if inner_node.inner_value < tmp_node.inner_value {
                tmp_node.adjacent.push(Rc::clone(node));
            }

            if inner_node.inner_value.samebool(&tmp_node.inner_value) {
                inner_node.adjacent.push(Rc::clone(&new_node));
            }
        }

        self.borrow_mut().nodes.push(new_node);
    }
}

pub fn test1() {
    let mut el1 = Content { i: 10, b: true };
    let mut el2 = Content { i: 11, b: true };
    let mut el3 = Content { i: 11, b: false };
    assert_eq!(el1 < el2, true);
    assert_eq!(el2 < el1, false);
    assert_eq!(el2 == el3, true);
}

pub fn test2() {
    let mut el1 = Content { i: 10, b: true };
    let mut el2 = Content { i: 11, b: true };
    let mut el3 = Content { i: 11, b: false };
    assert_eq!(el1.samebool(&el2), true);
    assert_eq!(el1.samebool(&el3), false);
}

pub fn test3() {
    let mut g: Graph<Content> = Graph::new();
    println!("{:?}", g);
}

pub fn test4() {
    let mut el1 = Content { i: 10, b: true };
    let mut el2 = Content { i: 11, b: true };
    let mut el3 = Content { i: 12, b: false };
    let mut g = Graph::new();
    println!("{:?}", g);
    g.add_node(el1);
    println!("{:?}", g);
    g.add_node(el2);
    println!("{:?}", g);
    g.add_node(el3);
    println!("{:?}", g);

    let mut el1 = Content { i: 10, b: true };
    let mut el2 = Content { i: 8, b: false };
    let mut g = Graph::new();
    println!("{:?}", g);
    g.add_node(el1);
    println!("{:?}", g);
    g.add_node(el2);
    println!("{:?}", g);

    let mut el1 = Content { i: 10, b: true };
    let mut el2 = Content { i: 11, b: true };
    let mut el3 = Content { i: 12, b: true };
    let mut g = Graph::new();
    println!("{:?}", g);
    g.add_node(el1);
    println!("{:?}", g);
    g.add_node(el2);
    println!("{:?}", g);
    g.add_node(el3);
    println!("{:?}", g);

    let mut el1 = Content { i: 10, b: true };
    let mut el2 = Content { i: 9, b: false };
    let mut el3 = Content { i: 8, b: true };
    let mut g = Graph::new();
    println!("{:?}", g);
    g.add_node(el1);
    println!("{:?}", g);
    g.add_node(el2);
    println!("{:?}", g);
    g.add_node(el3);
    println!("{:?}", g);
}
