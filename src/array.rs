use crate::FfiObject;
use std::{ mem, ptr, slice, any::TypeId, os::raw::c_void };


/// A C-compatible array
#[repr(C)]
struct Array<T: 'static> {
	ptr: *mut T,
	len: usize
}
impl<T: 'static> Array<T> {
	/// Creates a new `Array<T>` from `vec` and encapsulates it into an owned `FfiObject`
	pub fn object_from_vec(vec: Vec<T>) -> FfiObject {
		// Create boxed slice from vector
		let mut boxed_slice: Box<[T]> = vec.into_boxed_slice();
		let array = Box::new(Self{ ptr: boxed_slice.as_mut_ptr(), len: boxed_slice.len() });
		
		// Forget the slice and return the array
		mem::forget(boxed_slice);
		FfiObject {
			r#type: Self::r#type(),
			dealloc: Some(Self::dealloc),
			payload: Box::into_raw(array) as *mut c_void
		}
	}
	/// Moves the `Array<T>` out of the owned `object` and converts it back into a `Vec<T>` _if
	/// `object` was allocated by this implementation_
	pub fn object_back_into_vec(object: &mut FfiObject) -> Vec<T> {
		// Validate the object
		assert!(!object.payload.is_null());
		assert_eq!(object.r#type, Self::r#type());
		if object.dealloc != Some(Self::dealloc) { panic!("Not allocated by this implementation") }
		
		// Move the array into an owned box
		let array = unsafe{ Box::from_raw(object.payload as *mut Self) };
		object.dealloc = None;
		object.payload = ptr::null_mut();
		
		// Take the ownership over `slice`
		let slice = unsafe{ slice::from_raw_parts_mut(array.ptr, array.len) };
		unsafe{ Box::from_raw(slice) }.into_vec()
	}
	
	/// The deallocator for a `FfiObject` allocated by this implementation
	pub extern "C" fn dealloc(object: *mut FfiObject) {
		// Dereference the object and convert the array back into a vector which is then dropped
		let _vec = Self::object_back_into_vec(unsafe{ object.as_mut() }.unwrap());
	}
	
	/// The type of an FFI object with this array as payload
	pub fn r#type() -> u64 {
		match TypeId::of::<T>() {
			id if id == TypeId::of::<u8>() => FfiObject::DATA_ARRAY,
			id if id == TypeId::of::<FfiObject>() => FfiObject::OBJECT_ARRAY,
			_ => panic!("Unsupported array type")
		}
	}
}
impl<T> AsRef<[T]> for Array<T> {
	fn as_ref(&self) -> &[T] {
		unsafe{ slice::from_raw_parts(self.ptr, self.len) }
	}
}
impl<T> AsMut<[T]> for Array<T> {
	fn as_mut(&mut self) -> &mut[T] {
		unsafe{ slice::from_raw_parts_mut(self.ptr, self.len) }
	}
}


/// An interface-trait to work with `FfiObjects` containing a data array
pub type DataArray = dyn GenericArray<u8>;
/// An interface-trait to work with `FfiObjects` containing an object array
pub type ObjectArray = dyn GenericArray<FfiObject>;


/// An interface-trait to work with `FfiObjects` containing an array
///
/// __Important: Don't specialize this trait by yourself, but use the predefined types instead
/// ([`DataArray`](./type.DataArray.html) and [`ObjectArray`](type.ObjectArray.html))__
pub trait GenericArray<T: 'static> {
	/// Creates a new array from `vec` and wraps it into an owned `FfiObject`
	fn from_vec(vec: Vec<T>) -> FfiObject;
	
	/// Ensures that `object` has the correct type and is not empty
	fn check_type(object: &FfiObject) -> Option<&Self>;
	/// Ensures that `object` has the correct type and is not empty
	fn check_type_mut(object: &mut FfiObject) -> Option<&mut Self>;
	
	/// The underlying elements as slice
	fn as_slice(&self) -> &[T];
	/// The underlying elements as mutable slice
	fn as_slice_mut(&mut self) -> &mut[T];
	
	/// Moves the underlying memory from the array-payload out of the `FfiObject` and creates a
	/// `Vec<_>` _over_ it
	///
	/// _Note: this only works if the array was created using `from_vec`; otherwise this function
	/// returns `None`_
	fn move_into_vec(&mut self) -> Option<Vec<T>>;
}
impl<T: 'static> GenericArray<T> for FfiObject {
	fn from_vec(vec: Vec<T>) -> FfiObject {
		Array::object_from_vec(vec)
	}
	
	fn check_type(object: &FfiObject) -> Option<&Self> {
		match object.payload.is_null() {
			false if object.r#type == Array::<T>::r#type() => Some(object),
			_ => None
		}
	}
	fn check_type_mut(object: &mut FfiObject) -> Option<&mut Self> {
		match object.payload.is_null() {
			false if object.r#type == Array::<T>::r#type() => Some(object),
			_ => None
		}
	}
	
	fn as_slice(&self) -> &[T] {
		assert_eq!(self.r#type, Array::<T>::r#type(), "Invalid FFI object type");
		unsafe{ (self.payload as *const Array<T>).as_ref() }
			.expect("FFI object is empty").as_ref()
	}
	fn as_slice_mut(&mut self) -> &mut[T] {
		assert_eq!(self.r#type, Array::<T>::r#type(), "Invalid FFI object type");
		unsafe{ (self.payload as *mut Array<T>).as_mut() }
			.expect("FFI object is empty").as_mut()
	}
	
	fn move_into_vec(&mut self) -> Option<Vec<T>> {
		assert_eq!(self.r#type, Array::<T>::r#type(), "Invalid FFI object type");
		assert!(!self.payload.is_null(), "FFI object is emtpy");
		
		match self.dealloc {
			Some(f) if f == Array::<T>::dealloc => Some(Array::object_back_into_vec(self)),
			_ => None
		}
	}
}



