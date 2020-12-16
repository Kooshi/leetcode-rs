#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn it_works() {
        assert_eq!(true,true)
    }

    // use test::Bencher;
    // #[bench]
    // fn bench(b: &mut Bencher) {
    //     b.iter(|| )
    // }
}
struct Solution;
//Definition for a binary tree node.
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
impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut previous = i64::min_value();
        root.as_ref().map_or(true, |r| Self::check_node(&r, &mut previous))
    }

    pub fn check_node(node: &Rc<RefCell<TreeNode>>, previous: &mut i64) -> bool {
        node.borrow().left.as_ref().map_or(true, |l| Self::check_node(&l, previous))
            && Self::check(&node.borrow().val, previous)
            && node.borrow().right.as_ref().map_or(true, |r| Self::check_node(&r, previous))
    }

    pub fn check(val: &i32, prev: &mut i64) -> bool {
        let ret = *val as i64 > *prev;
        *prev = *val as i64;
        ret
    }
}