use std::{
	ptr, any::Any,
	os::raw::{ c_void, c_char }
};


/// A Rust-box implementation with a `Box<dyn Any + 'static>` as backing
mod rust_box_impl {
	use super::*;
	
	//noinspection DuplicatedCode
	pub unsafe extern "C" fn dealloc(opaque: *mut Opaque) {
		if let Some(opaque) = opaque.as_mut() {
			if !opaque.object.is_null() {
				let _object = Box::from_raw(opaque.object);
				opaque.object = ptr::null_mut();
			}
		}
	}
	pub unsafe extern "C" fn type_hint(opaque: *const Opaque) -> *const c_char {
		opaque.as_ref().expect("Unexpected NULL pointer")
			.object.as_ref().expect("Unexpected NULL pointer");
		b"Rust::Box<dyn Any + 'static>\0".as_ptr() as *const c_char
	}
}


/// A FFI-compatible wrapper for opaque types
#[repr(C)]
pub struct Opaque {
	/// The deallocator if the object is owned
	pub dealloc: unsafe extern "C" fn(*mut Self),
	/// A pointer to a type hint (which is a NULL-terminated string)
	pub type_hint: unsafe extern "C" fn(*const Self) -> *const c_char,
	/// The underlying object (implementation dependent)
	pub object: *mut c_void
}
impl Opaque {
	/// The type hint of the opaque object
	pub fn type_hint(&self) -> *const c_char {
		unsafe{ (self.type_hint)(self) }
	}
	/// A reference to a `Box<dyn Any + 'static>` if the opaque object contains such a box
	pub fn as_rust_box(&self) -> Option<&Box<dyn Any + 'static>> {
		match self.type_hint {
			_ if self.type_hint == rust_box_impl::type_hint =>
				unsafe{ (self.object as *const Box<dyn Any + 'static>).as_ref() }
					.expect("Unexpected NULL pointer").into(),
			_ => None
		}
	}
	/// A mutable reference to a `Box<dyn Any + 'static>` if the opaque object contains such a box
	pub fn as_rust_box_mut(&mut self) -> Option<&mut Box<dyn Any + 'static>> {
		match self.type_hint {
			_ if self.type_hint == rust_box_impl::type_hint =>
				unsafe{ (self.object as *mut Box<dyn Any + 'static>).as_mut() }
					.expect("Unexpected NULL pointer").into(),
			_ => None
		}
	}
}
impl From<Box<dyn Any + 'static>> for Opaque {
	fn from(boxed: Box<dyn Any + 'static>) -> Self {
		Self {
			dealloc: rust_box_impl::dealloc,
			type_hint: rust_box_impl::type_hint,
			object: Box::into_raw(Box::new(boxed)) as *mut c_void
		}
	}
}
impl Drop for Opaque {
	fn drop(&mut self) {
		unsafe{ (self.dealloc)(self) }
	}
}
