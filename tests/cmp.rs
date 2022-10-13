use core::cmp::Ordering;
use ibig::{ibig, ubig};

#[test]
fn test_cmp() {
    assert_eq!(ubig!(500).cmp(&ubig!(500)), Ordering::Equa