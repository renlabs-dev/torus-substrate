#[test]
fn foo() {
    let val = test_utils::new_test_ext().execute_with(|| "val");
    assert_eq!(val, "val");
}
