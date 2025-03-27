extern crate const_config_size;

use const_config_size::const_config_size;

#[test]
fn test_ints_from_root() {
    const LEN: usize = const_config_size!("tests/root_len2.json");
    assert_eq!(LEN, 2);
}

#[test]
fn test_objects_from_root() {
    const LEN: usize = const_config_size!("tests/root_len3_objects.json");
    assert_eq!(LEN, 3);
}

#[test]
fn test_1level_nesting() {
    const LEN: usize = const_config_size!(("tests/nest1_len2.json", "data"));
    assert_eq!(LEN, 2);
}

#[test]
fn test_2level_nesting() {
    const LEN: usize = const_config_size!(("tests/nest2_len2.json", "data.inner"));
    assert_eq!(LEN, 2);
}