#![feature(llvm_asm)]

#![no_std]
#![no_main]

use core::panic::PanicInfo;


#[no_mangle]
pub extern fn main() {
    delay();
}

fn delay() {
    for _ in 0..400000 {
        unsafe { llvm_asm!("" :::: "volatile") }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    //println!("{}", info);
    loop {}
}
