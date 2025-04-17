use std::ffi::{CStr, CString, NulError};
use std::os::raw::c_char;
use std::str::Utf8Error;

pub fn string_to_cstring(string: String) -> Result<CString, NulError> {
    let result = CString::new(string)?;

    Ok(result)
}

pub fn cstr_to_string(unsafe_string: *const c_char) -> Result<String, Utf8Error> {
    let result = unsafe { CStr::from_ptr(unsafe_string) }
        .to_str()?
        .to_string();

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn roundtrip_ascii() {
        let original = String::from("Hello, world!");
        let cstring = string_to_cstring(original.clone()).unwrap();

        let ptr = cstring.as_ptr();
        let recovered = cstr_to_string(ptr).unwrap();

        assert_eq!(recovered, original);
    }

    #[test]
    fn roundtrip_unicode() {
        let original = String::from("こんにちは世界");
        let cstring = string_to_cstring(original.clone()).unwrap();

        let ptr = cstring.as_ptr();
        let recovered = cstr_to_string(ptr).unwrap();

        assert_eq!(recovered, original);
    }

    #[test]
    fn empty_string() {
        let original = String::new();
        let cstring = string_to_cstring(original.clone()).unwrap();

        let ptr = cstring.as_ptr();
        let recovered = cstr_to_string(ptr).unwrap();

        assert_eq!(recovered, original);
    }

    #[test]
    fn from_cstring_direct() {
        let original = "Direct CString".to_string();
        let cstring = CString::new(original.clone()).unwrap();

        let ptr = cstring.as_ptr();
        let recovered = cstr_to_string(ptr).unwrap();

        assert_eq!(recovered, original);
    }

    #[test]
    fn invalid_utf8_pointer() {
        let bytes: &[u8] = b"\xff\x00";
        let ptr = bytes.as_ptr() as *const c_char;

        assert!(cstr_to_string(ptr).is_err());
    }

    #[test]
    fn interior_null_byte() {
        let original = String::from("Hello\0World");

        assert!(string_to_cstring(original).is_err());
    }

    #[test]
    fn invalid_utf8_sequence_in_cstr_to_string() {
        let bytes: &[u8] = b"\xff\x00";
        let ptr = bytes.as_ptr() as *const c_char;

        assert!(cstr_to_string(ptr).is_err());
    }
}
