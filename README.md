# Stringer

An easy way to turn an unsafe `*const c_char` into a Rust String type and return a pointer for FFI.

## Functions

### string_to_cstring
```rust
string_to_cstring(string: String) -> Result<CString, NulError>
```
Takes a `String` and returns a `CString`.
#
### cstr_to_string
```rust
cstr_to_string(unsafe_string: *const c_char) -> Result<String, Utf8Error>
```
Takes a `*const c_char` and returns a `String`.
#
### Example on how to load this into your project:

**Cargo.toml**

```rust
[dependencies]
stringer = "0.2.0"
```

**In the file you need the functions**

```rust
extern crate stringer;
use stringer::*;
```
### Things to consider

Due to the nature of what this library is doing:

* This code is unsafe
* Use at your own risk
