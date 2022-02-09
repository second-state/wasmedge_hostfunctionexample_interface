//! How to use this crate
//! # Adding this as a dependency
//! ```rust, ignore
//! [dependencies]
//! wasmedge_hostfunctionexample_interface = "^1.0.0"
//! ```
//!
//! # Bringing this into scope
//! ```rust, ignore
//! use wasmedge_hostfunctionexample_interface::*;
//! ```

use std::ffi::CString;

pub mod wasmedge_hostfunctionexample {
    use std::os::raw::c_char;
    #[link(wasm_import_module = "host_function_example")]
    extern "C" {
        pub fn host_function_example_set_class_id(cid: u32);
        pub fn host_function_example_add_student(student: *const c_char, len: u32) -> u32;
        pub fn host_function_example_set_class_name(name: *const c_char, len: u32);
        pub fn host_function_example_print();
    }
}

pub fn set_class_id(cid: u32) {
    unsafe {
        wasmedge_hostfunctionexample::host_function_example_set_class_id(cid);
    }
}

pub fn set_class_name<S: AsRef<str>>(name: S) {
    let name = CString::new((name.as_ref()).as_bytes()).expect("");
    unsafe {
        wasmedge_hostfunctionexample::host_function_example_set_class_name(
            name.as_ptr(),
            name.as_bytes().len() as u32,
            );
    }
}

pub fn add_student<S: AsRef<str>>(name: S) -> u32 {
    let name = CString::new((name.as_ref()).as_bytes()).expect("");
    let student_size: u32;
    unsafe {
        student_size = wasmedge_hostfunctionexample::host_function_example_add_student(
            name.as_ptr(),
            name.as_bytes().len() as u32,
            );
    }
    student_size
}

pub fn print() {
    unsafe {
        wasmedge_hostfunctionexample::host_function_example_print();
    }
}
