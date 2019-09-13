#ifndef C_BRIDGE_H
#define C_BRIDGE_H

#include <stdint.h>


/// A heap-allocated `uint8_t` array
typedef struct {
	void (*dealloc)(void**); ///< The deallocator (must handle `NULL` pointers)
	size_t (*len)(void const*); ///< Returns the amount of bytes
	uint8_t const* (*data)(void const*); ///< Returns a pointer to the underlying bytes
	uint8_t* (*data_mut)(void*); ///< Returns a mutable pointer to the underlying bytes
	void* object; ///< The underlying storage object
} array_u8_t;


/// A heap-allocated `array_u8_t` array
typedef struct {
	void (*dealloc)(void**); ///< The deallocator (must handle `NULL` pointers)
	size_t (*len)(void const*); ///< Returns the amount of arrays
	array_u8_t const* (*data)(void const*); ///< Returns a pointer to the underlying arrays
	array_u8_t* (*data_mut)(void*); ///< Returns a mutable pointer to the underlying arrays
	void* object; ///< The underlying storage object
} array_u8array_t;


/// A NULL object
typedef struct {
	uint8_t _dummy; ///< A dummy byte because empty structs are not allowed; just ignore it
} null_t;


/// An opaque object
typedef struct {
	void (*dealloc)(void**); ///< The deallocator (must handle `NULL` pointers)
	char const* (*type_hint)(); ///< Returns a pointer to a type hint
	void* object; ///< The underlying storage object
} opaque_t;


/// A result type with `array_u8_t` as result and `char const*` as error type
typedef struct {
	void (*dealloc)(void**); ///< The deallocator (must handle `NULL` pointers)
	array_u8_t (*into_ok)(void**); ///< Consumes the object and returns the underlying result
	char const* (*into_err)(void**); ///< Consumes the object and returns the underlying error
	uint8_t (*is_ok)(void const*); ///< Returns `1` if the result is ok; `0` otherwise
	void* object; ///< The underlying storage object
} result_u8array_constchar_t;


#endif //C_BRIDGE_H