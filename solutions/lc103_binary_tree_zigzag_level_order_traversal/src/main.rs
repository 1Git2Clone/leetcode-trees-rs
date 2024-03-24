/**
*
* LeetCode BFS problem. Number: 103
* https://leetcode.com/problems/binary-tree-zigzag-level-order-traversal/
*
* Given the root of a binary tree, return the zigzag level order traversal of its nodes' values.
* (i.e., from left to right, then right to left for the next level and alternate between).
*
*/
/// The simple library that contains the TreeNode schema.
use leetcode_trees_rs::{prelude::*, utils::TreeNode};

/// This empty struct needs to be declared to match LeetCode's struct implementation (method)
/// scoping.
struct Solution {}

// #region LeetCode Solutions

use std::collections::VecDeque;
use std::{cell::RefCell, rc::Rc};
/// This is a personal preference. Feel free to change that part around if you wish.
type TreeNodeElement = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    /// We use a double ended queue to store the items with the appropriate level of the queue via
    /// the push_back() and pop_front() functions. Allowing us to append/remove elements from the
    /// vector on both ends in O(1) time and space complexity. You may use two vectors for a
    /// Dynamic Programming approach too. This is the BFS approach though.
    pub fn zigzag_level_order(root: TreeNodeElement) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = vec![];
        let mut queue: VecDeque<TreeNodeElement> = VecDeque::new();
        if root.is_some() {
            queue.push_back(root)
        } else {
            return ans;
        }
        while !queue.is_empty() {
            let q_size = queue.len();
            let mut items: Vec<i32> = vec![];
            for _ in 1..=q_size {
                let temp = queue.front().unwrap().clone();
                queue.pop_front();
                if let Some(node) = temp {
                    let borrowed_node = node.borrow();
                    items.push(borrowed_node.val);

                    let left_node = borrowed_node.left.clone();
                    if left_node.is_some() {
                        queue.push_back(left_node);
                    }

                    let right_node = borrowed_node.right.clone();
                    if right_node.is_some() {
                        queue.push_back(right_node);
                    }
                }
            }
            ans.push(items);
        }
        for i in (1..ans.len()).step_by(2) {
            ans[i].reverse();
        }
        ans
    }
}

// #endregion

// #region Test NodeTree functions

fn test_node_root_with_2_branches() -> TreeNode {
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

// #endregion

/// Unfortunately you have to write your own tests for the unlisted solutions. The tests themselves
/// are just unique like that. Fortunately, you can make use of the basic functions to get some
/// boilerplate TreeNodes up and test around with.
fn main() -> Result<()> {
    println!("Running tests . . .");

    println!(
        "! ! ! Test 1 ! ! !\nEXPECTED RESULT: [[42], [6, 69], [9, 6]]\nRECIEVED RESULT: {:?}\nNOTE: There's also an assert_eq!() macro for it.",
        Solution::zigzag_level_order(Some(Rc::from(RefCell::from(
            test_node_root_with_2_branches()
        ))))
    );
    assert_eq!(
        vec![vec![42], vec![6, 69], vec![9, 6]],
        Solution::zigzag_level_order(Some(Rc::from(RefCell::from(
            test_node_root_with_2_branches()
        ))))
    );
    println!("PASSED");

    println!(
        "! ! ! Test 2 ! ! !\nTesting for off by one error in value 42 as 43 via assert_ne!()\n[[43], [6, 69], [9, 6]] - purposefully wrong\n{:?}",
        Solution::zigzag_level_order(Some(Rc::from(RefCell::from(
            test_node_root_with_2_branches()
        ))))
    );
    assert_ne!(
        vec![vec![43], vec![6, 69], vec![9, 6]],
        Solution::zigzag_level_order(Some(Rc::from(RefCell::from(
            test_node_root_with_2_branches()
        ))))
    );
    println!("PASSED");

    println!("! ! ! Test 3 ! ! !\nTesting the function with no node (Option None) via assert_eq!()\nIt's supposed to return an empty [] vector.");
    assert_eq!("[]", format!("{:?}", Solution::zigzag_level_order(None)));
    println!("PASSED");

    let test_node = TreeNode::new(1);
    let test_node_result = Solution::zigzag_level_order(Some(Rc::from(RefCell::from(test_node))));
    println!(
        "! ! ! Test 4 ! ! !\nTesting function with a 1 element TreeNode with value of 1 via assert_eq!()\n{:?}",
        test_node_result
    );
    assert_eq!(vec![vec![1]], test_node_result);
    println!("PASSED");

    println!("- - - END - - -");

    Ok(())
}
