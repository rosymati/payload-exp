#![no_std]
#![no_main]

extern "C" {
    fn halt() -> !;
    fn printf(as_ptr: *const i8);
}

#[no_mangle]
pub unsafe extern "C" fn main() -> i32 {
    printf(c"Hello Everynyan\n".as_ptr());
    halt();
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
