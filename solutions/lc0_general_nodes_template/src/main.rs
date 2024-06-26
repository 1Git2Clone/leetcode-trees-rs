// THIS IS JUST TO SUPPRESS THE INITIAL WARNINGS!
// It's hightly recommended that you remove it if you decide to work with this template!
#![allow(dead_code)]

/// The simple library that contains the TreeNode schema.
use leetcode_trees_rs::{
    prelude::*,
    symmetric_tree,
    utils::{list_node, ListNode, TreeNode},
};

/// This empty struct needs to be declared to match LeetCode's struct implementation (method)
/// scoping.
struct Solution {}

// #region LeetCode Solutions

use std::{cell::RefCell, rc::Rc};
/// This is a personal preference. Feel free to change that part around if you wish.
type TreeNodeElement = Option<Rc<RefCell<TreeNode>>>;
type ListNodeElement = Option<Box<ListNode>>;

impl Solution {
    pub fn your_leetcode_fn() {}
}

// #endregion

// #region Test NodeTree functions

fn test_tree_node_root_with_2_branches() -> TreeNode {
    let right_child_two = TreeNode::new(6);
    let right_child_one = TreeNode {
        val: 6,
        left: None,
        right: Some(Rc::from(RefCell::from(right_child_two))),
    };
    let left_child_two = TreeNode::new(9);
    let left_child_one = TreeNode {
        val: 69,
        left: Some(Rc::from(RefCell::from(left_child_two))),
        right: None,
    };
    TreeNode {
        val: 42,
        left: Some(Rc::from(RefCell::from(left_child_one))),
        right: Some(Rc::from(RefCell::from(right_child_one))),
    }
}

/// The list_node! macro invocation is equivalent to this:
/// ```rust
/// use leetcode_trees_rs::utils::ListNode;
/// use std::boxed::Box;
///
/// ListNode {
///     val: 1,
///     next: Some(Box::new(ListNode {
///         val: 2,
///         next: Some(Box::new(ListNode {
///             val: 3,
///             next: Some(Box::new(ListNode::new(4)))
///         }))
///     }))
/// }
/// ```
/// It's very visible that the boilerplate is a bit painful to deal with, that's why the macro is
/// re-exported in the `leetcode_trees_rs::utils` module.
fn test_list_node() -> ListNode {
    list_node!(1, 2, 3, 4)
}

fn symmetric_tree_node() -> TreeNode {
    symmetric_tree!(1, 2, 3, 4)
}

// #endregion

/// Unfortunately you have to write your own tests for the unlisted solutions. The tests themselves
/// are just unique like that. Fortunately, you can make use of the basic functions to get some
/// boilerplate TreeNodes up and test around with.
fn main() -> Result<()> {
    println!("Running tests . . .");

    Ok(())
}
