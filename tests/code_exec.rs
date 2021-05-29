use std::path::PathBuf;

use firstlang;

#[test]
fn test_multiplication() {
    let path = PathBuf::from("tests/test_data/multiplication-test.fl");
    let calculated = firstlang::run_file(&path, false);
    assert!(calculated.is_ok());
    assert_eq!(calculated.expect("error"), 21)
}
