#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{cell::RefCell, rc::Rc};

/// The TreeNode signature is entirely 1:1 with LeetCode with additional optional features like serde.
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
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}