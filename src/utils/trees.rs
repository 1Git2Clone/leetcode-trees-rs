#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{cell::RefCell, rc::Rc};

/// ## Description
///
/// When writing the nodes you need to start from the lowest ones and make your way up to the root
/// one.
///
/// There's no guarantees that this Binary Tree is balanced (this is important for optimizations.)
///
/// The struct signature is entirely 1:1 with LeetCode with additional optional features like
/// serde.
///
/// ## Examples
///
/// Example 1 - balanced tree:
///
/// ```rust
/// use std::{cell::RefCell, rc::Rc};
/// use leetcode_trees_rs::utils::TreeNode;
///
/// let left_node = TreeNode::new(42);
/// let right_node = TreeNode::new(42);
/// let root_node = TreeNode {
///     val: 100,
///     left: Some(Rc::from(RefCell::from(left_node))),
///     right: Some(Rc::from(RefCell::from(right_node))),
/// };
/// ```
///
/// Result:
///
/// ```markdown
///        Some(100)                     // Root Node
///      /           \
///   Some(42)     Some(42)              // Left and Right nodes respectively
///   /     \      /     \
/// None   None  None   None
/// ```
///
/// **NOTE:** Simplified `Option<Rc<RefCell<TreeNode>>>` to `Option<TreeNode.val>`.
///
/// /// Example 2 - unbalanced tree:
///
/// ```rust
/// use std::{cell::RefCell, rc::Rc};
/// use leetcode_trees_rs::utils::TreeNode;
///
/// let left_node = TreeNode::new(42);
/// let right_node = TreeNode::new(21);
/// let root_node = TreeNode {
///     val: 100,
///     left: Some(Rc::from(RefCell::from(left_node))),
///     right: Some(Rc::from(RefCell::from(right_node))),
/// };
/// ```
///
/// Result:
///
/// ```markdown
///        Some(100)                     // Root Node
///      /           \
///   Some(42)      Some(21)             // Left and Right nodes respectively
///   /     \       /     \
/// None   None   None   None
/// ```
///
/// Example 3 - unbalanced tree:
///
/// ```rust
/// use std::{cell::RefCell, rc::Rc};
/// use leetcode_trees_rs::utils::TreeNode;
///
/// // let left_node = TreeNode::new(42); // Removing the left node.
/// let right_node = TreeNode::new(21);
/// let root_node = TreeNode {
///     val: 100,
///     left: None, // Setting the left node to None.
///     right: Some(Rc::from(RefCell::from(right_node))),
/// };
/// ```
///
/// Result:
///
/// ```markdown
///        Some(100)                     // Root Node
///      /           \
///    None        Some(21)              // Left and Right nodes respectively
///                 /    \
///               None  None
/// ```
///
/// The second example is unbalanced because the depths of both sides don't match.
///
/// Example 4 - unbalanced tree:
///
/// ```rust
/// use std::{cell::RefCell, rc::Rc};
/// use leetcode_trees_rs::utils::TreeNode;
///
/// let left_right_node = TreeNode::new(16);
/// let left_node = TreeNode {
///     val: 42,
///     left: None,
///     right: Some(Rc::from(RefCell::from(left_right_node))),
/// };
/// let right_node = TreeNode::new(21);
/// let root_node = TreeNode {
///     val: 100,
///     left: Some(Rc::from(RefCell::from(left_node))),
///     right: Some(Rc::from(RefCell::from(right_node))),
/// };
/// ```
///
/// Result:
///
/// ```markdown
///         Some(100)                    // Root Node
///       /           \
///    Some(42)      Some(21)            // Left and Right nodes respectively
///    /     \        /    \
/// None   Some(16) None   None          // Left->Left; Left->Right;
///        /     \                       // Right->Left; Right->right
///     None     None
/// ```
///
/// Example 5 - balanced tree:
///
/// ```rust
/// use std::{cell::RefCell, rc::Rc};
/// use leetcode_trees_rs::utils::TreeNode;
///
/// let left_left_node = TreeNode::new(16);
/// let right_right_node = TreeNode::new(16);
/// let left_node = TreeNode {
///     val: 42,
///     left: Some(Rc::from(RefCell::from(left_left_node))),
///     right: None,
/// };
/// let right_node = TreeNode {
///     val: 42,
///     left: None,
///     right: Some(Rc::from(RefCell::from(right_right_node))),
/// };
/// let root_node = TreeNode {
///     val: 100,
///     left: Some(Rc::from(RefCell::from(left_node))),
///     right: Some(Rc::from(RefCell::from(right_node))),
/// };
/// ```
///
/// Result:
///
/// ```markdown
///              Some(100)               // Root Node
///          /              \
///       Some(42)        Some(42)       // Left and Right nodes respectively
///       /      \        /     \
///    Some(16)  None  None   Some(16)   // Left->Left; Left->Right;
///    /      \               /      \   // Right->Left; Right->right
///  None    None           None    None
/// ```
///
/// NOTE: If root->right.val != root->left.val then the tree wouldn't be balanced. The same is the
/// case if root->left->left.val != root->right->right.val.
///
/// ## Big O
///
/// Traversal (postorder, preorder and inorder) -> O(n)
///
/// Insertion at the start (above the old root) -> O(1)
///
/// Any other insertion:
/// - balanced -> O(log n)
/// - unbalanced -> O(n)
///
/// Searching:
/// - balanced -> O(log n)
/// - unbalanced -> O(n)
///
/// Deletion:
/// - balanced -> O(log n)
/// - unbalanced -> O(n)
///
/// Finding successor/predecessor:
/// - balanced -> O(log n)
/// - unbalanced -> O(n)
#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TreeNode {
    /// This uses normal i32 values that can be serialized and deserialized using serde if wanted.
    pub val: i32,

    /// An optional smart pointer contained within a reference cell. This provides very useful
    /// functionality like interior mutability. The poiter can be represented as the left child
    /// node of a binary tree.
    #[cfg_attr(feature = "serde", serde(skip))]
    pub left: Option<Rc<RefCell<TreeNode>>>,

    /// An optional smart pointer contained within a reference cell. This provides very useful
    /// functionality like interior mutability. The poiter can be represented as the right child
    /// node of a binary tree.
    #[cfg_attr(feature = "serde", serde(skip))]
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    /// Used to make a new TreeNode with next as `None`.
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
