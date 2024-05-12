#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
/// ## Description
///
/// The struct works by having a value and an optional pointer to a next ListNode.
/// When writing the nodes you need to start from the last one and make your way down to the first
/// one.
///
/// The struct signature is entirely 1:1 with LeetCode with additional optional features like
/// serde.
///
/// ## Example
///
/// ```rust
/// use leetcode_trees_rs::utils::ListNode;
/// use std::boxed::Box;
/// use std::option::Option;
///
/// // Last node
/// let node_3 = ListNode::new(7);
///
/// // Second node
/// let node_2 = ListNode{
///     val: 6,
///     next: Some(Box::new(node_3)),
/// };
///
/// // First node
/// let node_1 = ListNode {
///     val: 5,
///     next: Some(Box::new(node_2)),
/// };
/// ```
/// Result: 5 -> 6 -> 7 -> `None`
///
/// ## Big O
///
/// Peeking/modifying the first element -> O(1)
///
/// Peeking/modifying the last element -> O(n)
///
/// Removing the first element -> O(1)
///
/// Removing any next element -> O(n)
///
/// Adding a new element at the end -> O(n)
///
/// Adding a new element at the start -> O(1)
///
/// Searching -> O(n)
#[derive(PartialEq, Eq, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ListNode {
    /// This uses normal i32 values that can be serialized and deserialized using serde if wanted.
    pub val: i32,

    /// An optional box for a next ListNode.
    #[cfg_attr(feature = "serde", serde(skip))]
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    /// Used to make a new ListNode with next as `None`.
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
