// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32, left: Option<Rc<RefCell<TreeNode>>>, right: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    Some(Rc::new(RefCell::new( TreeNode {
      val,
      left,
      right
    })))
  }
}
#[cfg(test)]
mod tests {
    use crate::dailies::d20_12_12::{Solution, TreeNode};
    use std::rc::Rc;
    use std::cell::RefCell;

    #[test]
    fn it_works() {
        assert_eq!(2, Solution::subtree_with_all_deepest(
            TreeNode::new(3,
              TreeNode::new(5,
                TreeNode::new(6, None, None),
                TreeNode::new(2,
                  TreeNode::new(7, None, None),
                  TreeNode::new(4, None, None))),
              TreeNode::new(1,
                TreeNode::new(0, None, None),
                TreeNode::new(8, None, None)))
        ).unwrap().borrow().val);
    }
}


struct Solution;

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn subtree_with_all_deepest(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return root;
        }
        let mut deepest_leaves = Vec::new();
        let mut max_depth = 0;
        Solution::traverse(1, None, &root.unwrap(), &mut deepest_leaves, &mut max_depth);

        //moveup on all leaves until they hit same value
        let mut val = deepest_leaves[0].as_ref().unwrap().node.borrow().val;
        while !deepest_leaves[1..].iter().all(|l|l.as_ref().unwrap().node.borrow().val == val) {
            for i in 0..deepest_leaves.len() {
                deepest_leaves[i] = deepest_leaves[i].as_ref().unwrap().parent.clone();
            }
            val = deepest_leaves[0].as_ref().unwrap().node.borrow().val;
        }

        Some(deepest_leaves[0].as_ref().unwrap().node.clone())
    }

    fn traverse(depth:u32, parent:Option<Rc<UpNode>>, node:&NodePointer, deepest_leaves:&mut Vec<Option<Rc<UpNode>>>, max_depth:&mut u32){
        let this_node = Some(Rc::new(UpNode{node:node.clone(), parent }));
        let node = node.borrow();
        if node.left.is_none() && node.right.is_none(){
            if depth >= *max_depth {
                if depth > *max_depth {
                    *max_depth = depth;
                    deepest_leaves.clear();
                }
                deepest_leaves.push(this_node);
            }
            return;
        }

        if let Some(l) = &node.left {
            Solution::traverse(depth + 1, this_node.clone(), l, deepest_leaves, max_depth);
        }

        if let Some(r) = &node.right {
            Solution::traverse(depth + 1, this_node.clone(), r, deepest_leaves, max_depth);
        }
    }
}
type NodePointer = Rc<RefCell<TreeNode>>;
struct UpNode {
    node:NodePointer,
    parent:Option<Rc<UpNode>>
}