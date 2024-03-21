/**
 *
 * LeetCode BFS problem. Number: 101
 * https://leetcode.com/problems/symmetric-tree/
 *
 * Given the root of a binary tree, check whether it is a mirror of itself (i.e., symmetric around
 * its center).
 *
 */
/// The simple library that contains the TreeNode schema.
use leetcode_trees_rs::{prelude::*, utils::TreeNode};

/// This empty struct needs to be declared to match LeetCode's struct implementation (method)
/// scoping.
struct Solution {}

// #region LeetCode Solutions

use std::{cell::RefCell, rc::Rc};
/// This is a personal preference. Feel free to change that part around if you wish.
type TreeNodeElement = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    fn solve(left: TreeNodeElement, right: TreeNodeElement) -> bool {
        match (left, right) {
            (Some(l), Some(r)) => {
                let (first_half, second_half): (bool, bool) = (
                    Self::solve(l.borrow().left.clone(), r.borrow().right.clone()),
                    Self::solve(l.borrow().right.clone(), r.borrow().left.clone()),
                );
                l.borrow().val == r.borrow().val && first_half && second_half
            }
            (Some(_), None) => false,
            (None, Some(_)) => false,
            (None, None) => true,
        }
    }
    pub fn is_symmetric(root: TreeNodeElement) -> bool {
        match root {
            Some(node) => Self::solve(node.borrow().left.clone(), node.borrow().right.clone()),
            None => true,
        }
    }
}

// #endregion

// #region Test NodeTree functions

fn test_node_root_with_two_branches() -> TreeNode {
    let left_child_two = TreeNode::new(9);
    let left_child_one = TreeNode {
        val: 69,
        left: Some(Rc::from(RefCell::from(left_child_two))),
        right: None,
    };
    let right_child_two = TreeNode::new(6);
    let right_child_one = TreeNode {
        val: 6,
        left: None,
        right: Some(Rc::from(RefCell::from(right_child_two))),
    };
    TreeNode {
        val: 42,
        left: Some(Rc::from(RefCell::from(left_child_one))),
        right: Some(Rc::from(RefCell::from(right_child_one))),
    }
}

fn second_test_node_root_with_two_branches() -> TreeNode {
    let left_child_two = TreeNode::new(9);
    let left_child_one = TreeNode {
        val: 70, // This is the only difference between the two.
        left: Some(Rc::from(RefCell::from(left_child_two))),
        right: None,
    };
    let right_child_two = TreeNode::new(6);
    let right_child_one = TreeNode {
        val: 6,
        left: None,
        right: Some(Rc::from(RefCell::from(right_child_two))),
    };
    TreeNode {
        val: 42,
        left: Some(Rc::from(RefCell::from(left_child_one))),
        right: Some(Rc::from(RefCell::from(right_child_one))),
    }
}

// #endregion

fn main() -> Result<()> {
    println!("Running tests . . .");

    println!(
        "This first test should give - FALSE\nResult: {}\n",
        Solution::is_symmetric(Some(Rc::from(RefCell::from(
            test_node_root_with_two_branches()
        ))),),
    );
    assert_eq!(
        test_node_root_with_two_branches(),
        test_node_root_with_two_branches(),
    );

    println!(
        "This second test should give - TRUE\nResult: {}\n",
        Solution::is_symmetric(None),
    );

    println!(
        "This third test should give - FALSE\nResult: {}\n",
        Solution::is_symmetric(Some(Rc::from(RefCell::from(
            test_node_root_with_two_branches()
        ))))
    );
    assert_ne!(
        Some(Rc::from(RefCell::from(test_node_root_with_two_branches()))),
        None,
    );

    println!(
        "This fourth test should give - FALSE\nResult: {}",
        Solution::is_symmetric(Some(Rc::from(RefCell::from(
            test_node_root_with_two_branches()
        ))),)
    );
    assert_ne!(
        None,
        Some(Rc::from(RefCell::from(test_node_root_with_two_branches()))),
    );

    assert_eq!(
        Solution::is_symmetric(Some(Rc::from(RefCell::from(
            second_test_node_root_with_two_branches()
        ))),),
        Solution::is_symmetric(Some(Rc::from(RefCell::from(
            test_node_root_with_two_branches()
        ))),)
    );

    Ok(())
}
