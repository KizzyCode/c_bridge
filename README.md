[![docs.rs](https://docs.rs/c_bridge/badge.svg)](https://docs.rs/c_bridge)
[![License BSD-2-Clause](https://img.shields.io/badge/License-BSD--2--Clause-blue.svg)](https://opensource.org/licenses/BSD-2-Clause)
[![License MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![crates.io](https://img.shields.io/crates/v/c_bridge.svg)](https://crates.io/crates/c_bridge)
[![Download numbers](https://img.shields.io/crates/d/c_bridge.svg)](https://crates.io/crates/c_bridge)
[![Travis CI](https://travis-ci.org/KizzyCode/c_bridge.svg?branch=master)](https://travis-ci.org/KizzyCode/c_bridge)
[![AppVeyor CI](https://ci.appveyor.com/api/projects/status/github/KizzyCode/c_bridge?svg=true)](https://ci.appveyor.com/project/KizzyCode/c-bridge)
[![dependency status](https://deps.rs/crate/c_bridge/0.1.0/status.svg)](https://deps.rs/crate/c_bridge/0.1.0)

# About `c_bridge`
This crate provides some data structures and abstractions to create clean Rust <-> C FFI interfaces

# The FFI Object
This is an in-depth overview over the FFI object and the semantics behind it:
 - [Concept](#concept)
    - [Typing](#typing)
       - [Predefined Types](#predefined-types)
          - [Data Array](#data-array)
          - [Object Array](#object-array)
          - [Rust Object](#rust-object)
    - [Ownership](#ownership)
    - [Optionality](#optionality)
    - [The FFI Result Type](#the-ffi-result-type)
       - [OK-Variant](#ok-variant)
       - [Error-Variant](#error-variant)

## Concept
Each element is passed in an FFI object struct which looks like this and provides the following
properties:
 - Layout:
   ```c
   typedef struct {
       uint64_t type;
       void(*dealloc)(ffi_object_t*);
       void* payload;
   } ffi_object_t;
   ```
 - Typing: each FFI object provides information about the payload object's type
 - Ownership: each FFI object provides information about whether the the ownership is tied to the
   FFI object or if the object is just a reference to a payload object owned by someone else
 - Optionality: each FFI object may or may not contain a payload object

### Typing
Each FFI object contains a `uint64_t` field with a type ID which specifies the type of the payload.
The ID range is split into two segments:
 - `[0x0000000000000000, 0x8000000000000000)`: A range reserved for predefined types
 - `[0x8000000000000000, 0xffffffffffffffff]`: A range reserved for implementation defined types

#### Predefined Types
 - `0x0000000000000000`: An opaque type. This type is special because it indicates that the FFI
   object deliberately carries no type information and that it's `object` must not be interpreted
   unless you *know* it's type
 - `0x0000000000000001`: A [data array](#data-array) with this layout:
   ```c
   typedef struct {
       uint8_t* data;
       size_t len;
   } data_array_t;
   ```
 - `0x0000000000000002`: An [object array](#object-array) with this layout:
   ```c
   typedef struct {
       ffi_object_t* objects;
       size_t len;
   } object_array_t;
   ```
 - `0x0000000000000010`: A [Rust box](#rust-object) containing an owned Rust object
   (`Box<dyn Any + 'static>`)

##### Data Array
The data array is a simple struct which contains a pointer to some data and a length field. The
pointer __must never__ be reallocated and must be deallocated by the FFI object's `dealloc`-function
__only__.

Layout:
```c
typedef struct {
    uint8_t* data;
    size_t len;
} data_array_t;
```

##### Object Array
The object array is a simple struct which contains a pointer to some some object structs and a
length field. The pointer __must never__ be reallocated and must be deallocated by the FFI object's
`dealloc`-function __only__.

Layout:
```c
typedef struct {
    ffi_object_t* objects;
    size_t len;
} object_array_t;
```

##### Rust Object
The Rust object is a pointer to a `Box<dyn Any + 'static>` which may typesafely contain an arbitrary
Rust object.
 
### Ownership
Since in the C world memory management happens manually, attention must be payed to questions like
"Who owns the object" and "How do I release this object", which can be especially tedious if we need
to cross FFI boundaries. To reduce this problem, we add explicit ownership information to each FFI
object using the `dealloc` field:
 - If the `dealloc` field is non-null, this means that the FFI object is owned by itself and must be
   released by passing it to it's `dealloc` function
 - If the `dealloc` field is null, the underlying `object` is managed by someone else

If the FFI object is owned, some care must be taken to avoid problems like double-free or
use-after-free:
 - Don't copy an owned FFI object
 - If you need to copy an FFI object, ensure that the shorter living one is unowned
 - If you move the payload object out of the FFI object, set the `dealloc` and `object` fields to
   null

### Optionality
Each FFI object has an optional payload object. This is necessary to be able to move something out
of the object (see [Ownership](#ownership) for more information).

Optionality is represented in the most simple way possible:
 - An existing payload object is represented as a non-null pointer to the object
 - An empty FFI object is represented by a null pointer as payload object


## The FFI Result Type
Together with the [FFI object](#the-ffi-object), we introduce another top-level type, the
`ffi_result_t`:
```c
typedef struct {
    ffi_object_t ok;
    ffi_object_t err;
} ffi_result_t;
```

This result type is similar to Rust's result and it's purpose is the same: The ability to return
either a good result or some error information without error-pointer-arguments and other
workarounds.

### OK-Variant
To construct an ok-variant, set the `ok`-field to your result and set the `err`-field to an [emtpy
FFI object](#optionality).

Note: if the `err`-field contains an empty FFI object, the FFI result must always be treated as ok -
even if the `ok`-field contains an empty FFI object (this is to be able to mimic Rust's
`Result<(), E>`).

### Error-Variant
To construct an error-variant, set the `err`-field to a [__non-emtpy__ FFI object](#optionality)
which contains your error and set the `ok`-field to an empty FFI object.