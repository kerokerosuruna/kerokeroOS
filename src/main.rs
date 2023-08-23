#![no_std]
#![no_main]

//cargo rustc -- -C link-args="/ENTRY:_start /SUBSYSTEM:console"

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! { //enter
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! { //TODO: Make this do something
    loop {}
}