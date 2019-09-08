use crate::FfiObject;
use std::{ mem, ptr, slice, any::TypeId, convert::TryFrom, marker::PhantomData, os::raw::c_void };


/// A C-compatible array
#[repr(C)]
struct Array<T: 'static> {
	ptr: *mut T,
	len: usize
}
impl<T: 'static> Array<T> {
	/// Creates a new `Array<T>` from `vec` and encapsulates it into a wrapped and owned `FfiObject`
	pub fn from_vec(vec: Vec<T>) -> GenericArray<T> {
		// Create boxed slice from vector
		let mut boxed_slice: Box<[T]> = vec.into_boxed_slice();
		let array = Box::new(Self{ ptr: boxed_slice.as_mut_ptr(), len: boxed_slice.len() });
		
		// Forget the slice and return the array
		mem::forget(boxed_slice);
		GenericArray(FfiObject {
			r#type: Self::r#type(),
			dealloc: Some(Self::dealloc),
			payload: Box::into_raw(array) as *mut c_void
		}, PhantomData)
	}
	/// Moves the `Array<T>` out of the owned `object` and converts it back into a `Vec<T>` _if
	/// `object` was allocated by this implementation (panics otherwise)_
	pub fn back_into_vec(object: &mut FfiObject) -> Vec<T> {
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
		let _vec = Self::back_into_vec(unsafe{ object.as_mut() }.unwrap());
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


/// An interface to work with `FfiObjects` containing an array
///
/// __Important: You should not specialize this trait by yourself; use the predefined types instead
/// ([`DataArray`](./type.DataArray.html) and [`ObjectArray`](type.ObjectArray.html))__
#[repr(transparent)]
pub struct GenericArray<T: 'static>(FfiObject, PhantomData<T>);
impl<T: 'static> GenericArray<T> {
	/// The underlying elements as slice
	pub fn as_slice(&self) -> &[T] {
		unsafe{ (self.0.payload as *const Array<T>).as_ref() }.unwrap().as_ref()
	}
	/// The underlying elements as mutable slice
	pub fn as_slice_mut(&mut self) -> &mut[T] {
		unsafe{ (self.0.payload as *mut Array<T>).as_mut() }.unwrap().as_mut()
	}
	
	/// Moves the underlying memory from the array-payload out of the underlying `FfiObject` and
	/// creates a `Vec<_>` from it
	///
	/// _Note: this only works if the array was created using `from_vec`_
	pub fn move_into_vec(mut self) -> Result<Vec<T>, Self> {
		match self.0.dealloc {
			Some(f) if f == Array::<T>::dealloc => Ok(Array::back_into_vec(&mut self.0)),
			_ => Err(self)
		}
	}
}
impl<T: 'static> From<GenericArray<T>> for FfiObject {
	fn from(array: GenericArray<T>) -> Self {
		array.0
	}
}
impl<T: 'static> TryFrom<FfiObject> for GenericArray<T> {
	type Error = FfiObject;
	
	/// Converts `object` into the array if it has the correct type and is not empty
	fn try_from(object: FfiObject) -> Result<Self, Self::Error> {
		match object.payload.is_null() {
			false if object.r#type == Array::<T>::r#type() => Ok(Self(object, PhantomData)),
			_ => Err(object)
		}
	}
}


/// An interface to work with `FfiObjects` containing a data array
pub type DataArray = GenericArray<u8>;
impl From<Vec<u8>> for GenericArray<u8> {
	/// Creates a new array from `vec` and wraps it into a wrapped and owned `FfiObject`
	fn from(vec: Vec<u8>) -> Self {
		Array::from_vec(vec)
	}
}


/// An interface to work with `FfiObjects` containing an object array
pub type ObjectArray = GenericArray<FfiObject>;
impl From<Vec<FfiObject>> for GenericArray<FfiObject> {
	/// Creates a new array from `vec` and wraps it into a wrapped and owned `FfiObject`
	fn from(vec: Vec<FfiObject>) -> Self {
		Array::from_vec(vec)
	}
}