extern crate pid;

use pid::PID;

#[test]
fn check_p() {
	let mut pid = PID::new(2, 0, 0, 100);
	assert_eq!(8, pid.compute(4));
}

