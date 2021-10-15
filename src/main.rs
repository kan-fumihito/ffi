extern crate libc;

use libc::{c_char, c_void, size_t};
use std::ffi::{CStr, CString};

#[link(name = "hello", kind = "static")]
extern "C" {
    fn hello();
}

fn main() {
    println!("hello rust");
    unsafe {
        hello();
    };
}
