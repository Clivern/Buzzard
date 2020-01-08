pub mod another_mod {
    pub fn another_pub_fun() -> String {
        "Hello".to_string()
    }
}

#[test]
fn test_another_pub_fun() {
    assert_eq!(
        another_mod::another_pub_fun(),
        "Hello".to_string()
    );
}
