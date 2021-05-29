// Copyright 2021 The Spyder Authors.
// Use of this source code is governed by the MIT License which can be
// found in the LICENSE file.

use std::path::PathBuf;

use spyder;

#[test]
fn test_multiplication() {
    let path = PathBuf::from("tests/test_data/multiplication-test.spd");
    let calculated = spyder::run_file(&path, false);
    assert!(calculated.is_ok());
    assert_eq!(calculated.expect("error"), 21)
}
