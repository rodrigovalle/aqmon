// echo back every byte one by one

#![feature(llvm_asm)]
#![feature(const_fn)]

#![no_std]
#![no_main]

use aqmon::serial::Serial;
use core::panic::PanicInfo;

pub struct EchoServer {
    serial: Serial,
}

impl EchoServer {
    pub const fn new() -> EchoServer {
        EchoServer {
            serial: Serial::new(9600),
        }
    }

    pub fn serve(&self) {
        let buf: &[u8] = "hello?".as_bytes();
        self.serial.init();
        loop {
            //self.serial.rx(&mut buf);
            //self.serial.tx(&buf);
            self.serial.tx(buf);
            for _ in 0..100000 {
                unsafe { llvm_asm!("" :::: "volatile")}
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn main() {
    const SERVER: EchoServer = EchoServer::new();
    SERVER.serve()
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
