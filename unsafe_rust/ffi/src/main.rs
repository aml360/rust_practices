use std::os::raw::c_int;

extern "C" {
    fn my_c_function(input: *const u8) -> c_int;
}

fn main() {
    let byte_number = 5u8;
    let byte_number_ptr = &byte_number as *const u8;
    let ffi_result = unsafe { my_c_function(byte_number_ptr) };
    println!(
        "Number returned from C: {}, value of byte_number: {}",
        ffi_result, byte_number
    );
}
