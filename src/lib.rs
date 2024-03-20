//! # Introduction
//!
//! LeetCode Trees's purpose is making LeetCode problems easier to work around with via some simple
//! library exports that allow for a smoother development experience by allowing you to use your
//! primary IDE rather than the LeetCode website to solve the problems.
//!
//! ## Installation
//!
//! These are the two ways to can add this library to your project:
//! Number 1 - Via the `Cargo.toml` file by adding this;
//!
//! ```toml
//! [dependencies]
//! leetcode-trees-rs = "0.1"
//! ```
//!
//! Number 2 - Via running the following command in your terminal:
//!
//! ```sh
//! cargo add leetcode-trees-rs
//! ```
//!
//! ## Usage
//!
//! The usage is very simple. These are the primary components:
//! - [`prelude`]
//! - [`utils::TreeNode`]
//!
//!
//! Additionally, via the support of the cargo make library you can comfortably run your LeetCode
//! tests by running:
//!
//! ```sh
//! cargo make leetcode_problem_number
//! ```
//!
//! Assuming there's a LeetCode submittion for it. All the solutions are managed on the GitHub page
//! of this libray - [`solutions`].
//!
//!
//!
//! [`prelude`]: crate::prelude
//! [solutions]: https://github.com/1Kill2Steal/leetcode-trees-rs/tree/main/solutions
//! [`TreeNode`]: crate::utils::TreeNode

pub use crate::prelude::*;

pub mod error;
pub mod prelude;
pub mod utils;
