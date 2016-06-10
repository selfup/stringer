# Stringer

An easy way to turn an unsafe `*const c_char` into a Rust String type and return a pointer for FFI.

## Functions

### turn_into_pointer
```rust
turn_into_pointer(st: String)  -> CString
```
Takes a Rust String type and returns a CString.
#
### make_string
```rust
make_string(s1: *const c_char) -> String
```
Takes a `*const c_char` and returns a Rust `String` type.
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
