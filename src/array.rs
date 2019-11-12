use std::{
	ptr, slice, marker::PhantomData, os::raw::c_void,
	ops::{ Deref, DerefMut }
};


/// A data-array implementation with a `Vec<T>` as backing
mod vec_impl {
	use super::*;
	
	//noinspection DuplicatedCode
	pub unsafe extern "C" fn dealloc<T>(array: *mut Array<T>) {
		if let Some(array) = array.as_mut() {
			if !array.object.is_null() {
				let _vec = Box::from_raw(array.object);
				array.object = ptr::null_mut();
			}
		}
	}
	pub unsafe extern "C" fn len<T>(array: *const Array<T>) -> usize {
		array.as_ref().expect("Unexpected NULL pointer")
			.object.cast::<Vec<T>>().as_ref().expect("Unexpected NULL pointer")
			.len()
	}
	pub unsafe extern "C" fn data<T>(array: *const Array<T>) -> *const T {
		array.as_ref().expect("Unexpected NULL pointer")
			.object.cast::<Vec<T>>().as_ref().expect("Unexpected NULL pointer")
			.as_ptr()
	}
	pub unsafe extern "C" fn data_mut<T>(array: *mut Array<T>) -> *mut T {
		array.as_mut().expect("Unexpected NULL pointer")
			.object.cast::<Vec<T>>().as_mut().expect("Unexpected NULL pointer")
			.as_mut_ptr()
	}
}


/// A FFI-compatible data array which can be resized and `deref`s to a slice
#[repr(C)]
pub struct Array<T> {
	/// The deallocator
	pub dealloc: unsafe extern "C" fn(*mut Self),
	/// The amount of elements in the array
	pub len: unsafe extern "C" fn(*const Self) -> usize,
	/// A pointer to the underlying memory
	pub data: unsafe extern "C" fn(*const Self) -> *const T,
	/// A mutable pointer to the underlying memory
	pub data_mut: unsafe extern "C" fn(*mut Self) -> *mut T,
	/// The underlying memory (implementation dependent)
	pub object: *mut c_void,
	_type: PhantomData<T>
}
impl<T> Array<T> {
	/// The length of the data array
	pub fn len(&self) -> usize {
		unsafe{ (self.len)(self) }
	}
}
impl<T> Deref for Array<T> {
	type Target = [T];
	fn deref(&self) -> &Self::Target {
		let len = self.len();
		let data = unsafe{ (self.data)(self) };
		unsafe{ slice::from_raw_parts(data, len) }
	}
}
impl<T> DerefMut for Array<T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		let len = self.len();
		let data = unsafe{ (self.data_mut)(self) };
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
		unsafe{ (self.dealloc)(self) }
	}
}