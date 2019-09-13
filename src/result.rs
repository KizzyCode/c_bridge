use std::{ ptr, os::raw::c_void };


/// A Rust-box implementation with a `Box<dyn Any + 'static>` as backing
mod result_impl {
	use super::*;
	
	pub extern "C" fn dealloc<T, E>(object: *mut *mut c_void) {
		// Dereference the outer pointer
		let object = unsafe{ (object as *mut *mut Result<T, E>).as_mut() }
			.expect("Unexpected NULL pointer");
		
		// Deallocate the vec and set the outer pointer to NULL
		if !object.is_null() {
			let _result = unsafe{ Box::from_raw(*object) };
			*object = ptr::null_mut();
		}
	}
	pub extern "C" fn into_ok<T, E>(object: *mut *mut c_void) -> T {
		// Dereference the outer pointer
		let object = unsafe{ (object as *mut *mut Result<T, E>).as_mut() }
			.expect("Unexpected NULL pointer");
		assert!(!object.is_null(), "Unexpected NULL pointer");
		
		// Deallocate the vec and set the outer pointer to NULL
		let result = unsafe{ Box::from_raw(*object) };
		*object = ptr::null_mut();
		match *result {
			Err(_) => panic!("Unexpected `Err`-value in result"),
			Ok(r) => r
		}
	}
	pub extern "C" fn into_err<T, E>(object: *mut *mut c_void) -> E {
		// Dereference the outer pointer
		let object = unsafe{ (object as *mut *mut Result<T, E>).as_mut() }
			.expect("Unexpected NULL pointer");
		assert!(!object.is_null(), "Unexpected NULL pointer");
		
		// Deallocate the vec and set the outer pointer to NULL
		let result = unsafe{ Box::from_raw(*object) };
		*object = ptr::null_mut();
		match *result {
			Ok(_) => panic!("Unexpected `Ok`-value in result"),
			Err(e) => e
		}
	}
	pub extern "C" fn is_ok<T, E>(object: *const c_void) -> u8 {
		let is_ok = unsafe{ (object as *const Result<T, E>).as_ref() }
			.expect("Unexpected NULL pointer")
			.is_ok();
		match is_ok {
			true => 1,
			false => 0
		}
	}
}


/// A FFI-compatible result type
#[repr(C)]
pub struct FfiResult<T, E> {
	/// The deallocator if the object is owned
	pub dealloc: extern "C" fn(*mut *mut c_void),
	/// Consumes the object and returns the underlying result
	pub into_ok: extern "C" fn(*mut *mut c_void) -> T,
	/// Consumes the object and returns the underlying error
	pub into_err: extern "C" fn(*mut *mut c_void) -> E,
	/// Indicates if the result is ok (`1`) or if it contains an error (`0`)
	pub is_ok: extern "C" fn(*const c_void) -> u8,
	/// The underlying object (implementation dependent)
	pub object: *mut c_void
}
impl<T, E> FfiResult<T, E> {
	/// Converts `self` into a Rust result
	pub fn into_result(self) -> Result<T, E> {
		self.into()
	}
}
impl<T, E> Into<Result<T, E>> for FfiResult<T, E> {
	fn into(mut self) -> Result<T, E> {
		match (self.is_ok)(self.object) {
			1 => Ok((self.into_ok)(&mut self.object)),
			0 => Err((self.into_err)(&mut self.object)),
			i => panic!("`FfiResult::is_ok` returned an invalid value ({})", i)
		}
	}
}
impl<T, E> From<Result<T, E>> for FfiResult<T, E> {
	fn from(result: Result<T, E>) -> Self {
		Self {
			dealloc: result_impl::dealloc::<T, E>,
			into_ok: result_impl::into_ok::<T, E>,
			into_err: result_impl::into_err::<T, E>,
			is_ok: result_impl::is_ok::<T, E>,
			object: Box::into_raw(Box::new(result)) as *mut c_void
		}
	}
}
impl<T, E> Drop for FfiResult<T, E> {
	fn drop(&mut self) {
		(self.dealloc)(&mut self.object)
	}
}
