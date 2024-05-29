# LeetCode Trees in Rust

[![Build Icon]][Build Status]&emsp;[![Docs Icon]][Docs]&emsp;[![Version Icon]][Crate]&emsp;[![License Icon]][LICENSE]&emsp;[Coverage]

[Build Icon]: https://gitlab.com/1k2s/leetcode-trees-rs/badges/main/pipeline.svg
[Build Status]: https://gitlab.com/1k2s/leetcode-trees-rs/-/pipelines
[Docs Icon]: https://docs.rs/leetcode-trees-rs/badge.svg
[Docs]: https://docs.rs/leetcode-trees-rs/latest/leetcode_trees_rs/
[Version Icon]: https://img.shields.io/crates/v/leetcode-trees-rs.svg
[Crate]: https://crates.io/crates/leetcode-trees-rs
[License Icon]: https://img.shields.io/badge/license-MIT-blue.svg
[LICENSE]: LICENSE
[Coverage]: https://gitlab.com/1k2s/leetcode-trees-rs/badges/main/coverage.svg?job=coverage

## Description

This library is made to make any LeetCoders using Rust have a better experience
at solving their LeetCode (LC) problems. It uses `cargo make` to have
reproducible sub-modules (check `leetcode-trees-rs/solutions/README.md`) as
well as implementing the definition of the binary trees on LC.

---

Quick start on using the library:

## For `TreeNode` values (Binary Trees)

```rust
use leetcode_trees_rs::{
    prelude::*,
    utils::{symmetric_tree, tree, TreeNode},
};

struct Solution {} // Assigning an empty struct to make a solution impl block.

use std::{cell::RefCell, rc::Rc};
impl Solution {
    pub fn your_leetcode_fn() {}
}

#[cfg(test)]
mod tests {
    #[test]
    fn tests() {
        // . . .
    }
}

fn main() -> Result<()> {
    // Equivalent of:
    //        1
    //    2       2
    //  3   3   3   3
    let symmetric_tree_node = symmetric_tree!(1, 2, 3);

    // Equivalent of:
    //                   1
    //         2                   #
    //    3         3         #         #
    // 4     #   #     #   #     #   #     #
    let custom_tree = tree!(&[
        vec![Some(1)],
        vec![Some(2), None],
        vec![Some(3), Some(3)],
        vec![Some(4)],
    ]);

    // If you want trees that only branch to the left or to the right then
    // there're also the `left_tree!()` and `right_tree!()` macros!
    // Those macros can help you write your test runs easier.

    Ok(())
}
```

---

## For `ListNode` values (Singly Linked Lists)

```rust
use leetcode_trees_rs::{list_node, prelude::*, utils::ListNode};

struct Solution {} // Assigning an empty struct to make a solution impl block.

use std::{cell::RefCell, rc::Rc};
impl Solution {
    pub fn your_leetcode_fn() {}
}

#[cfg(test)]
mod tests {
    #[test]
    fn tests() {
        // . . .
    }
}

fn main() -> Result<()> {
    // This is the very cumbersome manual way of writing your ListNode structs.
    let some_list = ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode::new(3))),
        })),
    };
    // And this is the easier way:
    let another_list = list_node!(1, 2, 3);

    assert_eq!(some_list, another_list);

    Ok(())
}
```

## LICENSE

The project is licensed under the MIT license.

## Extra notes

---

Additional code templating can be found in [This template file](https://github.com/1Kill2Steal/leetcode-trees-rs/blob/main/solutions/lc0_general_nodes_template/src/main.rs).

It's located at: `solutions/lc0_general_nodes_template/src/main.rs`

---

Additional documentation can be found in [docs.rs](https://docs.rs/leetcode-trees-rs/latest/leetcode_trees_rs/).
