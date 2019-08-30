mod array;
mod rust_box;
use std::{ ptr, ffi::c_void };

pub use crate::{
	rust_box::RustBox,
	array::{ DataArray, ObjectArray, GenericArray }
};


/// A FFI object
pub struct FfiObject {
	/// The object type
	r#type: u64,
	/// The deallocator for __this__ specific instance (may be called multiple times)
	dealloc: Option<unsafe extern "C" fn(*mut FfiObject)>,
	/// A pointer to the object
	payload: *mut c_void
}
impl FfiObject {
	/// Indicates an opaque object
	pub const OPAQUE: u64 = 0x00;
	
	/// A byte array
	pub const DATA_ARRAY: u64 = 0x01;
	/// An object array
	pub const OBJECT_ARRAY: u64 = 0x02;
	/// An owned, boxed Rust object
	pub const RUST_BOX: u64 = 0x10;
	
	/// The mask for custom types
	///
	/// __Warning: you have to ensure that you don't define the same value twice for different
	/// types. If you don't need to check against the type later, use `OPAQUE`.__
	pub const CUSTOM_MASK: u64 = 1 << 63;
	
	/// Releases the payload if the struct is owned and sets it to unowned and empty in any case
	pub fn release(&mut self) {
		// Release the struct if owned
		if let Some(dealloc) = self.dealloc {
			unsafe{ dealloc(self) }
		}
		
		// Set the struct to unowned and empty
		self.dealloc = None;
		self.payload = ptr::null_mut();
	}
}
impl Drop for FfiObject {
	fn drop(&mut self) {
		self.release();
	}
}


/// A FFI result
#[repr(C)]
pub struct FfiResult {
	/// The ok variant or
	ok: FfiObject,
	err: FfiObject
}
impl FfiResult {
	/// Creates a new ok variant
	pub fn new_ok(ok: FfiObject) -> Self {
		let err = FfiObject{ r#type: FfiObject::OPAQUE, dealloc: None, payload: ptr::null_mut() };
		Self{ ok, err }
	}
	/// Creates a new error variant
	pub fn new_err(err: FfiObject) -> Self {
		let ok = FfiObject{ r#type: FfiObject::OPAQUE, dealloc: None, payload: ptr::null_mut() };
		Self{ ok, err }
	}
	
	/// Maps `self` into a Rust `Result`
	pub fn map(self) -> Result<FfiObject, FfiObject> {
		match self.err.payload.is_null() {
			false => Err(self.err),
			true => Ok(self.ok),
		}
	}
}