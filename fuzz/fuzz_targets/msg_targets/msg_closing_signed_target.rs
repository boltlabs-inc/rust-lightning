// This file is auto-generated by gen_target.sh based on msg_target_template.txt
// To modify it, modify msg_target_template.txt and run gen_target.sh instead.

extern crate lightning;

use lightning::ln::msgs;
use lightning::util::reset_rng_state;

mod utils;
use utils::VecWriter;

#[inline]
pub fn do_test(data: &[u8]) {
	reset_rng_state();
	test_msg!(msgs::ClosingSigned, data);
}

#[cfg(feature = "afl")]
#[macro_use] extern crate afl;
#[cfg(feature = "afl")]
fn main() {
	fuzz!(|data| {
		do_test(data);
	});
}

#[cfg(feature = "honggfuzz")]
#[macro_use] extern crate honggfuzz;
#[cfg(feature = "honggfuzz")]
fn main() {
	loop {
		fuzz!(|data| {
			do_test(data);
		});
	}
}

extern crate hex;
#[cfg(test)]
mod tests {
	#[test]
	fn duplicate_crash() {
		super::do_test(&::hex::decode("00").unwrap());
	}
}
