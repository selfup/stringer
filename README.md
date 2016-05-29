# Stringer

An easy way to turn Unsafe C strings into Rust String types and back.

*There is unsafe code in this crate. Please use a nightly version of rust*

*rustc 1.9.0-nightly c9629d61c 2016-03-10) is the version I used!*

### Functions

 ```rust
 turn_into_null_string(st: String)  -> *const c_char
 ```
 Takes a regular rust String type and turns into an unsafe C String.

 ```rust
 make_string(s1: *const c_char) -> String
 ```
 Takes a C String and turns into a Rust String type.

 ```rust
 into_null(st: String)  -> *const c_char
 ```
 is `turn_into_null_string` for people using FFI

 ```rust
 null_to_str(s1: *const c_char) -> String
 ```
 is `make_string` for people using FFI

### Example on how to load this into your project:

**Cargo.toml**

```rust
[dependencies]
stringer = "*"
```

**In the file you need the functions**

 ```rust
extern crate stringer;
use stringer::*;
 ```

 **Then you can just call them as normal as long as you feed the functions the right types**

 ```rust
let unsafe_string = CString::new("unsafe string!").unwrap().into_raw();
make_string(unsafe_string);

let safe_string   = "some safe string".to_str();
turn_into_null_string(safe_string);

 ```
