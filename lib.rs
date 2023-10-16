use wasm_bindgen::prelude::*;
use std::ffi::{c_char, c_int, c_long};
use std::ptr::{null, null_mut};

#[repr(C)]
pub struct tm {
	tm_sec: c_int,
	tm_min: c_int,
	tm_hour: c_int,
	tm_mday: c_int,
	tm_mon: c_int,
	tm_year: c_int,
	tm_wday: c_int,
	tm_yday: c_int,
	tm_isdst: c_int,
	tm_gmtoff: c_long,
	tm_zone: *mut c_char,
}

extern {
    pub fn strftime(s: *mut c_char, n: usize, f: *const c_char, tm: *const tm) -> usize;
}

#[wasm_bindgen]
pub fn test() {
    unsafe { strftime(null_mut(), 0, null(), null()) };
}