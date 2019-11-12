#[test] #[cfg(not(feature = "test_build"))]
fn sentry() {
	panic!("Build with `test_build` feature enabled to compile and link the required C code");
}


#[test] #[cfg(feature = "test_build")]
fn test_header() {
	extern "C" {
		fn dummy() -> u8;
	}
	
	assert_eq!(unsafe{ dummy() }, 7);
}


#[test] #[cfg(feature = "test_build")]
fn test_array_len() {
	use crate::Array;
	
	extern "C" {
		fn array_len(array: *const Array<u8>) -> u64;
		fn array_set0(array: *mut Array<u8>);
	}
	
	let mut array = Array::from(b"Testolope".to_vec());
	assert_eq!(unsafe{ array_len(&array) }, 9);
	
	unsafe{ array_set0(&mut array) };
	assert_eq!(b"000000000", array.as_ref());
}