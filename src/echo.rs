// echo back every byte one by one

use crate::serial::Serial;

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
