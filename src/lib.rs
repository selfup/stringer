use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;

pub fn turn_into_pointer(string: String)  -> CString {
    unsafe { CString::from_raw(CString::new(string).unwrap().into_raw()) }
}

pub fn make_string(unsafe_string: *const c_char) -> String {
    unsafe { CStr::from_ptr(unsafe_string) }.to_str().unwrap().to_string()
}