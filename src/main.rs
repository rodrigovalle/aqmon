#![feature(llvm_asm)]
#![feature(const_fn)]

#![no_std]
#![no_main]

use aqmon::serial::Serial;
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn main() {
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// These symbols were missing from compiler_builtins at the time of writing so
// I implemented my own.
//
// They're necessary because AVR has an 8 bit architecture and LLVM cannot rely
// on native instructions to do left and right shifts; instead LLVM will
// call into these symbols to do those operations.

// arithmetic shift left
// We can implement support for left shifts on 32 bit types using 16 bit types
// (which in turn have their own implementation using AVR's 8 bit registers)
// #[no_mangle]
// pub extern "C" fn __ashlsi3(a: u32, offset: u32) -> u32 {
//     // this _should_ be implemented as two 8 bit register lookups on avr
//     let low = a as u16;
//     let high = (a >> 16) as u16;
// 
//     if offset == 0 {
//         // easy case, we don't shift
//         a
//     } else if offset >= 16 {  // else if offset & 16 != 0 {
//         // we're shifting left by more than half the bits
//         // low end gets filled with zero, high end is the old low end
//         let ret_low = 0;
//         let ret_high = low << (offset - 16);
//         (ret_low as u32) | ((ret_high as u32) << 16)
//     } else {
//         let ret_low = low << offset;  // shift in zeros on the low side
//         let ret_high = (high << offset) | (low >> (16 - offset));
//         (ret_low as u32) | ((ret_high as u32) << 16)
//     }
// }

// logical shift right (always fills with zeros)
// #[no_mangle]
// pub extern "C" fn __lshrsi3(a: u32, offset: u32) -> u32 {
//     let low = a as u16;
//     let high = (a >> 16) as u16;
// 
//     if offset == 0 {
//         a
//     } else if offset >= 16 {
//         // the high side is all zeroes and the low side is now the high side
//         let ret_low = high >> (offset - 16);
//         let ret_high = 0;
//         (ret_low as u32) | ((ret_high as u32) << 16)
//     } else {
//         let ret_low = (low >> offset) | (high << (16 - offset));
//         let ret_high = high >> offset;
//         (ret_low as u32) | ((ret_high as u32) << 16)
//     }
// }
