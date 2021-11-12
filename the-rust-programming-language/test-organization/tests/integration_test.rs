use test_organization;

mod common;
/*
 * A integration test tests the same way the library is used. It
 * lives in the `/tests` directory.
 */
#[test]
fn it_adds_two() {
    // `/tests/common` directory is special. Compiler treats it as a module
    // that can be reused in test functions.
    common::setup();
    assert_eq!(test_organization::add_two(2), 4);
}
