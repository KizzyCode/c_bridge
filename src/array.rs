use std::{
	ptr, slice, marker::PhantomData, os::raw::c_void,
	ops::{ Deref, DerefMut }
};


/// A data-array implementation with a `Vec<T>` as backing
mod vec_impl {
	use super::*;
	
	pub extern "C" fn dealloc<T>(object: *mut *mut c_void) {
		// Dereference the outer pointer
		let object = unsafe{ (object as *mut *mut Vec<T>).as_mut() }
			.expect("Unexpected NULL pointer");
		
		// Deallocate the vec and set the outer pointer to NULL
		if !object.is_null() {
			let _vec = unsafe{ Box::from_raw(*object) };
			*object = ptr::null_mut();
		}
	}
	pub extern "C" fn len<T>(object: *const c_void) -> usize {
		unsafe{ (object as *const Vec<T>).as_ref() }
			.expect("Unexpected NULL pointer")
			.len()
	}
	pub extern "C" fn data<T>(object: *const c_void) -> *const T {
		unsafe{ (object as *const Vec<T>).as_ref() }
			.expect("Unexpected NULL pointer")
			.as_ptr()
	}
	pub extern "C" fn data_mut<T>(object: *mut c_void) -> *mut T {
		unsafe{ (object as *mut Vec<T>).as_mut() }
			.expect("Unexpected NULL pointer")
			.as_mut_ptr()
	}
}


/// A FFI-compatible data array which can be resized and `deref`s to a slice
#[repr(C)]
pub struct Array<T> {
	/// The deallocator if the object is owned
	pub dealloc: extern "C" fn(*mut *mut c_void),
	/// The length of the data array in bytes
	pub len: extern "C" fn(*const c_void) -> usize,
	/// A pointer to the underlying data
	pub data: extern "C" fn(*const c_void) -> *const T,
	/// A mutable pointer to the underlying data
	pub data_mut: extern "C" fn(*mut c_void) -> *mut T,
	/// The underlying object (implementation dependent)
	pub object: *mut c_void,
	_type: PhantomData<T>
}
impl<T> Array<T> {
	/// The length of the data array
	pub fn len(&self) -> usize {
		(self.len)(self.object)
	}
}
impl<T> Deref for Array<T> {
	type Target = [T];
	fn deref(&self) -> &Self::Target {
		let len = self.len();
		let data = (self.data)(self.object);
		unsafe{ slice::from_raw_parts(data, len) }
	}
}
impl<T> DerefMut for Array<T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		let len = self.len();
		let data = (self.data_mut)(self.object);
		unsafe{ slice::from_raw_parts_mut(data, len) }
	}
}
impl<T> From<Vec<T>> for Array<T> {
	fn from(vec: Vec<T>) -> Self {
		Self {
			dealloc: vec_impl::dealloc::<T>,
			len: vec_impl::len::<T>,
			data: vec_impl::data::<T>,
			data_mut: vec_impl::data_mut::<T>,
			object: Box::into_raw(Box::new(vec)) as *mut c_void,
			_type: PhantomData
		}
	}
}
impl<T> Drop for Array<T> {
	fn drop(&mut self) {
		(self.dealloc)(&mut self.object)
	}
}