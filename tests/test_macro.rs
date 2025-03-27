extern crate const_config_size;

use const_config_size::const_config_size;

#[test]
fn test_int_array_from_root() {
    const LEN: usize = const_config_size!("tests/root_len2.json");
    assert_eq!(LEN, 2);
}

#[test]
fn test_object_array_from_root() {
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

#[test]
fn test_int_from_root() {
    const LEN: usize = const_config_size!("tests/root_lit8.json");
    assert_eq!(LEN, 8);
}

#[test]
fn test_int_1level_nesting() {
    const LEN: usize = const_config_size!(("tests/nest1_lit4.json", "your_field"));
    assert_eq!(LEN, 4);
}