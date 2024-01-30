use rl::{self, add_two};

use crate::common::setup;

mod common;

#[test]
fn it_adds_two() {
    setup();
    assert_eq!(4, add_two(2));
}
