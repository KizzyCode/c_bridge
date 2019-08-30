#ifndef C_BRIDGE_H
#define C_BRIDGE_H

#include <stdint.h>


/// An opaque payload
uint64_t const C_BRIDGE_TYPE_OPAQUE = 0x00;

/// A data array
uint64_t const C_BRIDGE_TYPE_DATA_ARRAY = 0x01;
/// An object array
uint64_t const C_BRIDGE_TYPE_OBJECT_ARRAY = 0x02;

/// An owned, boxed Rust object
uint64_t const C_BRIDGE_TYPE_RUST_BOX = 0x10;

/// The mask for custom types
///
/// \warning You have to ensure that you don't define the same value twice for different types. If
///          you don't need to check against the type later, use `OPAQUE`
uint64_t const C_BRIDGE_TYPE_MASK_CUSTOM = 1 << 63;


/// A FFI object
typedef struct {
	uint64_t type; ///< The payload type
	void(*dealloc)(c_bridge_ffi_object*); ///< The deallocator or `NULL` the object is unowned
	void* payload; ///< The payload or `NULL` if it is empty
} c_bridge_ffi_object;


/// A FFI result
typedef struct {
	c_bridge_ffi_object ok; ///< The ok-result
	c_bridge_ffi_object err; ///< The error or an emtpy object in case of success
} c_bridge_ffi_result;


/// A data array
typedef struct {
	uint8_t* data; ///< The data
	size_t* len; ///< The data length
} c_bridge_data_array;


/// An object array
typedef struct {
	c_bridge_ffi_object* objects; ///< The objects
	size_t* len; ///< The amount of objects
} c_bridge_object_array;


#endif //C_BRIDGE_H