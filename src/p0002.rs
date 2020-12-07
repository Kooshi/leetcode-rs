
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}
struct Solution;

impl Solution {
  pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    fn add2(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>, carry: i32) -> Option<Box<ListNode>> {
      match (l1, l2) {
        (Some(x), Some(y)) => {
          let v = carry + x.val + y.val;
          let c = v / 10;
          let val = v % 10;
          Some(Box::new(ListNode { val, next: add2(x.next, y.next, c) }))
        }
        (None, Some(y)) => {
          let v = carry + y.val;
          let c = v / 10;
          let val = v % 10;
          Some(Box::new(ListNode { val, next: add2(None, y.next, c) }))
        }
        (Some(x), None) => {
          let v = carry + x.val;
          let c = v / 10;
          let val = v % 10;
          Some(Box::new(ListNode { val, next: add2(x.next, None, c) }))
        }
        (None, None) => if carry != 0 { Some(Box::new(ListNode::new(1))) } else { None }
      }
    }
    add2(l1, l2, 0)
  }
}