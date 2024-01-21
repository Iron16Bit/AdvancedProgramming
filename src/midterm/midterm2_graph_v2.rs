use std::{borrow::BorrowMut, ops::Deref};

#[derive(Debug)]
pub struct Content {
    pub i: i32,
    pub s: String,
}
impl Content {
    pub fn new(i: i32, s: String) -> Content {
        Content { i, s }
    }
}
type TreeLink<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    elem: T,
    left: TreeLink<T>,
    center: TreeLink<T>,
    right: TreeLink<T>,
}
impl<T> Node<T> {
    pub fn new(elem: T) -> Node<T> {
        Node {
            elem,
            left: None,
            center: None,
            right: None,
        }
    }
}
#[derive(Debug)]
pub struct Tree<T> {
    root: TreeLink<T>,
    size: i32,
}

//---------------------------------------------------------------------------------------------------------------------------------------------

impl<T> Tree<T> {
    fn new() -> Self {
        return Tree {
            root: None,
            size: 0,
        };
    }

    fn add(&mut self, el: T)
    where
        T: PartialOrd + PartialEq,
    {
        let root = &self.root;
        if root.is_some() {
            self.size += 1;
            rec_add(&mut self.root, el);
        } else {
            self.root = Some(Box::new(Node {
                elem: el,
                left: None,
                center: None,
                right: None,
            }));
            self.size += 1;
        }
    }

    fn howmany_smaller(&self, el: &T) -> i32
    where
        T: PartialEq + PartialOrd,
    {
        return rec_smaller(&self.root, el);
    }
}

fn rec_add<T>(node: &mut TreeLink<T>, el: T)
where
    T: PartialEq + PartialOrd,
{
    if node.is_some() {
        let actual_node = node.as_mut().unwrap();
        if &el > &actual_node.elem {
            rec_add(&mut actual_node.right, el);
        } else if &el < &actual_node.elem {
            rec_add(&mut actual_node.left, el);
        } else {
            rec_add(&mut actual_node.center, el);
        }
    } else {
        let new_node = Node {
            elem: el,
            left: None,
            center: None,
            right: None,
        };
        let _ = node.insert(Box::new(new_node));
    }
}

fn rec_smaller<T>(node: &TreeLink<T>, el: &T) -> i32
where
    T: PartialEq + PartialOrd,
{
    if node.is_some() {
        let actual_node = node.as_ref().unwrap();
        let val1 = rec_smaller(&actual_node.left, el);
        let val2 = rec_smaller(&actual_node.center, el);
        let val3 = rec_smaller(&actual_node.right, el);

        if el > &actual_node.elem {
            return 1+val1+val2+val3;
        }

        return val1+val2+val3;
    } else {
        return 0;
    }
}

impl PartialEq for Content {
    fn eq(&self, other: &Self) -> bool {
        self.s.len() == other.s.len()
    }
}

impl PartialOrd for Content {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.s.len().partial_cmp(&other.s.len())
    }
}

pub fn test1() {
    let mut t: Tree<Content> = Tree::new();
    println!("{:?}", t);
}

pub fn test2() {
    let e0 = Content::new(10, "asd".to_string());
    let e1 = Content::new(10, "who".to_string());
    let e2 = Content::new(11, "oneasd".to_string());
    assert_eq!(e1 == e2, false);
    assert_eq!(e1 == e0, true);
    assert_eq!(e1 < e2, true);
    assert_eq!(e1 > e2, false);
}

pub fn test3() {
    let mut t = Tree::new();
    let e1 = Content::new(10, "asd".to_string());
    let e2 = Content::new(11, "oneasd".to_string());
    let e3 = Content::new(8, "bhas".to_string());
    let e4 = Content::new(19, "xax".to_string());
    let e5 = Content::new(45, "gip".to_string());
    t.add(e1);
    t.add(e2);
    t.add(e3);
    t.add(e4);
    t.add(e5);
    println!("{:?}", t);
}

pub fn test4() {
    let mut t = Tree {
        root: Some(Box::new(Node {
            elem: Content::new(10, "what".to_string()),
            left: Some(Box::new(Node {
                elem: Content::new(11, "who".to_string()),
                left: None,
                center: Some(Box::new(Node {
                    elem: Content::new(19, "ten".to_string()),
                    left: None,
                    center: None,
                    right: None,
                })),
                right: None,
            })),
            center: Some(Box::new(Node {
                elem: Content::new(15, "zips".to_string()),
                left: None,
                center: None,
                right: None,
            })),
            right: Some(Box::new(Node {
                elem: Content::new(88, "whose".to_string()),
                left: None,
                center: None,
                right: None,
            })),
        })),
        size: 0,
    };
    let e56 = Content::new(45, "gips".to_string());
    let e57 = Content::new(45, "naasdasdsad".to_string());
    assert_eq!(t.howmany_smaller(&e56), 2);
    assert_eq!(t.howmany_smaller(&e57), 5);
}
