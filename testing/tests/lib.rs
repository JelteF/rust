extern crate testing;
use testing::add_three_times_four;

#[test]
fn math_checks_out() {
    let result = add_three_times_four(5i);

    assert_eq!(32i, result);
}
