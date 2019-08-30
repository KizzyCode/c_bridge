use crate::FfiObject;
use std::{ ptr, any::Any, ffi::c_void };


/// Some static helper methods for working with boxes
struct BoxExt;
impl BoxExt {
	/// Wraps `inner` into another box, converts this outer box into an unowned pointer and
	/// encapsulates the pointer into an owned `FfiObject`
	pub fn object_from_boxed(inner: Box<dyn Any + 'static>) -> FfiObject {
		let outer = Box::new(inner);
		FfiObject {
			r#type: FfiObject::RUST_BOX,
			dealloc: Some(Self::dealloc),
			payload: Box::into_raw(outer) as *mut c_void
		}
	}
	/// Moves the outer box out of the owned `object` and converts it back into the inner box _if
	/// `object` was allocated by this implementation_
	pub fn object_back_into_boxed(object: &mut FfiObject) -> Box<dyn Any + 'static> {
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
		let _box = Self::object_back_into_boxed(unsafe{ object.as_mut() }.unwrap());
	}
}


/// An interface-trait to work with `FfiObjects` containing a boxed Rust object
pub trait RustBox {
	/// Creates a new `Box` from `object` and wraps it into an owned `FfiObject`
	fn from_object(object: impl Any + 'static) -> FfiObject;
	
	/// Ensures that `object` has the correct type and is not empty
	fn check_type(object: &FfiObject) -> Option<&Self>;
	/// Ensures that `object` has the correct type and is not empty
	fn check_type_mut(object: &mut FfiObject) -> Option<&mut Self>;
	
	/// The underlying element as reference
	fn as_ref(&self) -> &(dyn Any + 'static);
	/// The underlying element as mutable reference
	fn as_mut(&mut self) -> &mut(dyn Any + 'static);
	
	/// Moves the underlying memory from the payload out of the `FfiObject` and creates a `Box`
	/// _over_ it
	///
	/// _Note: this only works if the Rust object was created using `from_object` (which should
	/// almost always be the case); otherwise this function returns `None`_
	fn move_into_box(&mut self) -> Option<Box<dyn Any + 'static>>;
}
impl RustBox for FfiObject {
	fn from_object(object: impl Any + 'static) -> FfiObject {
		BoxExt::object_from_boxed(Box::new(object))
	}
	
	fn check_type(object: &FfiObject) -> Option<&Self> {
		match object.payload.is_null() {
			false if object.r#type == FfiObject::RUST_BOX => Some(object),
			_ => None
		}
	}
	fn check_type_mut(object: &mut FfiObject) -> Option<&mut Self> {
		match object.payload.is_null() {
			false if object.r#type == FfiObject::RUST_BOX => Some(object),
			_ => None
		}
	}
	
	fn as_ref(&self) -> &(dyn Any + 'static) {
		assert_eq!(self.r#type, FfiObject::RUST_BOX, "Invalid FFI object type");
		unsafe{ (self.payload as *const Box<Box<dyn Any + 'static>>).as_ref() }
			.expect("FFI object is empty").as_ref().as_ref()
	}
	fn as_mut(&mut self) -> &mut(dyn Any + 'static) {
		assert_eq!(self.r#type, FfiObject::RUST_BOX, "Invalid FFI object type");
		unsafe{ (self.payload as *mut Box<Box<dyn Any + 'static>>).as_mut() }
			.expect("FFI object is empty").as_mut().as_mut()
	}
	
	fn move_into_box(&mut self) -> Option<Box<dyn Any + 'static>> {
		assert_eq!(self.r#type, FfiObject::RUST_BOX, "Invalid FFI object type");
		assert!(!self.payload.is_null(), "FFI object is emtpy");
		
		match self.dealloc {
			Some(f) if f == BoxExt::dealloc => Some(BoxExt::object_back_into_boxed(self)),
			_ => None
		}
	}
}