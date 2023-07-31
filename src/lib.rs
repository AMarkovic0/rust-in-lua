#[no_mangle]
pub extern "C" fn rust_add(a: i32, b: i32) -> i32 {
    a + b
}

#[no_mangle]
pub extern "C" fn rust_sub(a: i32, b: i32) -> i32 {
    a - b
}

#[no_mangle]
pub extern "C" fn rust_3add(a: i32, b: i32, c: i32) -> i32 {
    rust_add(add(a, b), c)
}

pub fn add(left: i32, right: i32) -> i32 {
    left + right
}
