#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

pub mod sensors {
	include!(concat!(env!("OUT_DIR"), "/sensors.rs"));
}

pub mod error {
	include!(concat!(env!("OUT_DIR"), "/error.rs"));
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn sensors_init() {
		assert_eq!(unsafe { sensors::sensors_init(std::ptr::null_mut()) }, 0);
	}
}
