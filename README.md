# Stringer

An easy way to turn unsafe CStrings into Rust String types and back.

## Functions

### turn_into_null_string
```rust
turn_into_null_string(st: String)  -> *const c_char
```
Takes a Rust String type and returns a CString.
# 
### make_string
```rust
make_string(s1: *const c_char) -> String
```
Takes a CString and returns a Rust String type.
# 
### into_null (ffi)
```rust
into_null(st: String)  -> *const c_char
```
is `turn_into_null_string` when using FFI as a last function call
# 
### null_to_str (ffi) 
```rust
null_to_str(s1: *const c_char) -> String
```
is `make_string` when using FFI as a last function call
# 
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
// unsafe_string is a *const c_char
make_string(unsafe_string);

let safe_string   = "some safe string".to_str();
// safe_string is a rust String type 
turn_into_null_string(safe_string);
```