mod linked_lists;
mod trees;
mod type_tests {
    /// This test DOESN'T implement the Send and Sync traits. That's because the program as a whole
    /// isn't made to be async. It's meant to just be a basic CLI app that you can check your leetcode
    /// submitions in. Send and Sync both require Arc which breaks the leetcode TreeNode signature
    /// Option<Rc<Refcell<TreeNode>>>.
    fn _is_normal<T: Sized + Unpin>() {}
    #[test]
    fn normal_types() {
        _is_normal::<crate::utils::TreeNode>();
        _is_normal::<crate::utils::ListNode>();
    }
}
