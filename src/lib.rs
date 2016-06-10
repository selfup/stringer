use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;

pub fn turn_into_pointer(st: String)  -> CString {
    unsafe { CString::from_raw(CString::new(st).unwrap().into_raw()) }
}

pub fn make_string(s1: *const c_char) -> String {
    unsafe { CStr::from_ptr(s1) }.to_str().unwrap().to_string()
}
