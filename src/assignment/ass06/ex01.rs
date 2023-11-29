use std::cell::{Ref, RefCell, RefMut};
use std::ops::Deref;
use std::rc::Rc;

#[derive(Clone, Debug)]
struct TreeNode <T> {
    value: T,
    left: Option<NodeRef<T>>,
    right: Option<NodeRef<T>>
}

type NodeRef<T> = Rc<RefCell<TreeNode<T>>>;

impl<T> TreeNode<T>
    where T: PartialOrd+Clone {
    fn new(value: T) -> Self {
        return TreeNode{value, left: None, right: None};
    }

    fn from_vec(vec: Vec<T>) -> Self {
        let mut first : bool = true;
        let mut tree : TreeNode<T> = TreeNode::new(vec[0].clone());
        for n in vec.iter() {
            if !first {
                tree.insert(n.clone());
            }
            first = false;
        }

        return tree;
    }

    fn insert(&mut self, value: T) {
        if value > self.value {
            if self.right.is_some() {
                TreeNode::rec_insert(self.right.as_mut().unwrap().borrow_mut(), value);
            } else {
                self.right.replace(Rc::new(RefCell::new(TreeNode::new(value))));
            }
        } else {
            if self.left.is_some() {
                TreeNode::rec_insert(self.left.as_mut().unwrap().borrow_mut(), value);
            } else {
                self.left.replace(Rc::new(RefCell::new(TreeNode::new(value))));
            }
        }
    }

    fn rec_insert(mut node: RefMut<TreeNode<T>>, value: T) {
        if value > node.value {
            if node.right.is_some() {
                TreeNode::rec_insert(node.right.as_mut().unwrap().borrow_mut(), value);
            } else {
                node.right.replace(Rc::new(RefCell::new(TreeNode::new(value))));
            }
        } else {
            if node.left.is_some() {
                TreeNode::rec_insert(node.left.as_mut().unwrap().borrow_mut(), value);
            } else {
                node.left.replace(Rc::new(RefCell::new(TreeNode::new(value))));
            }
        }
    }
}

pub fn test() {
    let mut tree1 = TreeNode::new(5);
    tree1.insert(7);
    tree1.insert(11);
    tree1.insert(8);
    tree1.insert(1);

    let tree2 = TreeNode::from_vec(vec![5,7,11,8,1]);

    println!("Tree1: \n{:?}", tree1);

    println!("Tree2: \n{:?}", tree2);
}