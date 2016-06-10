use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;

pub fn turn_into_pointer(st: String)  -> CString {
    let concated_string = CString::new(st).unwrap();
    unsafe { CString::from_raw(concated_string.into_raw()) }
}

pub fn make_string(s1: *const c_char) -> String {
    let cstr          = unsafe { CStr::from_ptr(s1) }; // &std::ffi::c_str::CStr
    let string        = cstr.to_str().unwrap();        // &str
    let return_string = string.to_string();            // collections::string::String
    return_string
}
