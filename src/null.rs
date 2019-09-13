/// A FFI compatible NULL type
#[repr(C)] #[derive(Default)]
pub struct Null {
	_dummy: u8
}
impl Null {
	/// A new NULL object
	pub fn null() -> Self {
		Self{ _dummy: 0 }
	}
}