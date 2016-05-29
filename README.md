# Stringer

An easy way to turn Unsafe C strings into Rust String types and back.

 * `turn_into_null_string(st: String)  -> *const c_char` Takes a regular rust String type and turns into an unsafe C String.
 * `make_string(s1: *const c_char) -> String` Takes a C String and turns into a Rust String type.

 * `into_null(st: String)  -> *const c_char` is `turn_into_null_string` for people using FFI
 * `null_to_str(s1: *const c_char) -> String` is `make_string` for people using FFI
