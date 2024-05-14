/// Iternal macro helper functions.
#[cfg(feature = "tree_macros_internals")]
#[doc(hidden)]
pub mod helper_functions;

///////////////////////////////////////////////////////////////////////////////////////////////////
// List Node macros
///////////////////////////////////////////////////////////////////////////////////////////////////

/// ## Description
///
/// A macro to reduce the boilerplate in generating a full ListNode.
///
/// ## Match arms
///
/// Arm 1:
/// - Takes the value as an argument.
/// - Equivalent of doing `ListNode::new()`.
///
/// Arm 2:
/// - Takes the value as an argument.
/// - Also takes a sequence of left and right node values at the same time (which means they're
/// symmetric) as an argument (and builds the `ListNode` struct with them).
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

/// ## Description
///
/// You can generate TreeNodes manually by utilizing a `[Vec<Option<i32>]` data structure.
///
/// **IMPORTANT:** Whenever you have more Nodes and there's a None node above - It's up to you to
/// make sure that the nodes are matched correctly. Whenever there's a `None` value at a level
/// that's not the last one then the next level will ignore every impossible value. This means that
/// if you want this tree:
///
/// ```markdown
///        Some(100)                     // Root Node
///      /           \
///    None        Some(21)              // Left and Right nodes respectively
///                 /    \
///               None  None
/// ```
///
/// You need to call the macro like this:
///
/// ```rust
/// use std::{rc::Rc, cell::RefCell};
/// use leetcode_trees_rs::utils::{tree, TreeNode};
///
/// let tree = tree!(
///     &[
///         vec![Some(100)],
///         vec![None, Some(21)],
///         vec![Some(11), None], // **DON'T** use 4 `Option` values! The first two are inferred!
///     ]
/// );
///
/// assert_eq!(
///     tree,
///     Some(Rc::new(RefCell::new(TreeNode {
///         val: 100,
///         left: None,
///         right: Some(Rc::new(RefCell::new(TreeNode {
///             val: 21,
///             left: Some(Rc::new(RefCell::new(TreeNode::new(11)))),
///             right: None
///         })))
///     })))
/// );
/// ```
///
/// Another important note: If the `None` value is a the end of a vec like this:
///
/// ```rust
/// use std::{rc::Rc, cell::RefCell};
/// use leetcode_trees_rs::utils::{tree, TreeNode};
///
/// let tree = tree!(
///     &[
///         vec![Some(100)],
///         vec![Some(21), None], // ! You need to have that trialing None !
///         vec![Some(11)],       // Otherwise this `11` will get written to root->right rather
///                               // than of root->left->left.
///     ]
/// );
///
/// assert_eq!(
///     tree,
///     Some(Rc::new(RefCell::new(TreeNode {
///         val: 100,
///         left: Some(Rc::new(RefCell::new(TreeNode {
///             val: 21,
///             left: Some(Rc::new(RefCell::new(TreeNode::new(11)))),
///             right: None
///         }))),
///         right: None
///     })))
/// );
/// ```
///
/// ## Match arms
///
/// Arm 1:
/// - Takes the `[Vec<Option<i32>>]` data type which contains the `TreeNode` values based on the
/// description for this macro.
///
///
/// ## Additional examples
///
/// Making a tree only towards the left side:
///
/// ```rust
/// use std::{cell::RefCell, rc::Rc};
/// use leetcode_trees_rs::utils::{tree, symmetric_tree, TreeNode};
///
/// let node_left_sided = TreeNode {
///     val: 1,
///     left: Some(Rc::new(RefCell::new(TreeNode {
///         val: 2,
///         left: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
///         right: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
///     }))),
///     right: None,
/// };
/// assert_eq!(
///     Rc::new(RefCell::new(node_left_sided)),
///     tree!(
///         &[
///             vec![Some(1)],
///             vec![Some(2), None], // (!) You need to specify any trailing `None` values.
///             vec![Some(5), Some(6), /* None, None */],
///         ]
///     ).expect("Failed to generate TreeNode from [Vec<i32>]")
/// );
/// ```
///
/// Making a tree only towards the right side:
///
/// ```rust
/// use std::{cell::RefCell, rc::Rc};
/// use leetcode_trees_rs::utils::{tree, symmetric_tree, TreeNode};
///
/// let node_right_sided = TreeNode {
///     val: 1,
///     left: None,
///     right: Some(Rc::new(RefCell::new(TreeNode {
///         val: 3,
///         left: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
///         right: Some(Rc::new(RefCell::new(TreeNode::new(8)))),
///     }))),
/// };
/// assert_eq!(
///     Rc::new(RefCell::new(node_right_sided)),
///     tree!(
///         &[
///             vec![Some(1)],
///             vec![None, Some(3)],
///             // The other `None` values are inferred from their parents.
///             //
///             // **IMPORTANT:** Don't add them in because it causes errors!
///             vec![/*None, None, */ Some(7), Some(8)],
///         ]
///     ).expect("Failed to generate TreeNode from [Vec<i32>]")
/// );
/// ```
///
/// Utilizig both sides in making a tree:
///
/// ```rust
/// use std::{cell::RefCell, rc::Rc};
/// use leetcode_trees_rs::utils::{tree, symmetric_tree, TreeNode};
///
/// let node_both_sided = TreeNode {
///     val: 1,
///     left: Some(Rc::new(RefCell::new(TreeNode {
///         val: 2,
///         left: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
///         right: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
///     }))),
///     right: Some(Rc::new(RefCell::new(TreeNode {
///         val: 3,
///         left: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
///         right: Some(Rc::new(RefCell::new(TreeNode::new(8)))),
///     }))),
/// };
/// assert_eq!(
///     Rc::new(RefCell::new(node_both_sided)),
///     tree!(
///         &[
///             vec![Some(1)],
///             vec![Some(2), Some(3)],
///             vec![Some(5), Some(6), Some(7), Some(8)],
///         ]
///     ).expect("Failed to generate TreeNode from [Vec<i32>]")
/// );
/// ```
///
/// ## Performance
///
/// The way this is implemented is with depth traversal (similiar to [Breath-First
/// Search](https://en.wikipedia.org/wiki/Breadth-first_search)) so the algorithm's performance is:
///
/// Worst-case time complexity: O(|V| + |E|) = O(b<sup>d</sup>)
///
/// Worst-case space complexity: O(|V|) = O(b<sup>d</sup>)
///
/// Where:
///
/// V = Verticies and E = Edges
#[macro_export]
macro_rules! tree {
    ($items:expr) => {{
        $crate::macros::helper_functions::_build_tree($items)
    }};
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
///
/// Arm 2:
/// - Takes the value as an argument.
/// - Also takes a sequence of left and right node values at the same time (which means they're
/// symmetric) as an argument (and builds the `TreeNode` struct with them).
///
/// ## Example usage
///
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
///
/// Another way of desugaring this symmetric tree is with the tree!() macro like so:
///
/// ```rust
/// use std::{rc::Rc, cell::RefCell};
/// use leetcode_trees_rs::utils::{symmetric_tree, tree};
/// assert_eq!(
///     tree!(
///         &mut [
///             vec![Some(1)],
///             vec![Some(2), Some(2)],
///             vec![Some(3), Some(3), Some(3), Some(3)],
///             vec![Some(4), Some(4), Some(4), Some(4), Some(4), Some(4), Some(4), Some(4)],
///         ]
///     ).unwrap(),
///     Rc::new(RefCell::new(symmetric_tree!(1, 2, 3, 4))));
/// ```
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
///
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
///
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
