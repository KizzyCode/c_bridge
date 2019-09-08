use crate::FfiObject;
use std::{ ptr, any::Any, convert::TryFrom, ffi::c_void };


/// Some static helper methods for working with boxes
struct BoxExt;
impl BoxExt {
	/// Wraps `inner` into another box, converts this outer box into an unowned pointer and
	/// encapsulates the pointer into an owned `FfiObject`
	pub fn from_boxed(inner: Box<dyn Any + 'static>) -> RustBox {
		let outer = Box::new(inner);
		RustBox(FfiObject {
			r#type: FfiObject::RUST_BOX,
			dealloc: Some(Self::dealloc),
			payload: Box::into_raw(outer) as *mut c_void
		})
	}
	/// Moves the outer box out of the owned `object` and converts it back into the inner box _if
	/// `object` was allocated by this implementation_
	pub fn back_into_boxed(object: &mut FfiObject) -> Box<dyn Any + 'static> {
		// Validate the object
		assert!(!object.payload.is_null());
		assert_eq!(object.r#type, FfiObject::RUST_BOX);
		if object.dealloc != Some(Self::dealloc) { panic!("Not allocated by this implementation") }
		
		// Move the array into an owned box
		let outer = unsafe{ Box::from_raw(object.payload as *mut Box<dyn Any + 'static>) };
		object.dealloc = None;
		object.payload = ptr::null_mut();
		
		// Move the inner box out of the outer box
		*outer
	}
	
	/// The deallocator for a `FfiObject` allocated by this implementation
	pub extern "C" fn dealloc(object: *mut FfiObject) {
		// Dereference the object and convert the outer box into the inner box which is then dropped
		let _box = Self::back_into_boxed(unsafe{ object.as_mut() }.unwrap());
	}
}


/// An interface to work with `FfiObjects` containing a boxed Rust object
#[repr(transparent)]
pub struct RustBox(FfiObject);
impl RustBox {
	/// The underlying element as reference
	pub fn as_ref(&self) -> &(dyn Any + 'static) {
		unsafe{ (self.0.payload as *const Box<Box<dyn Any + 'static>>).as_ref() }.unwrap()
			.as_ref().as_ref()
	}
	/// The underlying element as mutable reference
	pub fn as_mut(&mut self) -> &mut(dyn Any + 'static) {
		unsafe{ (self.0.payload as *mut Box<Box<dyn Any + 'static>>).as_mut() }.unwrap()
			.as_mut().as_mut()
	}
	
	/// Moves the underlying memory from the payload out of the `FfiObject` and creates a `Box`
	/// from it
	///
	/// _Note: this only works if the Rust object was created using `from_object` (which should
	/// almost always be the case)_
	pub fn move_into_box(mut self) -> Result<Box<dyn Any + 'static>, Self> {
		assert_eq!(self.0.r#type, FfiObject::RUST_BOX, "Invalid FFI object type");
		assert!(!self.0.payload.is_null(), "FFI object is emtpy");
		
		match self.0.dealloc {
			Some(f) if f == BoxExt::dealloc => Ok(BoxExt::back_into_boxed(&mut self.0)),
			_ => Err(self)
		}
	}
}
impl From<RustBox> for FfiObject {
	fn from(boxed: RustBox) -> Self {
		boxed.0
	}
}
impl TryFrom<FfiObject> for RustBox {
	type Error = FfiObject;
	
	/// Converts `object` into the Rust box if it has the correct type and is not empty
	fn try_from(object: FfiObject) -> Result<Self, Self::Error> {
		match object.payload.is_null() {
			false if object.r#type == FfiObject::RUST_BOX => Ok(Self(object)),
			_ => Err(object)
		}
	}
}
impl<T: Any + 'static> From<Box<T>> for RustBox {
	/// Creates a new `Box` from `object` and encapsulates it into a wrapped and owned `FfiObject`
	fn from(boxed: Box<T>) -> Self {
		BoxExt::from_boxed(boxed)
	}
}