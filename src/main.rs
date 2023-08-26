#![no_std]
#![no_main]

use core::panic::PanicInfo;

static TEXT: &[u8] = b"There is nothing to see here";

#[no_mangle]
pub extern "C" fn _start() -> ! { //enter
    let vga_buff = 0xb8000 as *mut u8;
    let mut colour: u8 = 0x1;
    
    for (i, &byte) in TEXT.iter().enumerate() {
        unsafe {
            *vga_buff.offset(i as isize * 2) = byte;
            *vga_buff.offset(i as isize * 2 + 1) = colour;
            colour += 1;
            if colour > 0xf{
                colour = 0x1;
            }
        }
    }
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! { //TODO: Make this do something
    loop {}
}
