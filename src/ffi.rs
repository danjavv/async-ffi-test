use std::ffi::{c_char, CStr, CString};
use std::os::raw::c_int;
use tokio::runtime::Runtime;

// FFI wrapper for say_hello_after_delay
#[no_mangle]
pub extern "C" fn say_hello_after_delay_ffi(name: *const c_char, seconds: u64) -> *mut c_char {
    println!("FFI: Starting say_hello_after_delay_ffi");
    
    if name.is_null() {
        println!("FFI: Received null pointer for name");
        return std::ptr::null_mut();
    }

    let name_str = unsafe {
        match CStr::from_ptr(name).to_str() {
            Ok(s) => {
                println!("FFI: Successfully converted name: {}", s);
                s
            },
            Err(e) => {
                println!("FFI: Failed to convert name: {:?}", e);
                return std::ptr::null_mut();
            },
        }
    };

    let rt = Runtime::new().unwrap();
    let result = rt.block_on(crate::say_hello_after_delay(name_str, seconds));
    println!("FFI: Got result from say_hello_after_delay: {}", result);
    
    match CString::new(result) {
        Ok(c_str) => {
            println!("FFI: Successfully created CString");
            c_str.into_raw()
        },
        Err(e) => {
            println!("FFI: Failed to create CString: {:?}", e);
            std::ptr::null_mut()
        },
    }
}

// FFI wrapper for calculate_fibonacci
#[no_mangle]
pub extern "C" fn calculate_fibonacci_ffi(n: u32) -> u64 {
    let rt = Runtime::new().unwrap();
    rt.block_on(crate::calculate_fibonacci(n))
}

// FFI wrapper for fetch_data
#[no_mangle]
pub extern "C" fn fetch_data_ffi(id: u32) -> *mut c_char {
    let rt = Runtime::new().unwrap();
    let result = rt.block_on(crate::fetch_data(id));
    
    match result {
        Ok(data) => match CString::new(data) {
            Ok(c_str) => c_str.into_raw(),
            Err(_) => std::ptr::null_mut(),
        },
        Err(_) => std::ptr::null_mut(),
    }
}

// Function to free strings returned by FFI functions
#[no_mangle]
pub extern "C" fn free_string(s: *mut c_char) {
    if !s.is_null() {
        unsafe {
            let _ = CString::from_raw(s);
        }
    }
} 