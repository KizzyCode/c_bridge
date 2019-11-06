#ifndef C_BRIDGE_H
#define C_BRIDGE_H

#include <stdint.h>


/// A NULL object
typedef struct {
	uint8_t _dummy; ///< A dummy byte because empty structs are not allowed (just ignore it)
} null_t;


/// An opaque object
typedef struct {
	void (*dealloc)(opaque_t*); ///< The deallocator (must handle `NULL`)
	char const* (*type_hint)(opaque_t const*); ///< Returns a pointer to a type hint
	void* object; ///< The underlying storage object
} opaque_t;


/// A heap-allocated `uint8_t` array
typedef struct {
	void (*dealloc)(array_u8_t*); ///< The deallocator (must handle `NULL`)
	size_t (*len)(array_u8_t const*); ///< Returns the amount of bytes
	uint8_t const* (*data)(array_u8_t const*); ///< Returns a pointer to the bytes
	uint8_t* (*data_mut)(array_u8_t*); ///< Returns a mutable pointer to the bytes
	void* object; ///< The underlying storage object
} array_u8_t;

/// A heap-allocated `array_u8_t` array
typedef struct {
	void (*dealloc)(array_u8array_t**); ///< The deallocator (must handle `NULL`)
	size_t (*len)(array_u8array_t const*); ///< Returns the amount of arrays
	array_u8_t const* (*data)(array_u8array_t const*); ///< Returns a pointer to the arrays
	array_u8_t* (*data_mut)(array_u8array_t*); ///< Returns a mutable pointer to the arrays
	void* object; ///< The underlying storage object
} array_u8array_t;

/// A heap-allocated `opaque_t` array
typedef struct {
	void (*dealloc)(array_opaque_t**); ///< The deallocator (must handle `NULL`)
	size_t (*len)(array_opaque_t const*); ///< Returns the amount of objects
	opaque_t const* (*elements)(array_opaque_t const*); ///< Returns a pointer to the objects
	opaque_t* (*data_mut)(array_opaque_t*); ///< Returns a mutable pointer to the objects
	void* object; ///< The underlying storage object
} array_opaque_t;


/// A result type with `null_t` as result and `array_u8_t` as error type
typedef struct {
	void (*dealloc)(result_null_u8array_t*); ///< The deallocator (must handle `NULL`)
	null_t (*into_ok)(result_null_u8array_t*); ///< Consumes the result and returns the ok object
	array_u8_t (*into_err)(result_null_u8array_t*); ///< Consumes the object and returns the error object
	uint8_t (*is_ok)(void const*); ///< Returns `1` if the result is ok; `0` otherwise
	void* object; ///< The underlying storage object
} result_null_u8array_t;

/// A result type with `opaque_t` as result and `array_u8_t` as error type
typedef struct {
	void (*dealloc)(result_opaque_u8array_t*); ///< The deallocator (must handle `NULL`)
	opaque_t (*into_ok)(result_opaque_u8array_t*); ///< Consumes the result and returns the ok object
	array_u8_t (*into_err)(result_opaque_u8array_t*); ///< Consumes the result and returns the error object
	uint8_t (*is_ok)(void const*); ///< Returns `1` if the result is ok; `0` otherwise
	void* object; ///< The underlying storage object
} result_opaque_u8array_t;

/// A result type with `array_u8_t` as result and `array_u8_t` as error type
typedef struct {
	void (*dealloc)(result_u8array_u8array_t*); ///< The deallocator (must handle `NULL`)
	array_u8_t (*into_ok)(result_u8array_u8array_t*); ///< Consumes the result and returns the ok object
	array_u8_t (*into_err)(result_u8array_u8array_t*); ///< Consumes the result and returns the error object
	uint8_t (*is_ok)(void const*); ///< Returns `1` if the result is ok; `0` otherwise
	void* object; ///< The underlying storage object
} result_u8array_u8array_t;


#endif //C_BRIDGE_H