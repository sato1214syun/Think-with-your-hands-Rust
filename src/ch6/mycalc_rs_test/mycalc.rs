// #[no_mungle] と pub extern "C"の記述が必要
#[no_mangle]
pub extern "C" fn rust_mul(a: isize, b: isize) -> isize {
    a * b
}