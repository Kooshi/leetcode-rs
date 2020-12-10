

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}

use std::rc::Rc;
use std::cell::RefCell;
type NodePointer = Rc<RefCell<TreeNode>>;
struct BSTIterator {
    first: bool,
    parents: Vec<StackNode>
}
enum StackNode {
    //Root(NodePointer),
    Left(NodePointer),
    Right(NodePointer)
}

impl StackNode {
    fn unwrap(&self) -> NodePointer {
        match self {
            StackNode::Left(n) => n.clone(),
            StackNode::Right(n) => n.clone()
        }
    }
    fn is_right(&self) -> bool {
        match self {
            StackNode::Left(n) => false,
            StackNode::Right(n) => true
        }
    }
}
/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BSTIterator {

    fn new(root: Option<NodePointer>) -> Self {
        let mut parents = Vec::new();
        if let Some(n) = root {
            parents.push(StackNode::Right(n.clone()));
        }

        let mut new = Self {
            first: true,
            parents
        };
        new.move_left();
        new
    }

    fn move_left(&mut self) {
        if self.parents.is_empty(){
            return;
        }
        let mut current = self.parents.last().unwrap().unwrap();
        let mut temp;
        loop {
            if let Some(n) = current.borrow().left.clone() {
                temp = n;
            } else {
                break;
            }
            self.parents.push(StackNode::Left(temp.clone()));
            current = temp;
        }
    }

    fn move_right(&mut self) -> bool {
        if self.parents.is_empty() {
            return false;
        }
        if let Some(n) = self.parents.last().unwrap().unwrap().borrow().right.clone() {
            self.parents.push(StackNode::Right(n));
            self.move_left();
            true
        } else {
            false
        }
    }

    fn next(&mut self) -> i32 {
        if self.first {
            self.first = false;
        } else {
            match self.parents.last().unwrap() {
                StackNode::Left(n) => {
                    if !self.move_right() {
                        self.parents.pop();
                    }
                }
                StackNode::Right(n) => {
                    if !self.move_right() {
                        while let Some(StackNode::Right(_)) = self.parents.pop() {}
                    }
                }
            }
        }
        self.parents.last().unwrap().unwrap().borrow().val
    }

    fn has_next(&self) -> bool {
        !(
            self.parents.is_empty()
                || self.parents.iter().all(|n| n.is_right())
                && self.parents.last().unwrap().unwrap().borrow().right.is_none()
        )
    }
}


 // Your BSTIterator object will be instantiated and called as such:
 // let obj = BSTIterator::new(root);
 // let ret_1: i32 = obj.next();
 // let ret_2: bool = obj.has_next();
