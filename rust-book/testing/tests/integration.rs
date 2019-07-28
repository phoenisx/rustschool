use testing;

mod common;

#[test]
fn it_odd_ones_out () {
	common::setup();
	assert_eq!(testing::odd_ones_out(&vec![11, 21, 22, 24, 25]), [11, 21, 25]);
}