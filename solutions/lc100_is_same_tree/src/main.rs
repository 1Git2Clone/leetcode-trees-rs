/**
 *
 * LeetCode BFS problem. Number: 100
 * https://leetcode.com/problems/same-tree/
 *
 * Given the roots of two binary trees p and q, write a function to check if they are the same or not.
 * Two binary trees are considered the same if they are structurally identical, and the nodes have the same value.
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
    /// This solution is the long solution. It has other variants too.
    /// You can always check for .is_none() and unwrap safely afterwards.
    /// You don't even need to store verbose variable names either. I did it because I find that to
    /// be more readable.
    pub fn is_same_tree_one(p: TreeNodeElement, q: TreeNodeElement) -> bool {
        match (p, q) {
            (Some(some_p), Some(some_q)) => {
                let p_borrowed = some_p.borrow();
                let q_borrowed = some_q.borrow();
                let (l, r): (bool, bool) = (
                    Self::is_same_tree_one(p_borrowed.left.clone(), q_borrowed.left.clone()),
                    Self::is_same_tree_one(p_borrowed.right.clone(), q_borrowed.right.clone()),
                );
                p_borrowed.val == q_borrowed.val && l && r
            }
            (None, Some(_)) => false,
            (Some(_), None) => false,
            (None, None) => true,
        }
    }

    /// You can utilize the Eq trait for TreeNode and directly check for the equality too.
    pub fn is_same_tree_two(p: TreeNodeElement, q: TreeNodeElement) -> bool {
        p == q
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
        "This first test should give - TRUE\nResult (method 1): {}\nResult (method 2): {}",
        Solution::is_same_tree_one(
            Some(Rc::from(RefCell::from(test_node_root_with_two_branches()))),
            Some(Rc::from(RefCell::from(test_node_root_with_two_branches()))),
        ),
        Solution::is_same_tree_two(
            Some(Rc::from(RefCell::from(test_node_root_with_two_branches()))),
            Some(Rc::from(RefCell::from(test_node_root_with_two_branches()))),
        ),
    );
    assert_eq!(
        test_node_root_with_two_branches(),
        test_node_root_with_two_branches(),
    );

    println!(
        "This second test should give - TRUE\nResult (method 1): {}\nResult (method 2): {}",
        Solution::is_same_tree_one(None, None),
        Solution::is_same_tree_two(None, None),
    );

    println!(
        "This third test should give - FALSE\nResult (method 1): {}\nResult (method 2): {}",
        Solution::is_same_tree_one(
            Some(Rc::from(RefCell::from(test_node_root_with_two_branches()))),
            None,
        ),
        Solution::is_same_tree_two(
            Some(Rc::from(RefCell::from(test_node_root_with_two_branches()))),
            None,
        )
    );
    assert_ne!(
        Some(Rc::from(RefCell::from(test_node_root_with_two_branches()))),
        None,
    );

    println!(
        "This fourth test should give - FALSE\nResult (method 1): {}\nResult (method 2): {}",
        Solution::is_same_tree_one(
            None,
            Some(Rc::from(RefCell::from(test_node_root_with_two_branches()))),
        ),
        Solution::is_same_tree_two(
            None,
            Some(Rc::from(RefCell::from(test_node_root_with_two_branches()))),
        )
    );
    assert_ne!(
        None,
        Some(Rc::from(RefCell::from(test_node_root_with_two_branches()))),
    );

    assert_ne!(
        Solution::is_same_tree_one(
            Some(Rc::from(RefCell::from(
                second_test_node_root_with_two_branches()
            ))),
            Some(Rc::from(RefCell::from(test_node_root_with_two_branches())))
        ),
        Solution::is_same_tree_two(
            Some(Rc::from(RefCell::from(test_node_root_with_two_branches()))),
            Some(Rc::from(RefCell::from(test_node_root_with_two_branches())))
        )
    );

    Ok(())
}
