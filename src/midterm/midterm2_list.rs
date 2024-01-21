#[derive(Debug)]
pub struct List<T> {
    head: Link<T>,
    len: i32,
}
type Link<T> = Option<Box<Node<T>>>;
#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Link<T>,
}
#[derive(Debug)]
pub struct Content {
    s: String,
    b: bool,
    i: i32,
}
impl Content {
    pub fn new_with(s: String, b: bool, i: i32) -> Content {
        return Content { s, b, i };
    }
}

// --------------------------------------------------------------------------------------------------------------------------------------------

impl<T> List<T> {
    fn new() -> Self {
        return List { head: None, len: 0 };
    }

    fn get(&self, p: usize) -> Option<&T> {
        let mut actual_index = p.clone();
        let mut tmp = self.head.as_ref();
        while actual_index > 0 {
            if tmp.is_some() {
                tmp = tmp.unwrap().next.as_ref();
                actual_index -= 1;
            } else {
                return None;
            }
        }

        if tmp.is_some() {
            return Some(&tmp.unwrap().elem);
        } else {
            return None;
        }
    }

    fn add(&mut self, el: T)
    where
        T: PartialEq + PartialOrd,
    {
        if self.head.is_none() {
            let _ = self.head.insert(Box::new(Node {
                elem: el,
                next: None,
            }));
        } else {
            if &el < &self.head.as_ref().unwrap().elem {
                let next = self.head.take();
                let _ = self.head.insert(Box::new(Node { elem: el, next: next }));
            } else {
                rec_add(&mut self.head, el);
            }
        }
        self.len += 1;
    }
}

fn rec_add<T>(node: &mut Option<Box<Node<T>>>, el: T)
where
    T: PartialEq + PartialOrd,
{
    if node.as_ref().unwrap().next.is_some() {
        if &el < &node.as_ref().unwrap().next.as_ref().unwrap().elem && &el > &node.as_ref().unwrap().elem {
            let prev = node.as_mut().unwrap();
            let next = prev.next.take();

            let _ = prev.next.insert(Box::new(Node { elem: el, next: next }));
        } else {
            rec_add(&mut node.as_mut().unwrap().next, el);
        }
    } else {
        let _ = node.as_mut().unwrap().next.insert(Box::new(Node { elem: el, next: None }));
    }
}

impl PartialEq for Content {
    fn eq(&self, other: &Self) -> bool {
        self.i == other.i
    }
}

impl PartialOrd for Content {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.i.partial_cmp(&other.i)
    }
}

pub fn test1() {
    let l: List<i32> = List::new();
    println!("{:?}", l);
    assert_eq!(l.len, 0);
    let l = List {
        head: Some(Box::new(Node {
            elem: 4,
            next: None,
        })),
        len: 1,
    };
    assert_eq!(l.len, 1);
    let s: String = format!("{:?}", l);
    assert_eq!(s.contains("Vec"), false);
}

pub fn test2() {
    let l: List<i32> = List::new();
    println!("{:?}", l);
    assert_eq!(l.get(0), None);
    assert_eq!(l.get(1), None);
    let l = List {
        head: Some(Box::new(Node {
            elem: 4,
            next: None,
        })),
        len: 1,
    };
    assert_eq!(l.get(0), Some(&4));
    let s: String = format!("{:?}", l);
    assert_eq!(s.contains("Vec"), false);
}

pub fn test3() {
    let elem1 = Content::new_with("what".to_string(), true, 2);
    let elem2 = Content::new_with("this".to_string(), false, 18);
    let elem3 = Content::new_with("who".to_string(), true, 18);

    assert_eq!(elem1 < elem2, true);
    assert_eq!(elem2 < elem1, false);
    assert_eq!(elem3 == elem2, true);
}

pub fn test4() {
    let elem1 = Content::new_with("what".to_string(), true, 2);
    let elem2 = Content::new_with("this".to_string(), false, 18);
    let elem3 = Content::new_with("dope".to_string(), false, 5);
    let mut l: List<Content> = List::new();
    println!("{:?}", l);
    l.add(elem1);
    println!("{:?}", l);
    l.add(elem2);
    println!("{:?}", l);
    l.add(elem3);
    println!("{:?}", l);
    let elem4 = Content::new_with("nope".to_string(), false, 1);
    l.add(elem4);
    println!("{:?}", l);
}
