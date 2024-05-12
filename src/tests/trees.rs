use std::cell::RefCell;
use std::rc::Rc;

use crate::prelude::*;
use crate::utils::{symmetric_tree, TreeNode};

///////////////////////////////////////////////////////////////////////////////////////////////////
// Utility
///////////////////////////////////////////////////////////////////////////////////////////////////

fn rc_cell_node(val: i32) -> Rc<RefCell<TreeNode>> {
    Rc::from(RefCell::from(TreeNode::new(val)))
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// Preset nodes
///////////////////////////////////////////////////////////////////////////////////////////////////

fn asymmetric_values_tree(left_val: i32, right_val: i32) -> TreeNode {
    let left_node = TreeNode::new(left_val);
    let right_node = TreeNode::new(right_val);
    TreeNode {
        val: 100,
        left: Some(Rc::from(RefCell::from(left_node))),
        right: Some(Rc::from(RefCell::from(right_node))),
    }
}

fn asymmetric_tree_no_right(val: i32) -> TreeNode {
    TreeNode {
        val: 100,
        left: Some(rc_cell_node(val)),
        right: None,
    }
}

fn asymmetric_tree_no_left(val: i32) -> TreeNode {
    TreeNode {
        val: 100,
        left: None,
        right: Some(rc_cell_node(val)),
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// Tests with the preset nodes
///////////////////////////////////////////////////////////////////////////////////////////////////

fn test_symmetric_tree() -> Result<()> {
    let symmetric_tree = symmetric_tree!(100, 50, 123, 124, 124, 1, 24, 12, 421, 41, 42, 14, 12, 4);

    match (symmetric_tree.left, symmetric_tree.right) {
        (Some(left_node), Some(right_node)) => {
            assert_eq!(left_node, right_node);
        }
        (Some(_), None) => {
            return Err("Missing right_node".into());
        }
        (None, Some(_)) => {
            return Err("Missing left_node".into());
        }
        _ => {
            return Err("Missing both left_node and right_node".into());
        }
    }

    Ok(())
}

fn test_asymmetric_values_tree() -> Result<()> {
    let left_val = 42;
    let right_val = 21;

    let asymmetric_tree = asymmetric_values_tree(left_val, right_val);

    match (asymmetric_tree.left, asymmetric_tree.right) {
        (Some(left_node), Some(right_node)) => {
            assert_eq!(left_node, rc_cell_node(left_val));
            assert_eq!(right_node, rc_cell_node(right_val));

            assert_ne!(left_node, right_node);
        }
        (Some(_), None) => {
            return Err("Missing right_node".into());
        }
        (None, Some(_)) => {
            return Err("Missing left_node".into());
        }
        _ => {
            return Err("Missing both left_node and right_node".into());
        }
    }

    Ok(())
}

fn test_asymmetric_depth_trees() -> Result<()> {
    let asymmetric_nodes_values = 50;

    let asymmetric_tree_no_right = asymmetric_tree_no_right(asymmetric_nodes_values);
    let asymmetric_tree_no_left = asymmetric_tree_no_left(asymmetric_nodes_values);

    match (
        asymmetric_tree_no_right.left,
        asymmetric_tree_no_right.right,
    ) {
        (Some(_), Some(_)) => {
            return Err("There shouldn't be a right_node".into());
        }
        (Some(left_node), None) => {
            assert_eq!(left_node, rc_cell_node(asymmetric_nodes_values));
        }
        (None, Some(_)) => {
            return Err("Missing left_node and there shouldn't be a right node".into());
        }
        _ => {
            return Err("There should just be a right_node but there're no nodes".into());
        }
    }

    match (asymmetric_tree_no_left.left, asymmetric_tree_no_left.right) {
        (Some(_), Some(_)) => {
            return Err("There shouldn't be a left_node".into());
        }
        (Some(_), None) => {
            return Err("Missing right_node and there shouldn't be a left node".into());
        }
        (None, Some(right_node)) => {
            assert_eq!(right_node, rc_cell_node(asymmetric_nodes_values));
        }
        _ => {
            return Err("There should just be a left_node but there're no nodes".into());
        }
    }

    Ok(())
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// Main
///////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn main() -> Result<()> {
    test_symmetric_tree()?;
    test_asymmetric_values_tree()?;
    test_asymmetric_depth_trees()?;
    Ok(())
}
