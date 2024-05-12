///////////////////////////////////////////////////////////////////////////////////////////////////
// List Node macros
///////////////////////////////////////////////////////////////////////////////////////////////////

/// ## Description
///
/// A macro to reduce the boilerplate in generating a full ListNode.
///
/// ## Example
///
/// This code:
///
/// ```rust
/// use leetcode_trees_rs::utils::list_node;
///
/// let node = list_node!(1, 2, 3, 4);
/// ```
///
/// Is the equivalent of the following:
///
/// ```rust
/// use std::boxed::Box;
/// use leetcode_trees_rs::utils::{list_node, ListNode};
///
/// let node = ListNode {
///     val: 1,
///     next: Some(Box::new(ListNode {
///         val: 2,
///         next: Some(Box::new(ListNode {
///             val: 3,
///             next: Some(Box::new(ListNode::new(4))),
///         }))
///     }))
/// };
/// assert_eq!(node, list_node!(1, 2, 3, 4));
/// ```
#[macro_export]
macro_rules! list_node {
    ($val:expr) => {
        $crate::utils::ListNode::new($val)
    };
    ($val:expr, $($rest:tt)*) => {
        {
            let mut node = $crate::utils::ListNode::new($val);
            node.next = Some(std::boxed::Box::new(list_node!($($rest)*)));
            node
        }
    };
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// Tree Node macros
///////////////////////////////////////////////////////////////////////////////////////////////////

/// TODO:
/// You can generate TreeNodes manually by utilizing tuples. The first value is a single value and
/// everything else is a tuple of `(Option<i32>, Option<i32>)`
// #[macro_export]
#[allow(unused_macros)]
macro_rules! tree {
    ($val:expr) => {
        $crate::utils::TreeNode::new($val)
    };
    ($($left:tt, $right:tt)*) => {
        $(
            if let Some(left) = $left {
                $node.left = Some(std::rc::Rc::new(std::cell::RefCell::new(tree!(
                    left
                ))));
            }
            if let Some(right) = $right {
                $node.right = Some(std::rc::Rc::new(std::cell::RefCell::new(tree!(
                    right
                ))));
            }
        )*
    };
    ($val:expr, $($left:expr, $right:expr)*) => {
        {
            let mut node = $crate::utils::TreeNode::new($val);
            $(
                tree!(node, ($left, $right));
            )*
            node
        }
    };
}

/// ## Description
///
/// A macro to reduce the boilerplate in generating symmetric binary trees.
///
/// ## Match arms
///
/// Arm 1:
/// - Takes the value as an argument.
/// - Equivalent of doing `TreeNode::new()`.
/// Arm 2:
/// - Takes the value as an argument.
/// - Also takes a sequence of left and right node values at the same time (which means they're
/// symmetric) as an argument (and builds the `TreeNode` struct with them).
///
/// ## Example usage
/// ```rust
/// use leetcode_trees_rs::utils::symmetric_tree;
/// symmetric_tree!(1, 2, 3, 4);
/// ```
/// The symmetric_tree! macro invocation is desugared to this:
/// ```rust
/// use std::{rc::Rc, cell::RefCell, boxed::Box};
///
/// use leetcode_trees_rs::utils::{symmetric_tree, TreeNode};
///
/// let node = TreeNode {
///     val: 1,
///     left: Some(Rc::new(RefCell::new(TreeNode{
///         val: 2,
///         left: Some(Rc::new(RefCell::new(TreeNode{
///             val: 3,
///             left: Some(Rc::new(RefCell::new(TreeNode{
///                 val: 4,
///                 left: None,
///                 right: None,
///             }))),
///             right: Some(Rc::new(RefCell::new(TreeNode{
///                 val: 4,
///                 left: None,
///                 right: None,
///             }))),
///         }))),
///         right: Some(Rc::new(RefCell::new(TreeNode{
///             val: 3,
///             left: Some(Rc::new(RefCell::new(TreeNode{
///                 val: 4,
///                 left: None,
///                 right: None,
///             }))),
///             right: Some(Rc::new(RefCell::new(TreeNode{
///                 val: 4,
///                 left: None,
///                 right: None,
///             }))),
///         }))),
///     }))),
///     right: Some(Rc::new(RefCell::new(TreeNode{
///         val: 2,
///         left: Some(Rc::new(RefCell::new(TreeNode{
///             val: 3,
///             left: Some(Rc::new(RefCell::new(TreeNode{
///                 val: 4,
///                 left: None,
///                 right: None,
///             }))),
///             right: Some(Rc::new(RefCell::new(TreeNode{
///                 val: 4,
///                 left: None,
///                 right: None,
///             }))),
///         }))),
///         right: Some(Rc::new(RefCell::new(TreeNode{
///             val: 3,
///             left: Some(Rc::new(RefCell::new(TreeNode{
///                 val: 4,
///                 left: None,
///                 right: None,
///             }))),
///             right: Some(Rc::new(RefCell::new(TreeNode{
///                 val: 4,
///                 left: None,
///                 right: None,
///             }))),
///         }))),
///     }))),
/// };
/// assert_eq!(node, symmetric_tree!(1, 2, 3, 4));
/// ```
/// Now you have a tree that branches all the way through the right side without having anything on
/// the left.
#[macro_export]
macro_rules! symmetric_tree {
    ($val:expr) => {
        $crate::utils::TreeNode::new($val)
    };
    ($val:expr, $($both_sides:tt)*) => {
        {
            let mut node = $crate::utils::TreeNode::new($val);
            node.left = Some(std::rc::Rc::new(std::cell::RefCell::new(symmetric_tree!(
                $($both_sides)*
            ))));
            node.right = Some(std::rc::Rc::new(std::cell::RefCell::new(symmetric_tree!(
                $($both_sides)*
            ))));
            node
        }
    };
}

/// ## Description
///
/// A macro to reduce the boilerplate in generating left-sided only binary trees.
///
/// ## Match arms
///
/// Arm 1:
/// - Takes the value as an argument.
/// - Equivalent of doing `TreeNode::new()`.
/// Arm 2:
/// - Takes the value as an argument.
/// - Also takes a sequence of left only node values as an argument (and builds the `TreeNode`
/// struct with them).
///
/// ## Example
///
/// This code:
///
/// ```rust
/// use leetcode_trees_rs::utils::left_tree;
///
/// let left_only_tree = left_tree!(1, 2, 3);
/// ```
///
/// Is equivalent to this:
///
/// ```rust
/// use std::{rc::Rc, cell::RefCell};
/// use leetcode_trees_rs::utils::{left_tree, TreeNode};
///
/// let left_only_tree = TreeNode {
///     val: 1,
///     left: Some(Rc::new(RefCell::new(TreeNode {
///         val: 2,
///         left: Some(Rc::new(RefCell::new(TreeNode {
///             val: 3,
///             left: None,
///             right: None,
///         }))),
///         right: None,
///     }))),
///     right: None,
/// };
/// assert_eq!(left_only_tree, left_tree!(1, 2, 3));
/// ```
#[macro_export]
macro_rules! left_tree {
    ($val:expr) => {
        $crate::utils::TreeNode::new($val)
    };
    ($val:expr, $($left:tt)*) => {{
        let mut node = $crate::utils::TreeNode::new($val);
        node.left = Some(std::rc::Rc::new(std::cell::RefCell::new(left_tree!($($left)*))));
        node
    }};
}

/// ## Description
///
/// A macro to reduce the boilerplate in generating right-sided only binary trees.
///
/// ## Match arms
///
/// Arm 1:
/// - Takes the value as an argument.
/// - Equivalent of doing `TreeNode::new()`.
/// Arm 2:
/// - Takes the value as an argument.
/// - Also takes a sequence of right only node values as an argument (and builds the `TreeNode`
/// struct with them).
///
/// ## Example
///
/// This code:
///
/// ```rust
/// use leetcode_trees_rs::utils::right_tree;
///
/// let right_only_tree = right_tree!(1, 2, 3);
/// ```
///
/// Is equivalent to this:
///
/// ```rust
/// use std::{rc::Rc, cell::RefCell};
/// use leetcode_trees_rs::utils::{right_tree, TreeNode};
///
/// let right_only_tree = TreeNode {
///     val: 1,
///     left: None,
///     right: Some(Rc::new(RefCell::new(TreeNode {
///         val: 2,
///         left: None,
///         right: Some(Rc::new(RefCell::new(TreeNode {
///             val: 3,
///             left: None,
///             right: None,
///         })))
///     })))
/// };
/// assert_eq!(right_only_tree, right_tree!(1, 2, 3));
/// ```
#[macro_export]
macro_rules! right_tree {
    ($val:expr) => {
        $crate::utils::TreeNode::new($val)
    };
    ($val:expr, $($right:tt)*) => {{
        let mut node = $crate::utils::TreeNode::new($val);
        node.right = Some(std::rc::Rc::new(std::cell::RefCell::new(right_tree!(
            $($right)*
        ))));
        node
    }};
}
