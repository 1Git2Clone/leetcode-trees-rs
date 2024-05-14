/// Contains the LeetCode `ListNode` definition and implementation.
pub mod linked_lists;

/// Contains the LeetCode `TreeNode` definition and implementation.
pub mod trees;

///////////////////////////////////////////////////////////////////////////////////////////////////
// Normal re-exports
///////////////////////////////////////////////////////////////////////////////////////////////////

/// A re-export for the the `linked_lists::ListNode` struct.
pub use linked_lists::ListNode;
/// A re-export for the `linked_lists::ListNode` struct.
pub use trees::TreeNode;

///////////////////////////////////////////////////////////////////////////////////////////////////
// Macro level re-exports
///////////////////////////////////////////////////////////////////////////////////////////////////

/// A re-export for the list_node! macro
pub use crate::list_node;
/// A re-export for the symmetric_tree!, right_tree! and left_tree! macros.
/// All of the TreeNode macros can be used to also just generate a new `TreeNode` instance without
/// expanding on it.
pub use crate::{left_tree, right_tree, symmetric_tree, tree};
