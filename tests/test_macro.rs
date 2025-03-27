extern crate const_config_len;

use const_config_len::const_config_len;

#[test]
fn test_correct_from_root() {
    const LEN: usize = const_config_len!("tests/root_len2.json");
    assert_eq!(LEN, 2);
}

#[test]
fn test_1level_nesting() {
    const LEN: usize = const_config_len!(("tests/nest1_len2.json", "data"));
    assert_eq!(LEN, 2);
}

#[test]
fn test_2level_nesting() {
    const LEN: usize = const_config_len!(("tests/nest2_len2.json", "data.inner"));
    assert_eq!(LEN, 2);
}