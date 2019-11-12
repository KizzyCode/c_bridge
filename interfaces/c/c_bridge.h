#ifndef C_BRIDGE_H
#define C_BRIDGE_H

#include <stddef.h>
#include <stdint.h>


/// A NULL object
typedef struct null_t null_t;
struct null_t {
	uint8_t _dummy; ///< A dummy byte because empty structs are not allowed (just ignore it)
};


/// An opaque object
typedef struct opaque_t opaque_t;
struct opaque_t {
	void (*dealloc)(opaque_t*); ///< The deallocator (must handle `NULL`)
	const char* (*type_hint)(const opaque_t*); ///< Returns a pointer to a type hint
	void* object; ///< The underlying storage object
};


/// A heap-allocated `uint8_t` array
typedef struct array_u8_t array_u8_t;
struct array_u8_t {
	void (*dealloc)(array_u8_t*); ///< The deallocator (must handle `NULL`)
	size_t (*len)(const array_u8_t*); ///< Returns the amount of bytes
	const uint8_t* (*data)(const array_u8_t*); ///< Returns a pointer to the bytes
	uint8_t* (*data_mut)(array_u8_t*); ///< Returns a mutable pointer to the bytes
	void* object; ///< The underlying storage object
};

/// A heap-allocated `array_u8_t` array
typedef struct array_u8array_t array_u8array_t;
struct array_u8array_t {
	void (*dealloc)(array_u8array_t*); ///< The deallocator (must handle `NULL`)
	size_t (*len)(const array_u8array_t*); ///< Returns the amount of arrays
	const array_u8_t* (*data)(const array_u8array_t*); ///< Returns a pointer to the arrays
	array_u8_t* (*data_mut)(array_u8array_t*); ///< Returns a mutable pointer to the arrays
	void* object; ///< The underlying storage object
};

/// A heap-allocated `opaque_t` array
typedef struct array_opaque_t array_opaque_t;
struct array_opaque_t {
	void (*dealloc)(array_opaque_t*); ///< The deallocator (must handle `NULL`)
	size_t (*len)(const array_opaque_t*); ///< Returns the amount of objects
	const opaque_t* (*elements)(const array_opaque_t*); ///< Returns a pointer to the objects
	opaque_t* (*data_mut)(array_opaque_t*); ///< Returns a mutable pointer to the objects
	void* object; ///< The underlying storage object
};


/// A result type with `null_t` as result and `array_u8_t` as error type
typedef struct result_null_u8array_t result_null_u8array_t;
struct result_null_u8array_t {
	void (*dealloc)(result_null_u8array_t*); ///< The deallocator (must handle `NULL`)
	null_t (*into_ok)(result_null_u8array_t*); ///< Consumes the result and returns the ok object
	array_u8_t (*into_err)(result_null_u8array_t*); ///< Consumes the object and returns the error object
	uint8_t (*is_ok)(const result_null_u8array_t*); ///< Returns `1` if the result is ok; `0` otherwise
	void* object; ///< The underlying storage object
};

/// A result type with `opaque_t` as result and `array_u8_t` as error type
typedef struct result_opaque_u8array_t result_opaque_u8array_t;
struct result_opaque_u8array_t {
	void (*dealloc)(result_opaque_u8array_t*); ///< The deallocator (must handle `NULL`)
	opaque_t (*into_ok)(result_opaque_u8array_t*); ///< Consumes the result and returns the ok object
	array_u8_t (*into_err)(result_opaque_u8array_t*); ///< Consumes the result and returns the error object
	uint8_t (*is_ok)(const result_opaque_u8array_t*); ///< Returns `1` if the result is ok; `0` otherwise
	void* object; ///< The underlying storage object
};

/// A result type with `array_u8_t` as result and `array_u8_t` as error type
typedef struct result_u8array_u8array_t result_u8array_u8array_t;
struct result_u8array_u8array_t {
	void (*dealloc)(result_u8array_u8array_t*); ///< The deallocator (must handle `NULL`)
	array_u8_t (*into_ok)(result_u8array_u8array_t*); ///< Consumes the result and returns the ok object
	array_u8_t (*into_err)(result_u8array_u8array_t*); ///< Consumes the result and returns the error object
	uint8_t (*is_ok)(const result_u8array_u8array_t*); ///< Returns `1` if the result is ok; `0` otherwise
	void* object; ///< The underlying storage object
};


#endif //C_BRIDGE_H