use crate::utils::TreeNode;
use std::{cell::RefCell, collections::VecDeque, rc::Rc};

/// Boilerplate...
#[cfg(feature = "tree_macros_internals")]
#[doc(hidden)]
fn rc_cell_tree_node(val: i32) -> Rc<RefCell<TreeNode>> {
    Rc::new(RefCell::new(TreeNode::new(val)))
}

/// This function can be inferred from the `tree!()` macro which works and is documented based on
/// it.
#[cfg(feature = "tree_macros_internals")]
#[doc(hidden)]
pub fn _build_tree(levels: &[Vec<Option<i32>>]) -> Option<Rc<RefCell<TreeNode>>> {
    if levels.is_empty() || levels[0].is_empty() || levels[0][0].is_none() {
        return None;
    }

    let root = rc_cell_tree_node(levels[0][0].unwrap());

    let mut queue = VecDeque::new();
    queue.push_back(Rc::clone(&root));

    for (i, _) in levels.iter().enumerate().skip(1) {
        let mut next_level_nodes = VecDeque::new();
        let mut curr_item = 0;

        for parent in &queue {
            if let Some(Some(left_val)) = levels[i].get(curr_item) {
                let left_child = rc_cell_tree_node(*left_val);
                parent.borrow_mut().left = Some(Rc::clone(&left_child));
                next_level_nodes.push_back(left_child);
            }
            curr_item += 1;

            if let Some(Some(right_val)) = levels[i].get(curr_item) {
                let right_child = rc_cell_tree_node(*right_val);
                parent.borrow_mut().right = Some(Rc::clone(&right_child));
                next_level_nodes.push_back(right_child);
            }
            curr_item += 1;
        }

        queue = next_level_nodes;
    }

    Some(root)
}
