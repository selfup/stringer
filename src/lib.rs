use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;

pub fn turn_into_null_string(st: String)  -> *const c_char {
    let concated_string = CString::new(st).unwrap();
    concated_string.into_raw()
}

pub fn make_string(s1: *const c_char) -> String {
    let cstr          = unsafe { CStr::from_ptr(s1) };  // &std::ffi::c_str::CStr
    let string        = cstr.to_str().unwrap();  // &str
    let return_string = string.to_string();  // collections::string::String
    return_string
}

#[test]
fn it_reads_and_modifies_both_cstr_and_rust_str() {
    let static_to_string = "string to compare".to_string();
    let null_string      = turn_into_null_string(static_to_string);

    let nul_test_str     = make_string(null_string);
    let test_string      = "string to compare".to_string();

    assert_eq!(test_string, nul_test_str);
}

//
// THESE EXTERN FUNCTIONS ARE FOR PEOPLE WHO ARE USING FFI
//

pub extern fn into_null(st: String)  -> *const c_char {
    let concated_string = CString::new(st).unwrap();
    concated_string.into_raw()
}

pub extern fn null_to_str(s1: *const c_char) -> String {
    let cstr          = unsafe { CStr::from_ptr(s1) };
    let string        = cstr.to_str().unwrap();
    let return_string = string.to_string();
    return_string
}
