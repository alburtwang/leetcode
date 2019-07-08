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

 pub struct Solution {

 }

use std::rc::Rc;
use std::cell::RefCell;
 use core::borrow::Borrow;

 impl Solution {
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, l: i32, r: i32) -> i32 {
        match root {
            None => 0,
            Some(rt) => {

                let t: &RefCell<TreeNode> = rt.borrow();
                let vref = t.borrow();

                if l <= vref.val && vref.val <= r {
                    return Solution::range_sum_bst(vref.left.clone(), l, r)
                        + vref.val +
                        Solution::range_sum_bst(vref.right.clone(), l, r)
                }else if vref.val < l {
                    return Solution::range_sum_bst(vref.right.clone(), l, r)
                }else {
                    return Solution::range_sum_bst(vref.left.clone(), l, r)
                }
            }
        }

    }
}