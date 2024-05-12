use crate::prelude::*;
use crate::utils::{list_node, ListNode};

///////////////////////////////////////////////////////////////////////////////////////////////////
// Utility
///////////////////////////////////////////////////////////////////////////////////////////////////

fn traverse_node(list_node: &mut Box<ListNode>, items: Vec<i32>) -> Result<()> {
    if items.is_empty() {
        return Err("The input vector is empty".into());
    }

    assert_eq!(items[0], list_node.val);

    let mut i: usize = 0;
    let len = items.len();
    while let Some(ref next) = list_node.next {
        i += 1;

        if i >= len {
            return Err("Out of bounds traversal.".into());
        }

        assert_eq!(items[i], next.val);

        *list_node = list_node
            .next
            .clone()
            .expect("Infalliable unwrap within while-let loop");
    }

    if i < len - 1 {
        return Err(format!(
            "{}{:?}{}{:?}{}{}",
            "The input vector: vec!",
            items,
            " is larger than the list node. Last node: ",
            list_node,
            ". At index: ",
            i,
        )
        .into());
    }

    Ok(())
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// Tests with the preset nodes
///////////////////////////////////////////////////////////////////////////////////////////////////

fn correct_traversals() -> Result<()> {
    // Three items

    let three_items_values = vec![1, 2, 3];
    let three_items_node = list_node!(
        three_items_values[0],
        three_items_values[1],
        three_items_values[2]
    );
    traverse_node(&mut Box::from(three_items_node), three_items_values)?;

    // Two items

    let two_items_values = vec![1, 2];
    let two_items_node = list_node!(two_items_values[0], two_items_values[1]);
    traverse_node(&mut Box::from(two_items_node), two_items_values)?;

    // One item

    let one_item_value = vec![1];
    let one_item_node = list_node!(one_item_value[0]);
    traverse_node(&mut Box::from(one_item_node), one_item_value)?;

    Ok(())
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// Main
///////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn main() -> Result<()> {
    correct_traversals()?;
    Ok(())
}
