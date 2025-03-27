extern crate const_config_len;
use const_config_len::const_config_len;


#[test]
fn test_length_correct_from_root() {
    const LEN: usize = const_config_len!(("tests/root_len2.json", "json"));
    assert_eq!(LEN, 2);
}

#[test]
fn test_explicit_json_same_as_implicit() {
    const EXPLICIT_LEN: usize = const_config_len!(("tests/root_len2.json", "json"));
    const IMPLICIT_LEN: usize = const_config_len!("tests/root_len2.json");
    assert_eq!(EXPLICIT_LEN, IMPLICIT_LEN);
}
